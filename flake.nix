# timber as a nix package, for `nix profile install github:boomdrivedev/timber`
# and for the GHA build. The development shell is devenv's (devenv.nix);
# this flake only builds and installs. Rust is the version .prototools
# names, so the release is built by the compiler the checks ran with. The
# package's version is the tree's, Cargo.toml's [workspace.package], and
# nothing else: 0.0.0 on main, and the release's at a release's tag,
# which names the release commit that wrote it (set-version.sh,
# release-commit.sh; record 0031). The binary says that version beside
# the revision it was built from, which this gives it, and the build that
# made it.
{
  description = "timber: build once, sign everything, promote with proof, in your own account";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
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
    in
    {
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
    };
}
