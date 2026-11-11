# Design

## Context

**The brief.** A timber release's tag names a release commit, off main,
that writes the release's version into the tree: the workspace's
`Cargo.toml` and every file that names timber's version. A build from
the tag therefore reports that version and names its packages by it,
with no variable or argument from the builder. main's tree names
`0.0.0`. This is the family's one way, as Flume's release, crew's
`code.10` and sluice's `a-sluice-release-reports-its-own-version` have
it. Sources: agentplot-design's reading of 2026-11-11 (timber v0.5.3,
boomdrive v0.7.3); agentplot/blueprints `design/principles.md` 14;
agentplot/boomdrive record 0023.

**timber today, read on this branch's tree on 2026-11-11:**

- `Cargo.toml`'s `[workspace.package]` gives `version = "0.0.0"`, which
  the five crates take (`version.workspace = true`). Its comment says
  the released version is the tag's, given to the flake as
  `TIMBER_VERSION` and never to the binary. `Cargo.lock`'s entries for
  `timber`, `timber-conformance`, `timber-core`, `timber-fakes` and
  `timber-seams`, the entries with no `source`, give `0.0.0`.
- `plugins/claude-code/.claude-plugin/plugin.json` and
  `plugins/herdr/herdr-plugin.toml` give `0.1.0`, which no release reads
  or writes. `.claude-plugin/marketplace.json` names no version.
  `site/package.json` names none either; it is the site's.
- `flake.nix` versions the package by `builtins.getEnv "TIMBER_VERSION"`
  where set, else by `Cargo.toml`'s. It gives the build
  `TIMBER_REVISION = self.rev or self.dirtyRev or "unknown"`.
- `crates/timber/build.rs` stamps `TIMBER_REVISION`, from the flake or
  git's HEAD, and `TIMBER_BUILD`, the compiler and target.
  `crates/timber/src/lib.rs`'s `VERSION_LINE` is
  `revision <rev>, built by <build>`, so `timber --version` prints
  `timber revision …, built by …`. Its unit test
  `the_binary_names_a_revision_and_no_version` holds that, citing 24 and
  C.11, as record 0011 decided.
- `.github/workflows/ci.yml`: `build` runs `nix build .#default` and
  `timber --version`. `conventional` starts at
  `git describe --tags --abbrev=0 --match 'v*'`. `version` runs
  `git-cliff --bumped-version`. `binaries` runs
  `TIMBER_VERSION="${TAG#v}" nix build --impure .#default` on three
  systems. `release` runs `gh release create "$TAG" --target "$GITHUB_SHA"`.
  Every tag up to v0.5.3 names a landing commit on main.
- `cliff.toml`'s header says the version lives only in the tag (24, C.11).
- The root project (`moon.yml`) holds the checks that span the tree, one
  task each. treefmt owns `*.sh` through shellcheck and shfmt, and
  actionlint reads the workflows (`treefmt.nix`).

**The family's form, read on agentplot/flume's main at e2973d57:**
`set-version.sh <x.y.z> [<root>]` writes the version into every file
that carries Flume's version, each by an awk program that refuses a
file it writes nothing in. Main gives `0.0.0` in each. `check-versions.sh`,
`root:versions`, holds each file to the workspace's version and runs the
writer on a scratch copy. The release job runs `set-version.sh` on the
pushed commit, commits `chore(release): $TAG`, tags that commit and
pushes the tag. Its conventional job starts at
`git tag --list 'v*' --sort=-v:refname | head -n 1`, and its version job
runs `git-cliff --unreleased --bumped-version`. Flume's binaries job
builds with cargo and its binary names no revision, so building the
written tree uncommitted costs it nothing.

**The statement.** 24 and C.11 bind the builds a timber publish offers
for a repository it serves: the build the landing's verdicts name never
knows the version it becomes, and a compiled binary a projection packs
carries the revision and the build identity, never the version. timber's
own GitHub release is no timber publish. CI's binaries job builds the
binaries itself and no verdict names that build. Record 0011 extended 24
and C.11 to timber's own binary. Under this change the statement holds
as written, and record 0031 supersedes record 0011's half on what the
binary says.

## Goals / Non-Goals

**Goals:**

- A build of a release's tag reports the tag's version, whether timber's
  CI builds it, another flake builds timber's package, or cargo builds
  the checkout. The build is given nothing.
- The binary released under a tag names the commit that tag names.
- CI keeps finding the next version and checking the commits since the
  last release once tags no longer stand on main's history.

**Non-Goals:**

