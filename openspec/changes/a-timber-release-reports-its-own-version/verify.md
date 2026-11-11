# Verify: a-timber-release-reports-its-own-version

This report checks the work on
`change/a-timber-release-reports-its-own-version` at `1dc2660` against
this change's proposal, design, two delta specs and tasks. The branch
was cut from `bolt/a-timber-release-reports-its-own-version`.

**The work falls short in one place, and the change is not archived.**
The whole checks pass, every task is ticked, and every requirement and
scenario of the two delta specs is met in the tree and held by a check.
One test is left that names the behaviour record 0031 supersedes:
`crates/timber/tests/cli.rs:922`,
`the_binary_says_its_revision_and_no_version`. It goes back to the code
stage (see The finding).

## The checks

Run once each, on the branch as it stood, as `CLAUDE.md`'s layers name
them.

| check | answer |
|---|---|
| `devenv shell -- moon run ':#check-fast'` | exit 0. 23 tasks completed, 5 of them from moon's cache. Every crate's tests passed with 0 failed; `cli:test` ran 620 tests in its main binary, `the_binary_says_its_version_its_revision_and_its_build` among them. `root:versions`, `root:release` and `root:adrs` passed. |
| `devenv shell -- moon run ':#check-tools'` | exit 0. 1 task completed, its 14 test binaries all passing with 0 failed. |
| `devenv shell -- openspec validate a-timber-release-reports-its-own-version --strict` | valid |
| git-cliff 2.14.1, the shell's, in a scratch repository: `v0.5.3` on main, `v0.5.4` on a release commit off main whose parent is main's `fix: two`, then `feat: three` on main | `git-cliff --unreleased --bumped-version` gives `v0.6.0`, and `git-cliff --unreleased --tag v0.6.0` lists `feat: three` alone. The version and release jobs' reckoning holds with tags off main's history (Decision 6). |

## The tasks

Tasks 1.1 to 3.5 are all ticked, 7 of them, and each one's Verify holds:

- 1.1: both plugin manifests give `0.0.0`, and `Cargo.toml`'s comment
  is the one Task notes 1.1 gives. `set-version.sh` writes the four
  files, each by its own awk program, beside the original, and moves
  none until every one is written; `--files` lists the four. The root's
  `versions` task runs `check-versions.sh`, which holds every case of
  Task notes 1.1, and passed in check-fast.
- 2.1: `VERSION_LINE` is `CARGO_PKG_VERSION, revision …, built by …`.
  The unit test `the_binary_names_a_revision_and_no_version` is gone from
  `lib.rs`, and the `tests` module with it. `build.rs`'s header and the
  doc comments of `REVISION` and `VERSION_LINE` say what the design
  says. `cli.rs` has
  `the_binary_says_its_version_its_revision_and_its_build`, comparing
  the whole line, and it passed.
- 3.1: `release-commit.sh` refuses a tree that differs from HEAD, runs
  `set-version.sh` and commits the listed files by path as the bot,
  dated at HEAD's commit time, with no signing and no hooks.
  `check-release.sh`, the root's `release` task, proves two clones make
  one id, one of them under a host's settings that would sign, run hooks
  and name another author, and holds every case of Task notes 3.1.
- 3.2: `flake.nix` takes `inherit (workspace) version;` and names no
  `TIMBER_VERSION` and no `getEnv`; its header says so.
  `check-release.sh` holds it.
- 3.3: `ci.yml`'s `build`, `conventional`, `version`, `binaries` and
  `release` jobs are as Decision 6. No line names `TIMBER_VERSION` and no
  `nix build` takes `--impure`. The release job writes its notes before
  the release commit, pushes `refs/tags/$TAG` alone and publishes with
  `--verify-tag` and no `--target`. `cliff.toml`'s header and the
  workflow's comments say where the version lives. `check-release.sh`
  reads each job for each step Task notes 3.3 names.
- 3.4: record 0031 is accepted on the brief, quoting its first three
  sentences, with the four options, the reading of 24 and C.11 and the
  consequences. Record 0011 carries the line "Superseded in part by
  record 0031 …" under its `Deciders` line. `root:adrs` passed.
- 3.5: `grep -c 'core\.74' docs/architecture/core.md` prints `2`, the
  rule under `### The release` and its details last in `## Details`.
  README has rows for `set-version.sh` and `release-commit.sh` and the
  paragraph after the commands.

## The specs

`release-version`:

- Main's tree names no release: `check-versions.sh` holds the workspace
  at `0.0.0` and each sourceless `Cargo.lock` entry and both manifests to
  it, and reports a scratch copy whose herdr manifest gives `0.1.0`,
  naming the file and `0.1.0`.
- set-version.sh writes a release's version: `98.76.54` on a scratch
  copy changes exactly the lines that give the version; `0.56` is
  refused with `0.56 is not a version major.minor.patch` and the copy
  unchanged; the Claude Code manifest with no version is refused naming
  it, nothing moved and no scratch file left; `--files` prints the four
  paths and writes nothing.
- release-commit.sh makes one release commit wherever it runs: two
  clones, one id, its subject, its one parent, the bot as author and
  committer at the parent's commit time, exactly the listed files
  changed; a changed tree refused with HEAD unmoved; `0.56` refused with
  HEAD unmoved and the tree clean.
- A release's tag names its release commit, the binaries built from it
  with nothing given, a build takes its version from its tree alone, and
  CI reckons from the newest tag by version: each scenario is a reading
  of `ci.yml` and `flake.nix` that `check-release.sh` makes, and passed.

`command-line`:

- The binary says its version, its revision and its build: the built
  binary's `--version` is compared whole with
  `timber <CARGO_PKG_VERSION>, revision <REVISION>, built by <BUILD>`,
  and `build.rs` stamps `BUILD` as `<compiler> for <target>`.

## The finding

`crates/timber/tests/cli.rs:922`, under
`// Requirement: Publish and verify take what the statement gives them`,
still holds the test `the_binary_says_its_revision_and_no_version`. Its
body compares `timber --version` with `timber {VERSION_LINE}`, so it
passes, but `VERSION_LINE` now begins with the version: the test's name
says the binary names no version, which record 0031 and the
`command-line` requirement this change adds say it does. Task 2.1 and
Decision 5 remove the test that held a revision and no version; this is
the second such test, which neither names, and it was left. With it the
tree says two contrary things of what `timber --version` prints, and it
repeats what `the_binary_says_its_version_its_revision_and_its_build`
already holds.

The code stage removes it, the new test under its own requirement
heading being the one that holds the line, and runs check-fast again.
Nothing else in the tree, the site or the specs still says the binary
names no version or that a build is handed `TIMBER_VERSION`.
