# timber as a nix package, for `nix profile install github:boomdrivedev/timber`
# and for the GHA build, and services.timber, the nix-darwin and NixOS
# modules that write the host's bindings file and install the package
# (record 0035, core.76). The development shell is devenv's (devenv.nix);
# everything else of the flake's is under nix/, the moon project `nix`.
# Rust is the version .prototools names, so the release is built by the
# compiler the checks ran with. The package's version is the tree's,
# Cargo.toml's [workspace.package], and nothing else: 0.0.0 on main, and
# the release's at a release's tag, which names the release commit that
# wrote it (set-version.sh, release-commit.sh; record 0031). The binary
# says that version beside the revision it was built from, which this
# gives it, and the build that made it.
{
  description = "timber: build once, sign everything, promote with proof, in your own account";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The tests' nix-darwin; a host's configuration brings its own.
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      # nixpkgs refuses x86_64-darwin since 26.11, and no release builds
      # it (record 0035).
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAll =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlays.default ];
            }
          )
        );
      rustVersion = (builtins.fromTOML (builtins.readFile ./.prototools)).rust;
      workspace = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package;
      revision = self.rev or self.dirtyRev or "unknown";
      packages = forAll (
        pkgs:
        let
          rust = pkgs.rust-bin.stable.${rustVersion}.minimal;
          platform = pkgs.makeRustPlatform {
            cargo = rust;
            rustc = rust;
          };
        in
        {
          default = platform.buildRustPackage {
            pname = "timber";
            inherit (workspace) version;
            env.TIMBER_REVISION = revision;
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "-p"
              "timber"
            ];
            # The tests run in the devenv shell, through moon, by layer.
            doCheck = false;
            meta = {
              description = "Build once, sign everything, promote with proof, in your own account";
              homepage = "https://github.com/boomdrivedev/timber";
              license = pkgs.lib.licenses.mit;
              mainProgram = "timber";
            };
          };
        }
      );

      # A module of a platform, keyed as sluice's and Flume's flakes key
      # their own, so a configuration that imports it twice, directly and
      # through Boomdrive's flake, declares services.timber once (record
      # 0035).
      moduleOf = platform: file: {
        _file = file;
        key = "github:boomdrivedev/timber#${platform}Modules.timber";
        imports = [ (import file { inherit packages; }) ];
      };
    in
    {
      inherit packages;

      # services.timber, the host's bindings file and the package (record
      # 0035, core.76).
      darwinModules = rec {
        timber = moduleOf "darwin" ./nix/modules/darwin.nix;
        default = timber;
      };
      nixosModules = rec {
        timber = moduleOf "nixos" ./nix/modules/nixos.nix;
        default = timber;
      };
    };
}