- Anything timber does for a repository it serves (24, C.11 unchanged).
- A `release` branch for the plugins, a tagged install command, and
  Boomdrive's package of timber (proposal, Out of scope).

## Decisions

### 1. Every file that carries timber's version gives `0.0.0` on main

The files are `Cargo.toml`'s `[workspace.package]`, `Cargo.lock`'s
entries with no `source`, the Claude Code plugin's manifest and the
herdr plugin's manifest. The two manifests move from `0.1.0` to `0.0.0`,
so main never states a version no release has, and a release's
manifests state its own. `Cargo.lock` is among them because
`nix build` builds with the lock as it is, and a lock whose
workspace entries disagree with `Cargo.toml` fails a locked build.

`.claude-plugin/marketplace.json` names no version for its plugin and
stays so. Flume's marketplace entry carries one. timber's would be one
more file to write, read by nothing while the marketplace is read from
main.

Not taken: `0.0.0` in `Cargo.toml` alone, with the manifests left at
`0.1.0`. A release's plugins would then name a version that is not the
release's.

### 2. `set-version.sh` writes them, in Flume's form

`sh set-version.sh <major.minor.patch> [<root>]` at the tree's root is
POSIX sh and awk, so a runner with no Nix shell runs it. It is Flume's
script with timber's files and one addition, `--files`, which prints
the four paths, one per line, so `release-commit.sh` and the checks read
the list from one place. The version is checked before any file is
touched. Each file is rewritten by an awk program into a scratch file
and moved over the original, and a program that writes nothing refuses
the file, naming it:

- `Cargo.toml`: the `version = "…"` line under `[workspace.package]`.
- `Cargo.lock`: the `version = "…"` line of each `[[package]]` entry
  with no `source` line.
- `plugins/claude-code/.claude-plugin/plugin.json`: the
  `"version": "…"` line.
- `plugins/herdr/herdr-plugin.toml`: the `version = "…"` line ahead of
  its first table.

`check-versions.sh`, the root's `versions` task, holds main's files at
`0.0.0` and runs the writer on a scratch copy. Its cases are under
Task notes 1.1.

### 3. `release-commit.sh` makes one commit wherever it runs

The binary names the revision it was built from. A released binary
built from `set-version.sh`'s writes left uncommitted would say
`revision <pushed>-dirty`, which no tag names. So the binaries job
builds the release commit itself, and every job that needs that commit
makes it the same way:

```sh
sh release-commit.sh <major.minor.patch>
```

It refuses a tree whose tracked files differ from HEAD, runs
`set-version.sh`, and commits the files `set-version.sh --files` lists,
by path, as `chore(release): v<x.y.z>`. The author and committer are
`github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>`.
Both dates are HEAD's commit time, `@<%ct> +0000`. It commits with
`--no-verify` and `commit.gpgsign=false`, so no host's hook or setting
changes the commit. One parent, one tree, one message and fixed
identities and dates give one id on every runner.

The version job makes it first and hands on its id as an output. The
binaries and release jobs each make it again and stop unless their id
is that one, so a release whose commits disagree fails before anything
is tagged.

Not taken:

- Building the written tree uncommitted, as Flume does. The binary
  would name a revision no tag names.
- Pushing the tag before the binaries are built. A build that failed
  would leave a tag with no release, which the next run reads as
  released.
- Carrying the commit between jobs as a git bundle artifact. It is more
  machinery for what a deterministic commit gives, and still needs the
  id checked.

`check-release.sh`, the root's `release` task, proves the script in
scratch repositories and reads the workflow and the flake. Its cases
are under Task notes 3.1 to 3.3. It needs git and jq alone and no
credential (core.38).

### 4. The flake takes its package's version from `Cargo.toml` alone

`version = workspace.version`, with `released` and `getEnv` gone, so the
flake is pure again and `nix build .#default` of a tag names its
package `timber-<x.y.z>`. `TIMBER_REVISION` is unchanged: a build of the
tag stamps the release commit. The header comment says the version is
the tree's.

### 5. The binary says its version, its revision and its build

```
timber 0.5.4, revision 3f1c…e9, built by rustc 1.91.0 (… 2025-…) for aarch64-apple-darwin
```

`VERSION_LINE` is
`concat!(env!("CARGO_PKG_VERSION"), ", revision ", env!("TIMBER_REVISION"), ", built by ", env!("TIMBER_BUILD"))`,
and clap prints the name before it. The version is cargo's, from the
tree, so a cargo build of a checkout and a nix build of the tag say the
same. `build.rs` adds nothing. On main a build says `0.0.0`, the
version no release has. The revision and the build stay, since a host
that holds two builds of one version tells them apart by them.

