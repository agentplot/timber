## Purpose

How timber itself is released, so that a build of a release's tag reports
that release's version from timber's own tree, with nothing handed to the
build (agentplot/blueprints principle 14; record 0031; rule core.74).

## ADDED Requirements

### Requirement: Main's tree names no release

On main, every file that carries timber's version SHALL give `0.0.0`:
`Cargo.toml`'s `[workspace.package]`, which each crate takes,
`Cargo.lock`'s entries for timber's own crates, the Claude Code plugin's
manifest `plugins/claude-code/.claude-plugin/plugin.json`, and the herdr
plugin's manifest `plugins/herdr/herdr-plugin.toml`.

#### Scenario: Main's files

- **WHEN** `root:versions` reads main's tree
- **THEN** the workspace's version, each of timber's five crates in `Cargo.lock`, and each plugin's manifest give `0.0.0`, and the check passes

#### Scenario: A manifest at another version

- **WHEN** `root:versions`'s agreement reads a scratch copy of main's files whose herdr plugin's manifest gives `0.1.0`
- **THEN** it reports the copy, naming `plugins/herdr/herdr-plugin.toml` and the `0.1.0` it gives

### Requirement: set-version.sh writes a release's version into every file that carries timber's

`sh set-version.sh <x.y.z> [<root>]` SHALL write the version into every
file that carries timber's version under the root, the folder it is in
by default, and change no other line. It SHALL refuse a version that is
not `x.y.z`, and a file in which it finds no version, naming the file
and exiting non-zero. `sh set-version.sh --files` SHALL list the files
it writes, one per line.

#### Scenario: A release's version

- **WHEN** `sh set-version.sh 98.76.54 <copy>` runs on a scratch copy of main's files
- **THEN** each file gives `98.76.54`, and each file's difference from main's is exactly the lines that give the version

#### Scenario: Not a version

- **WHEN** `sh set-version.sh 0.56 <copy>` runs
- **THEN** it exits non-zero saying `0.56` is not a version `major.minor.patch`, and no file of the copy changes

#### Scenario: A file with no version

- **WHEN** `sh set-version.sh 98.76.54 <copy>` runs where the copy's Claude Code plugin manifest gives no `version`
- **THEN** it exits non-zero naming `plugins/claude-code/.claude-plugin/plugin.json`

#### Scenario: The files it writes

- **WHEN** `sh set-version.sh --files` runs
- **THEN** it prints `Cargo.toml`, `Cargo.lock`, `plugins/claude-code/.claude-plugin/plugin.json` and `plugins/herdr/herdr-plugin.toml`, one per line, and writes nothing

### Requirement: release-commit.sh makes one release commit wherever it runs

`sh release-commit.sh <x.y.z>` SHALL run `set-version.sh` on the commit
checked out and commit the files it writes, by path, as
`chore(release): v<x.y.z>`, with that commit as its one parent, by
`github-actions[bot]` as author and committer, both dated as the parent
was committed. It SHALL refuse a tree whose tracked files differ from
the commit checked out, committing nothing.

#### Scenario: Two clones of one commit

- **WHEN** `sh release-commit.sh 98.76.54` runs in two clones of one scratch commit
- **THEN** both make a commit with the same id, whose subject is `chore(release): v98.76.54`, whose one parent is the scratch commit, whose author and committer are `github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>` dated at the parent's commit time, and which changes exactly the files `set-version.sh --files` lists

#### Scenario: A tree that differs from its commit

- **WHEN** `sh release-commit.sh 98.76.54` runs in a clone where a tracked file has been changed
- **THEN** it exits non-zero saying the tree differs from its commit, and HEAD has not moved

#### Scenario: Not a version

- **WHEN** `sh release-commit.sh 0.56` runs in a clean clone
- **THEN** it exits non-zero, no file changes and HEAD has not moved

### Requirement: A release's tag names its release commit

CI SHALL release a version by tagging the release commit
`release-commit.sh` makes on the pushed commit. The version job SHALL
make it and hand on its id; the release job SHALL make it again, refuse
unless its id is the one handed on, tag it, push the tag and no other
ref, and publish the GitHub release with `--verify-tag`. Nothing SHALL
be committed to main, no branch pushed and no tag moved.

#### Scenario: The version and release jobs

- **WHEN** `root:release` reads `.github/workflows/ci.yml`
- **THEN** the version job runs `sh release-commit.sh` and hands on the commit's id; the release job runs `sh release-commit.sh "${TAG#v}"`, compares its commit with that id, runs `git tag "$TAG"`, pushes `refs/tags/$TAG` alone with no `+` and no `--force`, and runs `gh release create "$TAG" --verify-tag` with no `--target`

### Requirement: The release's binaries are built from the release commit with nothing given

CI's binaries job SHALL make the release commit with
`release-commit.sh`, refuse unless its id is the version job's, and
build it with `nix build .#default` given no variable, no `--impure` and
no argument. It SHALL fail unless the binary's `timber --version` names
the release's version and that commit as its revision.

#### Scenario: The binaries job

- **WHEN** `root:release` reads `.github/workflows/ci.yml`
- **THEN** the binaries job runs `sh release-commit.sh "${TAG#v}"`, compares its commit with the version job's, runs `nix build .#default` with no `--impure`, and fails unless `timber --version` begins `timber ${TAG#v}, revision <that commit>, built by `

### Requirement: A build takes its version from its tree alone

timber's flake SHALL version its package by `Cargo.toml`'s
`[workspace.package]` version and nothing else, and no part of the
release SHALL hand a build a version by a variable, an argument or an
impure read.

#### Scenario: Nothing hands a build a version

- **WHEN** `root:release` reads `flake.nix` and `.github/workflows/ci.yml`
- **THEN** neither names `TIMBER_VERSION`, `flake.nix` calls no `getEnv`, and the workflow passes `--impure` to no `nix build`

#### Scenario: The flake's package names the tree's version

- **WHEN** `root:release` reads `.github/workflows/ci.yml`
- **THEN** the build job fails unless `nix eval --raw .#default.version` gives `0.0.0`, and the binaries job fails unless, on the release commit, it gives `${TAG#v}`

### Requirement: CI reckons the next release from the newest tag by version

CI's version job SHALL reckon the next version with git-cliff's
`--unreleased` and `--bumped-version`, the release's notes SHALL be
git-cliff's `--unreleased` notes, and the conventional check SHALL check
the commits since the newest `v*` tag by version, since a release's tag
names a commit off main's history.

#### Scenario: The version, notes and conventional steps

- **WHEN** `root:release` reads `.github/workflows/ci.yml`
- **THEN** the version job runs `git-cliff --unreleased --bumped-version`, the release job's notes run `git-cliff --unreleased`, and the conventional job's range starts at the first of `git tag --list 'v*' --sort=-v:refname`, with no `git describe`
