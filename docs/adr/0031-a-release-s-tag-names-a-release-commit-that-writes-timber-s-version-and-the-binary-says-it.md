---
number: 31
title: A release's tag names a release commit that writes timber's version, and the binary says it
status: accepted
date: 2026-11-11
---

# A release's tag names a release commit that writes timber's version, and the binary says it

Status: accepted on the brief of the change a-timber-release-reports-its-own-version, 2026-11-11, which decides it in its words: "A timber release's tag names a release commit, off main, that writes the release's version into the tree: the workspace's `Cargo.toml` and every file that names timber's version. A build from the tag therefore reports that version and names its packages by it, with no variable or argument from the builder. main's tree names `0.0.0`." The family's decision is agentplot/blueprints `design/principles.md` 14 and agentplot/boomdrive record 0023; how timber carries it out below is the change's own, cheap to change.
Sources: agentplot/blueprints `design/principles.md` 14: "A tool that needs what another program knows reads that program's own source, never a file a host makes for it."; agentplot/boomdrive record 0023, the set taking each tool's own package; Flume's `set-version.sh`, `check-versions.sh` and release job at e2973d57, and its tag v0.21.2, which names `chore(release): v0.21.2`; crew's `code.10` and its change a-crew-release-reports-its-own-version, landed as #73 at 81a883f3; sluice's change a-sluice-release-reports-its-own-version and its record 0111; statement clauses 24 and C.11; record 0011.

## Context and problem statement

A build of a timber release's tag did not know the tag's version. CI tagged the commit of main it released, whose tree says `0.0.0`: at v0.5.3, `Cargo.toml`'s `[workspace.package]` gives `0.0.0` and the tag names a landing commit. Only CI's binaries job was told the version, as `TIMBER_VERSION="${TAG#v}" nix build --impure`, which the flake read with `builtins.getEnv` for the package's name alone. The binary said no version at all: `timber --version` printed the revision and the build, as record 0011 decided. Any other builder of the tag, Boomdrive's set among them, got a package `timber-0.0.0` whose binary could not say which release it is, unless the builder named the version itself, which principle 14 rules out.

The plugins' manifests, `plugins/claude-code/.claude-plugin/plugin.json` and `plugins/herdr/herdr-plugin.toml`, gave `0.1.0`, a version no release had.

**The reading of 24 and C.11.** They bind the builds a timber publish offers for a repository it serves: the build the landing's verdicts name never knows the version it becomes, and a compiled binary a projection packs carries the revision and the build identity, never the version. timber's own GitHub release is no timber publish: CI's binaries job builds the binaries itself, and no verdict names that build. Record 0011 extended 24 and C.11 to timber's own binary. Under this record the statement holds as written, and its half on what the binary says is superseded.

## Considered options

1. The version handed to the build, as before: `TIMBER_VERSION` and `--impure` in CI, and every other builder naming it.
2. The version written uncommitted for the binaries, as Flume's binaries job writes it and builds the tree as it stands.
3. The release commit made the same in every job that needs it, and the tag on it.
4. The binary naming its revision alone, the version in the package's name only.

## Decision outcome

Option 3, the family's form, as Flume's release, crew's `code.10` and sluice's record 0111 have it.