The unit test `the_binary_names_a_revision_and_no_version` goes. A
`cli.rs` test runs the built binary's `--version` and compares the line
whole with `CARGO_PKG_VERSION`, `timber::REVISION` and `timber::BUILD`.

Not taken: a binary that names its revision alone, with the version in
the package's name only. A host would still learn which release it runs
from something other than timber, which principle 14 rules out. Flume's
binary and sluice's say their version.

### 6. CI makes the release from the release commit

- **`build`** (every push and pull request): it fails unless
  `nix eval --raw .#default.version` gives `0.0.0`, and, after
  `nix build .#default`, unless `timber --version` begins
  `timber 0.0.0, revision $(git rev-parse HEAD), built by `. That shows
  a build of main's tree names its package and itself by main's version
  and the commit built.
- **`conventional`:** `last` is the first of
  `git tag --list 'v*' --sort=-v:refname`. A release commit's parent is
  the pushed commit, so `R..HEAD` is the commits main took since.
- **`version`:** `git-cliff --unreleased --bumped-version`; where the tag
  is new, `sh release-commit.sh "${tag#v}"`, with outputs `tag` and
  `commit`.
- **`binaries`:** `sh release-commit.sh "${TAG#v}"`, the id compared with
  `COMMIT`; `nix eval --raw .#default.version` must give `${TAG#v}`;
  `nix build .#default --print-build-logs` with no
  `--impure`; `timber --version` must begin
  `timber ${TAG#v}, revision $COMMIT, built by `; then the tarball as
  today.
- **`release`:** git-cliff's `--unreleased --tag "$TAG"` notes, run
  before the release commit so the range is main's; then
  `sh release-commit.sh "${TAG#v}"`, the id compared with `COMMIT`;
  `git tag "$TAG"`; `git push origin "refs/tags/$TAG"`;
  `gh release create "$TAG" --verify-tag --title "$TAG" --notes-file notes.md timber-*.tar.gz`.
  No `--target`, no branch and nothing to main.

`cliff.toml`'s header and the workflow's header and job comments say
the version lives in the tree of the tag's release commit.
`devenv.nix`'s `release-preview` already runs `git cliff --unreleased`
and is unchanged.

### 7. The record and the rule

Record 0031 records the decision, accepted on the brief. It supersedes
record 0011's half on what the binary says, and record 0011's status
line says so. Rule core.74, under a new heading `### The release` in
`docs/architecture/core.md`, binds a diff to it (Task notes 3.5).
README says how a release names its version.

### Rules relied on

core.23 (24 and C.11 hold as written; no clause is worked around),
core.24, core.25 (two root tasks, one check each, and the new scripts
under shellcheck and shfmt), core.26 (the checks run the shell's git and
jq), core.38 (scratch repositories, no credential).

### Departures

None from a rule of `docs/architecture/core.md`. Record 0011's half on
what the binary says is superseded by record 0031 (Decision 5).

## Risks / Trade-offs

- [A release commit made differently on two runners] → Every job checks
  its id against the version job's. A mismatch fails the run before the
  tag is pushed, and `root:release` proves two clones make one id.
- [A tag pushed but `gh release create` failing] → The next run finds
  the tag and releases nothing. The cure is `gh release create` by hand
  at the tag. Flume and crew carry the same risk.
- [git-cliff and a tag off main's history] → `--unreleased` starts the
  range at the newest tag. Flume's CI releases this way.
- [timber publishing itself in phase 1] → Its offered binary would be a
  build of main, saying `0.0.0`, while the GitHub release's says the
  version. That unit's design reconciles the two (proposal, Out of
  scope).
- [`nix build` of a shallow checkout with a local commit] → The
  binaries job builds a shallow checkout today, and the local commit is
  on top of it.

## Migration Plan

Nothing moves. The tags up to v0.5.3 name main's own commits and stay.
The first push to main after this lands is released the new way:
git-cliff finds v0.5.3 as the newest tag, the version job makes the
release commit on the pushed commit, and the release job tags it.

## Task notes

**1.1** The tree's version and its writer:

- `plugins/claude-code/.claude-plugin/plugin.json` and
  `plugins/herdr/herdr-plugin.toml`: `0.0.0`.
