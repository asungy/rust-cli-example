{
  inputs = {
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    nixpkgs-stable.url = "github:nixos/nixpkgs/release-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [(import inputs.rust-overlay)];
        pkgs-stable = import inputs.nixpkgs-stable { inherit system overlays; };
        pkgs-unstable = import inputs.nixpkgs-unstable { inherit system overlays; };
      in
      {
        devShells.default = pkgs-unstable.callPackage ./nix/devShell.nix {};
        packages =
        let
          mkArgs = argBuildType: {
            inherit argBuildType;
          };
        in rec
        {
          git-cli-debug = pkgs-unstable.callPackage ./nix/packages.nix (mkArgs "debug");
          git-cli-release = pkgs-unstable.callPackage ./nix/packages.nix (mkArgs "release");
          git-cli = git-cli-release;
          default = git-cli;
        };
      }
    );
}
