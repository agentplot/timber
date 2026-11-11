# A timber release reports its own version

## Why

A build of a timber release's tag does not know the tag's version. CI
tags the commit of main it releases, whose tree says `0.0.0`: at v0.5.3,
`Cargo.toml`'s `[workspace.package]` gives `version = "0.0.0"` and the
tag names `chore(land): #93 …`, a landing commit. Only CI's release
build is told the version, from its environment:
`TIMBER_VERSION="${TAG#v}" nix build --impure`
(`.github/workflows/ci.yml`'s `binaries` job), which the flake reads with
`builtins.getEnv "TIMBER_VERSION"` for the package's name alone. The
binary says no version at all: `timber --version` prints the revision
and the build (record 0011).

So any other builder of the tag, the Boomdrive set among them, gets a
package `timber-0.0.0` whose binary cannot say which release it is,
unless the builder names the version itself. agentplot-ops found the
same defect in sluice on 2026-11-11, where a sluice built at v0.88.0
reported 0.78.1 (`~/.local/state/crew/agentplot-main/reports/deneb-boomdrive-572919e.nix`).
agentplot/blueprints `design/principles.md` 14 says a tool says its own:
"A tool that needs what another program knows reads that program's own
source, never a file a host makes for it."

The family releases one way. Flume's tag v0.21.2 names
`chore(release): v0.21.2`, whose `Cargo.toml` says 0.21.2
(`set-version.sh`). crew's `code.10` does the same (crew bolt
`a-crew-release-reports-its-own-version`, #73 at 81a883f3), and sluice's
change `a-sluice-release-reports-its-own-version` brings sluice to it.
timber takes the same form.

## What Changes

- **timber's tree names its version.** On main, every file that
  carries timber's version gives `0.0.0`: `Cargo.toml`'s
  `[workspace.package]`, which each of the five crates takes,
  `Cargo.lock`'s entries for those crates, the Claude Code plugin's
  manifest (`plugins/claude-code/.claude-plugin/plugin.json`) and the
  herdr plugin's (`plugins/herdr/herdr-plugin.toml`). The two manifests
  move from `0.1.0`, a version no release had.
- **`set-version.sh` writes a release's version.** A new
  `sh set-version.sh <x.y.z> [<root>]` at the root writes the version
  into each of those files and changes no other line. It refuses a
  version that is not `x.y.z`, and a file in which it finds no version,
  naming the file. `sh set-version.sh --files` lists the files.
  `root:versions` holds main's files at `0.0.0` and runs the writer on
  a scratch copy.
- **`release-commit.sh` makes one release commit wherever it runs.** A
  new `sh release-commit.sh <x.y.z>` runs `set-version.sh` on the
  commit checked out and commits its files by path as
  `chore(release): v<x.y.z>`. The commit is made by
  `github-actions[bot]`, dated as the checked-out commit was committed,
  so every job that runs it on one pushed commit makes the same commit.
  `root:release` proves it in scratch repositories.
- **A release's tag names that release commit.** CI's version job finds
  the next version, makes the release commit and hands on its id. The
  binaries job makes the same commit and builds it with
  `nix build .#default`, with no variable, no `--impure` and no
  argument. It fails unless `timber --version` names the release's
  version and that commit. The release job makes the same commit, tags
  it, pushes the tag alone and publishes the GitHub release with
  `--verify-tag`. Nothing is committed to main, no branch is pushed and
  no tag is moved.
- **A build takes its version from its tree alone.** The flake's
  package version is `Cargo.toml`'s, with no `getEnv`. `TIMBER_VERSION`
  leaves the flake and CI.
- **The binary says its version.** `timber --version` prints
  `timber <version>, revision <revision>, built by <build>`, the version
  being cargo's from the tree. It no longer prints the revision and
  build with no version.
- **CI reckons from the newest tag by version.** A release's tag is now
  off main's history. The version job and the notes run git-cliff with
  `--unreleased`, and the conventional check starts at the newest `v*`
  tag by version, not at `git describe`'s.
- The decision is recorded as record 0031, which supersedes record
  0011's half on what the binary says, and as rule core.74. README says
  how a release names its version.

## Capabilities

### New Capabilities

- `release-version`: how timber is released so that a build of a
  release's tag reports that release's version. Main's tree names
  `0.0.0`, `set-version.sh` writes a release's version into every file
  that carries timber's, and `release-commit.sh` makes one release
  commit of it. A release's tag names that commit and its binaries are
  built from it with nothing given. A build reads the version from the
  tree alone, and CI reckons the next release from the newest tag by
  version.

### Modified Capabilities

- `command-line`: `timber --version` names the binary's version beside
  its revision and its build.

## Impact

- New: `set-version.sh`, `release-commit.sh`, `check-versions.sh`,
  `check-release.sh`, `docs/adr/0031-…md`, and the root project's
  `versions` and `release` tasks in `moon.yml`.
- `plugins/claude-code/.claude-plugin/plugin.json`,
  `plugins/herdr/herdr-plugin.toml`: `0.0.0`. `Cargo.toml`: its comment.
- `crates/timber/src/lib.rs` (`VERSION_LINE`, its doc comments, the old
  unit test removed), `crates/timber/build.rs` (its header comment),
  `crates/timber/tests/cli.rs` (one test).
- `flake.nix`, `.github/workflows/ci.yml` (its header and the `build`,
  `conventional`, `version`, `binaries` and `release` jobs),
  `cliff.toml` (its header comment).
- `docs/architecture/core.md` (core.74), `docs/adr/0011-…md` (its status
  line), `README.md`.
- Unchanged: everything timber does for a repository it serves. Clause
  24 and C.11 still bind the builds a publish offers: those builds never
  know the version they become, and a compiled binary a projection packs
  carries the revision and the build identity, never the version. A
  build of main, or of any commit no tag names, reports `0.0.0`.

### Out of scope

- **Boomdrive's set.** It packages no timber yet. Once it does, it takes
  timber's own package at a tag made this way, and names no version of
  its own (agentplot/boomdrive record 0023). That is agentplot/boomdrive's,
  after this change's first release.
- **Reading the plugins at a release.** The Claude Code marketplace and
  the herdr plugin are read from main, where the manifests now say
  `0.0.0` as they said `0.1.0` before. A `release` branch for them, as
  Flume's, is a change of its own if one is wanted.
- **Installing a release by its tag.** The roadmap's install command
  names the flake at main. Naming a tag there is a change of its own.
- **timber publishing itself.** When phase 1 has timber publish timber,
  the artifact it offers is a build of a landed revision of main, which
  says `0.0.0` as 24 and C.11 have it. How that sits beside the GitHub
  release's binary is that unit's design.
- **Tags made before this change.** v0.1.0 to v0.5.3 name main's own
  commits, whose trees say `0.0.0`, and stay as they are (core.74's
  details).