- `Cargo.toml`'s comment above `version`, in place of today's:
  "main gives 0.0.0, the version no release has. A release's tag names
  the release commit that writes the release's version here and in
  every file that carries timber's (set-version.sh, release-commit.sh),
  so a build takes its version from the tree alone and the binary says
  it (record 0031)."
- `set-version.sh` at the root as Decision 2, its header naming the
  files, who runs it and `root:versions`, as Flume's does. `--files`
  takes no other argument. Usage:
  `usage: set-version.sh <major.minor.patch> [<root>] | --files`.
- `check-versions.sh` at the root, with `set -u`, its scratch under
  `${SLUICE_TMP:-${TMPDIR:-/tmp}}` and removed on exit. Cases:
  - The workspace's version is `0.0.0`, else it fails saying main's tree
    names no release.
  - Each sourceless `Cargo.lock` entry, the Claude Code manifest (`jq`)
    and the herdr manifest's top `version` give the workspace's version.
    It fails naming the file, the entry where there is one, and the
    version it gives, or `none`.
  - The same agreement, read on a scratch copy whose herdr manifest
    gives `0.1.0`, reports `plugins/herdr/herdr-plugin.toml` and `0.1.0`;
    the check passes only where it does.
  - `set-version.sh --files` lists exactly the four files this check
    reads.
  - `sh set-version.sh 98.76.54 <scratch>` leaves every file giving
    `98.76.54`, and each file's `diff` from the tree's changes as many
    lines as it adds, each added line naming `"98.76.54"`.
  - `sh set-version.sh 0.56 <scratch>` exits non-zero and leaves the
    scratch copy as it was.
  - A scratch copy whose Claude Code manifest has no version line is
    refused, the words naming `plugins/claude-code/.claude-plugin/plugin.json`.
- `moon.yml`: the root task `versions`, `command: 'sh check-versions.sh'`,
  inputs `/Cargo.toml`, `/Cargo.lock`, the two manifests,
  `/set-version.sh`, `/check-versions.sh`, `/devenv.nix` and
  `/devenv.lock`, tags `[check-fast]`, `runFromWorkspaceRoot`, with a
  comment saying what it holds.

**2.1** The binary:

- `crates/timber/src/lib.rs`: `VERSION_LINE` as Decision 5, its doc
  comment saying what `--version` prints. `REVISION`'s doc comment says
  the binary names its revision beside its version, which is the tree's
  (record 0031). Remove the `tests` module's
  `the_binary_names_a_revision_and_no_version`, and the module if it is
  left empty.
- `crates/timber/build.rs`'s header: what the binary says of itself
  beside the version cargo gives from the tree, the revision it was
  built from and the build that made it. Drop "never a version
  (statement 24, C.11)".
- `crates/timber/tests/cli.rs`, under a comment
  `// Requirement: The binary says its version, its revision and its build`:
  `the_binary_says_its_version_its_revision_and_its_build` runs
  `env!("CARGO_BIN_EXE_timber") --version` and asserts exit 0 and stdout
  equal to
  `format!("timber {}, revision {}, built by {}\n", env!("CARGO_PKG_VERSION"), timber::REVISION, timber::BUILD)`.

**3.1** `release-commit.sh` and its check:

- `release-commit.sh` at the root as Decision 3, POSIX sh and git. Its
  header says it makes the release commit the version, binaries and
  release jobs each make, one id wherever it runs. It runs from the
  root of the checkout it is in. It exits 2 with its usage on anything
  but one argument. It refuses a tree that differs from HEAD with
  `release-commit: the tree differs from its commit <short id>`.
- `check-release.sh` at the root, with `set -u` and its scratch as
  `check-versions.sh`'s. It copies `set-version.sh`,
  `release-commit.sh` and the four files into a scratch repository,
  commits them with a fixed identity and date, and clones it twice.
  Cases:
  - `sh release-commit.sh 98.76.54` in each clone gives one id.
  - The commit's subject is `chore(release): v98.76.54`, its one parent
    is the scratch commit, and its author and committer are the bot.
    Both of its times are the parent's `%ct`.
  - `git diff --name-only HEAD^ HEAD`, sorted, is
    `set-version.sh --files`, sorted.
  - In a third clone with a tracked file changed, it exits non-zero
    with the words above, and HEAD is the scratch commit.
  - `sh release-commit.sh 0.56` in a clean clone exits non-zero, HEAD
    unmoved and the tree clean.
- `moon.yml`: the root task `release`, `command: 'sh check-release.sh'`,
  inputs `/set-version.sh`, `/release-commit.sh`, `/check-release.sh`,
  `/Cargo.toml`, `/Cargo.lock`, the two manifests,
  `/.github/workflows/ci.yml`, `/flake.nix`, `/devenv.nix` and
  `/devenv.lock`, tags `[check-fast]`, `runFromWorkspaceRoot`.

