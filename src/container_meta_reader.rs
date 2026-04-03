use std::{collections::HashMap, path::Path};

use containerd_client::{
    connect,
    services::v1::{containers_client::ContainersClient, GetContainerRequest},
    tonic::transport::Channel,
    tonic::Request,
    with_namespace,
};

use crate::{
    errors::InitError,
    pids_item,
    read::{AllProcInfo, Value},
    ContainerMetaReader,
};

#[derive(Debug)]
pub struct ContainerMeta {
    name: String,
    namespace: String,
}

pub struct ContainerdReader {
    client: ContainersClient<Channel>,
}

impl ContainerMetaReader for ContainerdReader {
    async fn new(runtime_endpoint: String) -> Result<Self, InitError>
    where
        Self: Sized,
    {
        let channel = match connect(runtime_endpoint).await {
            Ok(c) => c,
            Err(err) => return Err(InitError::ContainerClientInit(err)),
        };

        let client = ContainersClient::new(channel);

        Ok(Self { client })
    }

    async fn proc_to_container(
        mut self,
        infos: AllProcInfo,
    ) -> Result<HashMap<i32, ContainerMeta>, crate::errors::ReadError> {
        let Some(cgroup_index) = infos
            .items
            .iter()
            .position(|x| *x == pids_item::PIDS_CGROUP_V)
        else {
            return Err(crate::errors::ReadError::MissingItem(
                pids_item::PIDS_CGROUP_V,
            ));
        };
        let mut output: HashMap<i32, ContainerMeta> = HashMap::new();
        for (pid, info) in infos.procs {
            // I don't care for this multiline match
            let cgroup = match info
                .stat
                .get(cgroup_index)
                .expect("all processes to have a cgroup")
            {
                Value::Str(c) => c.to_string(),
                _ => unreachable!(),
            };
            // non containerized process
            if !cgroup.contains("kubelet-kubepods.slice") {
                continue;
            }
            let cgroup_path = Path::new(&cgroup);
            // we have to trim the resulting path
            // kubelet.slice/kubelet-kubepods.slice/kubelet-kubepods-burstable.slice/kubelet-kubepods-burstable-pod70c196014b615996a2893e59ef9bd41d.slice/cri-containerd-8f92d57269cccc8617178621006662f45c086e1251ba6ce5508b7f6165d8a2a1.scope
            // we want the last part but without the leading cri-containerd- and the trailing .scope
            let container_id = cgroup_path
                .iter()
                .find(|x| x.to_str().unwrap().starts_with("cri-containerd"))
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("cri-containerd-")
                .trim_end_matches(".scope");
            let request = GetContainerRequest {
                id: container_id.to_string(),
            };
            let response = self
                .client
                .get(with_namespace!(request, "k8s.io"))
                .await
                .unwrap();
            let labels = response.into_inner().container.unwrap().labels;
            let pod_name = labels.get("io.kubernetes.pod.name").unwrap();
            let pod_namespace = labels.get("io.kubernetes.pod.namespace").unwrap();

            output.insert(
                pid,
                ContainerMeta {
                    name: pod_name.to_string(),
                    namespace: pod_namespace.to_string(),
                },
            );
        }

        Ok(output)
    }
}
