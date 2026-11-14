## Why

A repository of Dash0 dashboards, one Perses file each with a fixed
identifier, is to reach its dataset as a timber publish from its main
(swancloud record 0062 as amended on 2026-11-14). timber binds two
targets, the Nix cache and the clan push profile, and neither is a
dataset. The statement now has the row: C.12 gains Dash0's dashboards
as a push target, and 118's bindings file gains `[credentials]`, a
credential's name to the path of a file holding only it, which timber
hands to the target's own tool (`boomdrivedev/blueprints`
`design/timber/requirements.md` at aaa66ca, on
`design/timber-deploys-dash0-dashboards`). Record 0033 and rule core.21
say how the token reaches Dash0's CLI. This change builds the target,
its table and the host's credentials.

## What Changes

- **The host's bindings file gains `[credentials]`** (118). Each key is
  a credential's name, and each value the path of a file that holds only
  that credential, absolute or beginning `~/`. The file never holds a
  credential, and timber never opens the file a path names: it hands the
  path to the target's own tool (45, 64, rule core.21). A value that is
  no such path is unreadable, naming the file and the key (rule
  core.53). The binding hands every target profile the credentials and
  the file's path.
- **The Dash0 target**, `crates/timber/src/profiles/target/dash0/`,
  bound as `dash0` (C.12's Dash0 row):
  - Its table is `target.dash0 = { api, dataset, credential, dashboards }`.
    `api` is the API's address, `https://` only. `dataset` is the
    dataset's identifier. `credential` is the name of the host's
    credential. `dashboards` names the component's outputs whose
    artifacts are dashboard files. Any other key is unreadable, naming
    it (45).
  - An environment whose credential `[credentials]` doesn't name is
    unreadable, naming the credential and the bindings file, and nothing
    runs (118).
  - Every run of Dash0's CLI gets each setting as a flag. The token goes
    by its path, `--auth-token-file`. The run has a configuration folder
    of its own, empty, and every `DASH0_*` variable is removed from its
    environment, so no profile or variable of the host's is read
    (record 0033). The CLI is `dash0` as found on `PATH`, as attic and
    clan are.
  - `apply` reads the version's index and each named output's file by
    digest (42, rule core.4). It writes them into the run's folder and
    reads their kinds and identifiers from one dry run of the CLI, which
    takes no token. It refuses, before anything reaches Dash0, a document
    that is no dashboard, a dashboard with no identifier, an identifier
    named twice, and an output with no dashboard.
  - `apply` then writes the marker naming every identifier it may apply,
    applies the files with one `dash0 apply`, and deletes each dashboard
    the marker names that the version doesn't hold. Last, it writes the
    marker naming the version applied. It removes nothing the marker
    doesn't name, so a dashboard made by hand is never removed (C.12).
  - The marker is a dashboard of timber's own in the dataset, named for
    the checkout's repository, the component and the environment, in
    Dash0's folder `/timber`. Its description holds what timber applied
    there (119). An index naming another publishing repository than the
    checkout's is refused before anything runs.
  - `current` and `report` read the marker. They answer the digest it
    names and, for report, when Dash0 says the marker was last written.
    They answer unread where it names an apply that didn't finish, has
    been deleted, or isn't there (89, 91, 119).
- **The target files are read in one family module.** The reading of a
  version's index and a file artifact's bytes moves from `nix_closure.rs`
  to a module every target profile shares. The closure's form stays in
  `nix_closure.rs`, and no profile calls another (rule core.2).
- **The suite runs the target group over the Dash0 target** with a
  scripted Dash0, declaring that it holds a marker (93).
- **The proof against Dash0 is run by hand.** The tag `check-dash0` runs
  the target group and three applies against a throwaway dataset the
  operator names in `TIMBER_TOOLS_DASH0`. The run refuses without the
  setting, naming it. The token is a file a person makes, which no gate
  task reads (rule core.38). The devenv shell pins Dash0's CLI from
  Dash0's own flake at v1.20.0 (rule core.26).
- The site's targets page and self-hosting guide document the target
  and `[credentials]`. Record 0034 holds the choices of design D4 to D8,
  and rule core.75 holds what a removal may remove.

## Capabilities

### New Capabilities

- `dash0-target`: the Dash0 target profile: its table, the credential
  handed by its path, every run's flags and folder, apply's dry run,
  marker, apply and removals, current and report read from the marker,
  and its proofs.

### Modified Capabilities

- `host-bindings`: the file's `[credentials]`, and the binding handing
  the targets the host's credentials.
- `conformance-suite`: the target group runs over the Dash0 target.

## Out of scope, and what each waits on

- `madswan-dev/dashboards` itself: its layout, its checks, its
  `.config/timber.toml` naming its component, outputs, channel and
  `target.dash0` environment, and the build task that leaves its
  dashboard files. That repository's own unit, once ops's chore adds it
  to deneb's afterthought drive, Flume and sluice (swancloud record 0062
  as amended on 2026-11-14).
- The write token and where `[credentials]` points. swancloud declares
  the token's var on deneb, and sets Boomdrive's timber module option
  mapping `dash0-dashboards` to the var's file. The module writes
  timber's host file, and adding that option is Boomdrive's (swancloud
  record 0062 as amended later on 2026-11-14, its record 0059).
- What runs `timber apply` for the dashboards environment, with the
  repository's own pinned `dash0` on `PATH`, never deneb's wrapper: the
  host's, by C.16 and rule core.52. timber starts nothing.
- Other Dash0 asset kinds, such as check rules, views and synthetic
  checks. The C.12 row is dashboards. A later row of the statement takes
  them.
- A second host applying the same environment at once. One host holds
  the write token (the design's risks).

## Impact

- New: `crates/timber/src/profiles/target/dash0/` (the profile, its
  table, its marker, its CLI and the scripted stand-in),
  `crates/timber/src/profiles/target/file_artifact.rs`,
  `crates/timber/tests/dash0.rs` (by hand).
- `crates/timber/src/profiles/target/nix_closure.rs` keeps the
  closure's form and calls the family module. `profiles/target/mod.rs`
  binds `dash0` and hands the credentials.
- `crates/timber/src/bindings/mod.rs` reads `[credentials]` and hands it
  to the target family. No core, seam, fake or command changes: the
  target seam is unchanged, and apply's answer and report are the core's
  already.
- `crates/timber/moon.yml` gains the `dash0` task, tagged
  `check-dash0`. `devenv.yaml` gains Dash0's flake at v1.20.0, and
  `devenv.nix` its CLI and `TIMBER_TOOLS_DASH0_CLI`. No crate,
  dependency, language or file type is added.
- The site's targets page, its integrations index and its self-hosting
  guide. Record 0034 in `docs/adr/` and rule core.75 in
  `docs/architecture/core.md`.
- Rules kept:
  - core.1: Dash0 and its CLI are named only under `profiles/target/`,
    and `[credentials]` is read generically.
  - core.2: the files are read through the family module, and the store
    through the store seam.
  - core.4: the index and each file are read by digest.
  - core.19: Dash0's words are carried verbatim, and absent, refused and
    unreadable are kept apart.
  - core.21: the token's path alone is handed to the CLI, which alone
    reads it, with no `DASH0_*` variable and no `~/.dash0`.
  - core.29: nothing is applied before the core's judge.
  - core.38: the proof against Dash0 runs by hand.
  - core.53: a broken `[credentials]` binds nothing.
  - core.71: the core signs the report once the marker names the index.
