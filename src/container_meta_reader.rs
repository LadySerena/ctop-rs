use std::path::Path;

use containerd_client::{
    connect,
    services::v1::{containers_client::ContainersClient, GetContainerRequest},
    tonic::transport::Channel,
    tonic::Request,
    with_namespace,
};

use crate::{errors::InitError, pids_item, read::ProcessInfo, ContainerMetaReader};

#[derive(Debug)]
pub struct ContainerMeta {
    process_id: i32,
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
        infos: ProcessInfo,
    ) -> Result<Vec<ContainerMeta>, crate::errors::ReadError> {
        let Some(pid_index) = infos
            .items
            .iter()
            .position(|x| *x == pids_item::PIDS_ID_PID)
        else {
            return Err(crate::errors::ReadError::MissingItem(
                pids_item::PIDS_ID_PID,
            ));
        };
        let Some(cgroup_index) = infos
            .items
            .iter()
            .position(|x| *x == pids_item::PIDS_CGROUP_V)
        else {
            return Err(crate::errors::ReadError::MissingItem(
                pids_item::PIDS_CGROUP_V,
            ));
        };
        let mut output: Vec<ContainerMeta> = Vec::new();
        for info in infos.procs {
            let pid = match info.get(pid_index).expect("all processes to have pid") {
                crate::read::Value::Int32(p) => *p,
                _ => unreachable!(),
            };
            let cgroup = match info
                .get(cgroup_index)
                .expect("all processes to have a cgroup")
            {
                crate::read::Value::Str(c) => c.to_string(),
                _ => unreachable!(),
            };
            if !cgroup.contains("kubelet-kubepods.slice") {
                continue;
            }
            let cgroup_path = Path::new(&cgroup);
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

            output.push(ContainerMeta {
                process_id: pid,
                name: pod_name.to_string(),
                namespace: pod_namespace.to_string(),
            });
        }

        Ok(output)
    }
}
