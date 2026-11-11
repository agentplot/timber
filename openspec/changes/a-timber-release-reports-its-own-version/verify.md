# Verify: a-timber-release-reports-its-own-version

This report checks the work on
`change/a-timber-release-reports-its-own-version` at `743def6` against
this change's proposal, design, two delta specs and tasks. The branch
was cut from `bolt/a-timber-release-reports-its-own-version`.

**The work holds, and the change is archived.** The whole checks pass,
every task is ticked, and every requirement and scenario of the two
delta specs is met in the tree and held by a check.

## The checks

Run once each, on the branch as it stood at `743def6`, the tree clean,
as `CLAUDE.md`'s layers name them.

| check | answer |
|---|---|
| `devenv shell -- moon run ':#check-fast'` | exit 0. 23 tasks completed, 17 of them from moon's cache, which keys each by its inputs. Every crate's tests passed with 0 failed; `cli:test`'s main binary ran 620 tests, and `the_binary_says_its_version_its_revision_and_its_build` passed. `root:versions`, `root:release` and `root:adrs` (31 records) passed. |
| `devenv shell -- moon run ':#check-tools'` | exit 0. 1 task completed, `cli:tools`, run afresh in 2m 28s: its 14 test binaries all passed with 0 failed. |
| `devenv shell -- openspec validate a-timber-release-reports-its-own-version --strict` | valid |
| git-cliff 2.14.1, the shell's, in a scratch repository under this tree's `cliff.toml`: `v0.5.3` on main, then `fix: two`; `v0.5.4` on a release commit off main whose parent is `fix: two`; then `feat: three` on main | `git-cliff --unreleased --bumped-version` gives `v0.6.0`; `git tag --list 'v*' --sort=-v:refname` gives `v0.5.4` first; `git-cliff --unreleased --tag v0.6.0` lists `feat: three` alone. The version, release and conventional jobs reckon rightly from a tag off main's history (Decision 6). |

## The tasks

Tasks 1.1 to 3.5 are all ticked, 8 of them, and each one's Verify holds:

- 1.1: both plugin manifests give `0.0.0`, and `Cargo.toml`'s comment
  says how a release writes its version. `set-version.sh` checks the
  version first, writes the four files each by its own awk program
  beside the original, and moves none until every one is written;
  `--files` lists the four from the one list it walks. The five crates
  take `version.workspace = true`, and `Cargo.lock`'s five sourceless
  entries are exactly timber's crates. The root's `versions` task runs
  `check-versions.sh` and passed.
- 2.1: `VERSION_LINE` is `CARGO_PKG_VERSION, revision …, built by …`.
  The unit test `the_binary_names_a_revision_and_no_version` and the
  `tests` module of `lib.rs` are gone. `build.rs`'s header and the doc
  comments of `REVISION` and `VERSION_LINE` say the binary names its
  version beside its revision and its build. `cli.rs`'s
  `the_binary_says_its_version_its_revision_and_its_build` compares the
  whole line and passed.
- 2.2: `crates/timber/tests/cli.rs` holds no
  `the_binary_says_its_revision_and_no_version`; the one test of the
  version line is
  `the_binary_says_its_version_its_revision_and_its_build`, under its own
  requirement heading. check-fast passed.
- 3.1: `release-commit.sh` refuses a tree whose tracked files differ from
  HEAD, runs `set-version.sh` and commits the listed files by path as the
  bot, dated at HEAD's commit time, with `commit.gpgsign=false`,
  `core.hooksPath=/dev/null` and `--no-verify`. `check-release.sh`, the
  root's `release` task, proves two clones make one id, one of them under
  a host's settings that would sign, run hooks and name another author,
  and passed.
- 3.2: `flake.nix` takes `inherit (workspace) version;` and names no
  `TIMBER_VERSION` and no `getEnv`; its header says the version is the
  tree's. `check-release.sh` holds it.
- 3.3: `ci.yml`'s `build`, `conventional`, `version`, `binaries` and
  `release` jobs are as Decision 6. No line names `TIMBER_VERSION` and no
  `nix build` takes `--impure`. The release job, with `contents: write`
  and full history, writes its notes before the release commit, pushes
  `refs/tags/$TAG` alone and publishes with `--verify-tag` and no
  `--target`. `cliff.toml`'s header and the workflow's comments say where
  the version lives. `check-release.sh` reads each job for each step.
- 3.4: record 0031 is accepted on the brief, quoting it, with the four
  options, the reading of 24 and C.11, the paths not taken and the
  consequences. Record 0011 carries "Superseded in part by record 0031"
  under its `Deciders` line. `root:adrs` passed.
- 3.5: `grep -c 'core\.74' docs/architecture/core.md` prints `2`, the
  rule under `### The release` and its details last in `## Details`.
  README has rows for `set-version.sh` and `release-commit.sh` and says
  how a release names its version.

## The specs

`release-version`, each scenario held by `root:versions` or
`root:release`:

- Main's tree names no release: `check-versions.sh` holds the workspace
  at `0.0.0`, each sourceless `Cargo.lock` entry and both manifests to
  it, and reports a scratch copy whose herdr manifest gives `0.1.0`,
  naming the file and `0.1.0`.
- set-version.sh writes a release's version: `98.76.54` on a scratch
  copy changes exactly the lines that give the version; `0.56` is refused
  with `0.56 is not a version major.minor.patch` and the copy unchanged;
  the Claude Code manifest with no version is refused naming it, nothing
  moved and no scratch file left; `--files` prints the four paths and
  writes nothing.
- release-commit.sh makes one release commit wherever it runs: two
  clones, one id, its subject, its one parent, the bot as author and
  committer at the parent's commit time, exactly the listed files
  changed; a changed tree refused saying it differs from its commit,
  HEAD unmoved; `0.56` refused with HEAD unmoved and the tree clean.
- A release's tag names its release commit; the binaries are built from
  it with nothing given; a build takes its version from its tree alone;
  CI reckons from the newest tag by version: each scenario is a reading
  of `ci.yml` and `flake.nix` that `check-release.sh` makes, and passed.
  The reckoning's behaviour under git-cliff is the scratch run in The
  checks.

`command-line`:

- The binary says its version, its revision and its build: the built
  binary's `--version` is compared whole with
  `timber <CARGO_PKG_VERSION>, revision <REVISION>, built by <BUILD>`,
  and `build.rs` stamps `BUILD` as `<compiler> for <target>`. No
  requirement of `openspec/specs/` says the binary names no version, so
  the added requirement contradicts none.

Nothing else in the tree, the site or `openspec/specs/` says the binary
names no version or that a build is handed `TIMBER_VERSION`.
