# Local Development

## Docker

This method doesn't work well for interactive development, but if you want to
run a build or tests then it's good enough.

1. `docker build . -f Dockerfile.dev -t ctop-dev`
2. `docker run -it ctop-dev`

The entry point of the Dockerfile is bash. From there you can run cargo commands
such as "build", "test", etc

## Linux

### Nix users

The nix flake within the project root contains the rust tool chain and run
`nix develop`. This will give you a bash shell with which you can build the
project.

### Ubuntu (probably Debian too)

Install the following packages via apt:

- llvm
- clang
- clang-tools
- rustup
- pkg-config
- libproc2-dev

### Running ctop against a local kubernetes cluster

This section assumes your environment uses cgroupV2 and systemd.

Install [kind] which creates a Kubernetes cluster within a docker container.

The cgroup hierarchy will be under the docker container for the kind control
plane. Using `kind create cluster` (default options), you can list all the
cgroups via
`docker ps -f Name=kind-control-plane --format json|jq '.ID' | xargs -I{} docker inspect {}| jq '.[0].Id' | xargs -I{} systemd-cgls /system.slice/docker-{}.scope`.

The command will filter your running containers searching for a container named
"kind-control-plane" and getting the long container ID. The container ID will be
used within the `systemd-cgls` command to navigate the tree. By default, on most
docker installations containers will be within the `/system.slice`.

#### Example output

```shell
CGroup /system.slice/docker-19389c03ce644c66b0d9276214d8529ebf3c95110caefd3bc4a5c38ebb0c449e.scope:
├─init.scope
│ └─19683 /sbin/init
├─system.slice
│ ├─containerd.service …
│ │ ├─19914 /usr/local/bin/containerd
│ │ ├─20353 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 268426b40c29d3a00802fab58130e93122a18652fd55d905acf588ad7176385d -address /run/containerd/containerd.sock
│ │ ├─20372 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 3641690a6b910b5371889b06323409be3f79af400264bf9f17307ee63474913d -address /run/containerd/containerd.sock
│ │ ├─20411 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id cd346de1702f12e5d5de43608af89426a2c8638a5879a191cf542aa5a4025645 -address /run/containerd/containerd.sock
│ │ ├─20428 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 76cdbdc108721b9e7276505fe4ba9b2b3e08c5603ca40672c27cdc704c19dbd4 -address /run/containerd/containerd.sock
│ │ ├─21535 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id fc3a9c4919dea08c3f7f96f9d65a3b8e3fcf5b74627b5cb55cded6c52d906a85 -address /run/containerd/containerd.sock
│ │ ├─21565 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 655e16e3cb5b8531036d89579e1c242236993f9e7e4e8b0b4bb7a3fc1c1c5950 -address /run/containerd/containerd.sock
│ │ ├─22039 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 2d45db7ca88ed05be86fa572e2066a1eb39c1992868aecacc3a5f71c4607b71b -address /run/containerd/containerd.sock
│ │ ├─22073 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 12d470996d6b32b927f23233d4ec843b9960301ce81d8f4ed481d1939328b398 -address /run/containerd/containerd.sock
│ │ └─22082 /usr/local/bin/containerd-shim-runc-v2 -namespace k8s.io -id 84eee9ab6dec40e93e4a3414d13433802f792a45c3bdf4b35971e8f7a392c527 -address /run/containerd/containerd.sock
│ └─systemd-journald.service
│   └─19898 /lib/systemd/systemd-journald
└─kubelet.slice
  ├─kubelet.service
  │ └─20880 /usr/bin/kubelet --bootstrap-kubeconfig=/etc/kubernetes/bootstrap-kubelet.conf --kubeconfig=/etc/kubernetes/kubelet.conf --config=/var/lib/kubelet/config.yaml --node-ip=172.18.0.2 --node-labels= --p>
  └─kubelet-kubepods.slice
    ├─kubelet-kubepods-pod2b698bd8_b22c_4173_bf9d_900c652aab26.slice
    │ ├─cri-containerd-b8957594098e7ac63007b1987f731f3e42fd6565b0c5e7caaef2ad894d675993.scope …
    │ │ └─21745 /bin/kindnetd
    │ └─cri-containerd-fc3a9c4919dea08c3f7f96f9d65a3b8e3fcf5b74627b5cb55cded6c52d906a85.scope …
    │   └─21589 /pause
    ├─kubelet-kubepods-besteffort.slice
    │ ├─kubelet-kubepods-besteffort-pod7e915e12_ded7_4b8e_84cb_55dd063ce065.slice
...
```

## macOS

This project uses libproc2 which does not compile on macOS (due to macOS lacking
`/proc` pseudo-filesystem). Thus, you will need to use a Linux virtual machine.

I recommend [UTM] since it was less fussy than
[lima](https://github.com/lima-vm/lima), and you can run graphical applications.

1. Install [UTM]
2. Follow the guide to install Ubuntu, [install guide]
   - This takes you through the Ubuntu server install
   - Use the same architecture as your machine (Intel or Arm)
   - You could also bring your own Linux qcow image
3. Install these packages for sharing files from your host to your guest
   - bindfs
   - Required to resolve permission errors, [permissions guide]
4. Install packages from [here](#ubuntu-probably-debian-too)

[UTM]: https://docs.getutm.app/installation/macos/
[install guide]: https://docs.getutm.app/guides/ubuntu/
[permissions guide]: https://docs.getutm.app/guest-support/linux/#virtfs
[kind]: https://kind.sigs.k8s.io/
