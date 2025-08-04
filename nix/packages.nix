{
  argBuildType,
  pkgs,
}:
pkgs.rustPlatform.buildRustPackage {
  name = "git-cli";
  src = pkgs.lib.cleanSource ../.;

  buildType = argBuildType;
  cargoLock.lockFile = ../Cargo.lock;
  cargoBuildFlags = "";
}