- **Every file that carries timber's version gives `0.0.0` on main.** They are `Cargo.toml`'s `[workspace.package]`, which the five crates take; `Cargo.lock`'s entries with no `source`, which a locked build reads and which must agree with `Cargo.toml`; the Claude Code plugin's manifest; and the herdr plugin's. The two manifests move from `0.1.0`, so main never states a version no release has and a release's manifests state its own. `.claude-plugin/marketplace.json` names no version and stays so.
- **`set-version.sh` writes them, in Flume's form.** `sh set-version.sh <major.minor.patch> [<root>]`, POSIX sh and awk, checks the version before it touches a file, writes each file by an awk program of its own beside the original, and refuses a file its program wrote nothing in, naming it; only once every file is written is any moved over its original, so a refusal leaves the tree as it was. `sh set-version.sh --files` lists the four files from the one list the writer walks, so `release-commit.sh` and the checks read it from one place. `root:versions` (`check-versions.sh`) holds main's files at `0.0.0` and to one another, and runs the writer on a scratch copy: a release's version written into each and nothing else changed, `0.56` refused, and the Claude Code manifest with no version refused, naming it, with nothing written.
- **`release-commit.sh` makes one commit wherever it runs.** The binary names the revision it was built from, so a binary built from written files left uncommitted would say a revision no tag names. `sh release-commit.sh <major.minor.patch>` refuses a tree whose tracked files differ from HEAD, runs `set-version.sh`, and commits the files `--files` lists, by path, as `chore(release): v<x.y.z>`, the checked-out commit its one parent. The bot, `github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>`, is author and committer, both dated at HEAD's commit time. It commits with `commit.gpgsign=false`, `--no-verify` and `core.hooksPath=/dev/null`, the last because `--no-verify` leaves `prepare-commit-msg` to run, which could change the message. One parent, one tree, one message and fixed identities and dates give one id on every runner. `root:release` (`check-release.sh`) proves it in scratch repositories: two clones of one commit make one id, one of them under a host's settings that would sign, run hooks that write the message and name another author.
- **The flake takes its package's version from `Cargo.toml` alone,** with no `getEnv` and no `TIMBER_VERSION`, so it is pure again and `nix build .#default` of a tag names its package `timber-<x.y.z>`.
- **The binary says its version, its revision and its build.** `timber --version` prints `timber <version>, revision <revision>, built by <build>`, the version cargo's from the tree, so a cargo build of a checkout and a nix build of the tag say the same, and a build of main says `0.0.0`. The revision and the build stay, since a host holding two builds of one version tells them apart by them.
- **CI makes the release from the release commit.** The version job reckons with `git-cliff --unreleased --bumped-version`, since a release's tag is off main's history and `--unreleased` starts the range at the newest tag; where the tag is new it makes the release commit and hands on its id. The binaries job makes it again, stops unless its id is the version job's and the package's version the release's, builds it with `nix build .#default` given nothing, and stops unless `timber --version` begins `timber <x.y.z>, revision <that commit>, built by `. The release job writes git-cliff's `--unreleased` notes before the commit, so the range is main's, makes the commit again and holds it to the id, tags it, pushes the tag alone and publishes with `gh release create --verify-tag`. Nothing is committed to main, no branch is pushed and no tag is moved. The build job holds main's package and binary to `0.0.0` and the binary to the commit built, and the conventional job starts at the first of `git tag --list 'v*' --sort=-v:refname`, a release commit's parent being the commit it released. `root:release` reads each job for these steps.

Not taken:

- **Option 1.** Every builder but timber's CI must know the tag and say it, which principle 14 rules out; the source of a release says it is none.
- **Option 2.** The binary would name a revision, the pushed commit's and dirty, that no tag names.
- **Pushing the tag before the binaries are built.** A build that then failed would leave a tag with no release, which the next run reads as released.
- **Carrying the commit between jobs as a git bundle.** More machinery than a deterministic commit, and its id still needs checking.
- **Option 4.** A host would still learn which release it runs from something other than timber, which principle 14 rules out; Flume's binary and sluice's say their version.

### Consequences

- Good: a build of a release's tag, by timber's CI, `nix build github:agentplot/timber/v<x.y.z>`, a flake that takes timber's package, or cargo in a checkout, reports that release's version and names its package by it, with nothing given.
- Neutral: the plugins are read from main, by the Claude Code marketplace and by herdr, and their manifests there say `0.0.0`, as they said `0.1.0` before; a `release` branch for them is a change of its own.
- Neutral: Boomdrive's set takes timber's package at a tag made this way and names no version of its own (boomdrive record 0023); that is agentplot/boomdrive's, after this record's first release.
- Neutral: when timber publishes timber, the artifact it offers is a build of a landed revision of main, which says `0.0.0` as 24 and C.11 have it, beside the GitHub release's binary, which says its version; that unit's design reconciles the two.
- Neutral: tags made before this record, v0.1.0 to v0.5.3, name main's own commits, whose trees say `0.0.0`, and stay as they are.
- Bad: where `gh release create` fails after the tag is pushed, the next run finds the tag and releases nothing; the cure is `gh release create "$TAG" --verify-tag` by hand with the run's artifacts, as for Flume and crew.
- Rule core.74 says it.
