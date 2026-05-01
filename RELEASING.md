# Cutting new releases

1. verify you're on main and the git ref you want to be on
2. verify version and revisions in cargo.toml
  * if changes needed change the version, commit to main, cut tag
3. run ./nix-build-deb
  * requires having qemu packages installed [buildx entry]

[buildx entry]: https://wiki.archlinux.org/title/Docker#Using_buildx_for_cross-compiling
