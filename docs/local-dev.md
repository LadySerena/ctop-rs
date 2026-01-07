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