**3.2** The flake, as Decision 4. Add to `check-release.sh`: `flake.nix`
names no `TIMBER_VERSION` and calls no `getEnv`.

**3.3** The workflow and `cliff.toml`, as Decision 6. Add to
`check-release.sh`, reading each job of `ci.yml` as the lines from
`^  <job>:$` to the next job:

- No line of `ci.yml` names `TIMBER_VERSION`, and no `nix build` in it
  takes `--impure`.
- `version` runs `git-cliff --unreleased --bumped-version` and
  `sh release-commit.sh`, and has outputs `tag` and `commit`.
- `build` compares `nix eval --raw .#default.version` with `0.0.0` and
  matches `timber --version` against
  `timber 0.0.0, revision $(git rev-parse HEAD), built by `.
- `binaries` runs `sh release-commit.sh "${TAG#v}"`, compares
  `git rev-parse HEAD` with `$COMMIT` and `nix eval --raw .#default.version`
  with `${TAG#v}`, runs `nix build .#default`, and matches
  `timber --version` against `timber ${TAG#v}, revision $COMMIT, built by `.
- `release` runs `git-cliff --unreleased`, `sh release-commit.sh "${TAG#v}"`,
  the same comparison, `git tag "$TAG"`, exactly one `git push`, of
  `"refs/tags/$TAG"` with no `+` and no `--force`, and
  `gh release create "$TAG" --verify-tag`, with no `--target`.
- `conventional` names `git tag --list 'v*' --sort=-v:refname` and no
  `git describe`.

**3.4** `docs/adr/0031-a-release-s-tag-names-a-release-commit-that-writes-timber-s-version-and-the-binary-says-it.md`,
MADR as record 0030: number 31, status accepted, date 2026-11-11.

- Status line: accepted on the brief of the change
  `a-timber-release-reports-its-own-version`, quoting the brief's first
  three sentences. The family's decision is principle 14 and
  agentplot/boomdrive record 0023, and how timber carries it out is the
  change's own, cheap to change.
- Sources: principle 14; boomdrive record 0023; Flume's
  `set-version.sh` and release job at e2973d57; crew's `code.10` (#73 at
  81a883f3); sluice's change; clauses 24 and C.11; record 0011.
- Considered options: the version handed to the build, as today; the
  version written uncommitted for the binaries, as Flume's; the release
  commit made the same in every job (chosen); the binary naming its
  revision alone.
- Decision outcome as Decisions 1 to 6, the reading of 24 and C.11 in
  Context, and the consequences: the plugins read at main still say
  `0.0.0`; Boomdrive takes timber's package with no version of its own;
  timber publishing itself is that unit's to reconcile.
- In record 0011, under its `Deciders` line, add: "Superseded in part
  by record 0031: the binary says its version beside its revision and
  its build, and a release's tag names a release commit."

**3.5** Rule core.74 in `docs/architecture/core.md`, under a new
`### The release` after `### The gate`:

- **[core.74]** "On main every file that carries timber's version gives
  `0.0.0`; a release's tag names a release commit, cut from the commit
  it releases, that writes the release's version into each, and a build
  takes its version from its tree alone."
- Its details, last in `## Details`: "Rules out: a version handed to a build
  by a variable, an argument or an impure read, such as
  `TIMBER_VERSION`; a file that carries timber's version giving another
  than the workspace's; a release's binaries built from any tree but the
  one its tag names; a release commit pushed to main or any branch, or a
  tag moved; a tag on a commit of main's own made after record 0031. Not
  ruled out: the tags v0.1.0 to v0.5.3, which name main's own commits; a
  build of main, or of any commit no tag names, saying `0.0.0`; the
  binary saying its version beside its revision, timber's own release
  being no publish of timber's (24 and C.11 bind the builds a publish
  offers). Source: agentplot/blueprints principle 14; agentplot/boomdrive
  record 0023; record 0031."

README, under "This repository": rows for `set-version.sh` and
`release-commit.sh`, and a paragraph after the commands. It says CI
releases each push to main that git-cliff finds a new version in, that
main's tree names `0.0.0`, and that the release's tag names a release
commit, cut from the pushed commit, that writes the version into every
file carrying timber's, so a build of the tag says that version with
nothing given. It ends with `devenv shell -- release-preview`, which
shows the next release.
