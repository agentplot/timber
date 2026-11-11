# Tasks

Every command runs from the place's root, as written. Build the groups
in order: group 3's checks read the scripts group 1 writes and the
version line group 2 writes. Submit nothing: Flume submits the place
once its last stage delivers.

## 1. timber's tree names its version, and set-version.sh writes it

- [x] 1.1 Main's tree gives `0.0.0` in every file that carries timber's version: the two plugin manifests move from `0.1.0`, and `Cargo.toml`'s comment says how a release writes its version. `set-version.sh <x.y.z> [<root>]` writes the version into each file, changing no other line. It refuses a version that is not `x.y.z` and a file with no version, naming it, and `--files` lists the four files. `check-versions.sh` holds them as the root's `versions` task (release-version spec, Main's tree names no release and set-version.sh; design, Decisions 1 and 2; Task notes 1.1). Verify with `devenv shell -- moon run root:versions`.

## 2. The binary says its version

- [x] 2.1 `timber --version` prints `timber <version>, revision <revision>, built by <build>`, with the version cargo's from the tree. The unit test that held a revision and no version goes. `build.rs`'s and `lib.rs`'s comments say the binary names its version beside its revision and its build (command-line spec; design, Decision 5; Task notes 2.1). Verify with `devenv shell -- cargo test -p timber --test cli the_binary_says_its_version_its_revision_and_its_build`.

## 3. A release's tag names its release commit

- [x] 3.1 `release-commit.sh <x.y.z>` refuses a tree that differs from HEAD, runs `set-version.sh` and commits its files by path as `chore(release): v<x.y.z>`, by the bot, dated at HEAD's commit time, so two clones of one commit make one id. `check-release.sh` proves it in scratch repositories as the root's `release` task (release-version spec, release-commit.sh makes one release commit wherever it runs; design, Decision 3; Task notes 3.1). Verify with `devenv shell -- moon run root:release`.
- [ ] 3.2 The flake versions its package by `Cargo.toml`'s `[workspace.package]` alone, with no `getEnv` and no `TIMBER_VERSION`, and its header says so. `check-release.sh` holds that (release-version spec, A build takes its version from its tree alone; design, Decision 4; Task notes 3.2). Verify with `devenv shell -- moon run root:release`.
- [ ] 3.3 CI releases from the release commit. The version job reckons with `--unreleased`, makes the commit and hands on its id. The binaries job makes it again, checks the id and the package's version, builds it with `nix build .#default` given nothing, and checks `timber --version`. The release job writes the notes, makes and checks the commit, tags it, pushes the tag alone and publishes with `--verify-tag`. The build job checks that main's package and binary say `0.0.0` and the binary its commit, and the conventional job starts at the newest tag by version. `cliff.toml`'s header and the workflow's comments say where the version lives, and `check-release.sh` reads each job (release-version spec, the tag, binaries and reckoning requirements; design, Decision 6; Task notes 3.3). Verify with `devenv shell -- moon run root:release`.
- [ ] 3.4 Record 0031 records the decision, accepted on the brief, with the reading of 24 and C.11 and the paths not taken. Record 0011's status says its half on what the binary says is superseded by 0031 (design, Decision 7; Task notes 3.4). Verify with `devenv shell -- moon run root:adrs`.
- [ ] 3.5 Rule core.74, under `### The release` in `docs/architecture/core.md`, with its details. README says how a release names its version, with rows for `set-version.sh` and `release-commit.sh` (design, Decision 7; Task notes 3.5). Verify that `grep -c 'core\.74' docs/architecture/core.md` prints `2` and `grep -c 'release-commit.sh' README.md` prints at least `1`.
