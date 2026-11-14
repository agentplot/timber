## Context

C.12's row for Dash0, at `boomdrivedev/blueprints` aaa66ca on
`design/timber-deploys-dash0-dashboards`, reads: apply runs the
backend's own CLI, `dash0 apply`, on each dashboard file the version
holds, by its fixed identifier, into the dataset at the API the
environment's table names, `api` and `dataset`, and removes each
dashboard an earlier version applied that this one does not hold, never
one timber did not apply; the token is the host's credential the table
names (118), handed to the CLI by its path, `--auth-token-file`, every
other setting by its flag, with a configuration folder of the run's
own, so no profile or `DASH0_*` variable of the host's is read; current
is the version the dataset's dashboards were last applied from, which
apply records with them. 118 gains `[credentials]`. Record 0033 records
both, and rule core.21 holds them.

What the statement leaves open is the profile's: how a version holds
its dashboard files, how apply learns their identifiers, where and how
apply records what it applied, and where the CLI comes from. Dash0's
CLI (`dash0hq/dash0-cli`, v1.20.0, read at its source on 2026-11-14)
settles what can be built:

- A dashboard is `kind: Dashboard`, its identifier
  `metadata.dash0Extensions.id`, or a Perses `kind: PersesDashboard`,
  its identifier the label `dash0.com/id`. `apply` upserts each by its
  identifier, and creates a fresh one on every run where there is none.
- `apply -f <directory>` reads every `.yaml` and `.yml` under it, a
  file holding one document or several, and validates every document
  before it applies any.
- `apply --dry-run` calls no API and needs no token. In agent mode it
  prints JSON: per file, each change's `op`, `kind`, `name` and
  `originOrId`.
- A dashboard's metadata keeps no labels or annotations of the user's:
  the API's form holds only Dash0's own, `dash0.com/folder-path` among
  them. A Perses document's own labels and annotations, other than
  Dash0's, are dropped on the way in.
- The dataset of every request is the `--dataset` flag's, and a
  document's own dataset is stripped before it is sent.
- `dashboards get <id> -o json` answers a deleted dashboard too,
  annotated `dash0.com/deleted-at`. In agent mode a dashboard that isn't
  there is an error on stderr, `{"error": "dashboard \"<id>\" not
  found…"}`. `dashboards delete <id> --force` succeeds for one already
  gone.
- `--agent-mode`, `--config-dir`, `--auth-token-file` and
  `--no-skill-hint` are the root's flags. `--api-url` and `--dataset`
  are each subcommand's own, so they come after the subcommand. A flag
  wins over the matching `DASH0_*` variable, and that over a profile in
  the configuration folder.

## Goals / Non-Goals

**Goals:**

- A dataset converged on its environment's version: every dashboard the
  version holds applied by its identifier, and every dashboard an
  earlier version applied that this one doesn't hold removed.
- What timber applied to a dataset readable in the dataset, with no
  timber process running (20, 119).
- The token reaching the CLI by its path alone (rule core.21).
- The target group, the profile's refusals and the CLI's runs all
  checked in `check-fast`, with a by-hand proof against Dash0.

**Non-Goals:**

- Assets other than dashboards.
- The dashboards repository, its checks and what runs `timber apply`
  (the proposal's out of scope).
- Locking a dataset against two hosts applying at once.

## Decisions

### D1. The profile is `dash0`, and it reads the version's files through a family module

The profile lives in `profiles/target/dash0/`. Its `mod.rs` holds the
target, `table.rs` the table, `marker.rs` the marker's form, `cli.rs`
the CLI's runs behind a trait private to the profile, `scripted.rs` a
Dash0 that answers as a test sets, and `tests.rs`. The family's `bound`
binds it under `dash0` beside `clan` and `nix_cache`.

The reading of the verified version's index, of an output's file
artifact in it, and of that artifact's bytes by its one layer moves from
`nix_closure.rs` to `profiles/target/file_artifact.rs`. Each function
takes the words naming what the artifact is to the asking profile.
`nix_closure.rs` keeps the closure's form and calls the family module
with a closure file's words, so every answer the Nix profiles give is
unchanged, and their tests pass as they are. The Dash0 profile calls the
same functions with a dashboard file's words. No profile calls another
(rule core.2).

Not taken: calling `nix_closure`'s readers from the Dash0 profile. They
name a closure in every answer they give. A copy of them in the profile
would be a second reading of one form.

### D2. The table is `dash0 = { api, dataset, credential, dashboards }`

```toml
[environments.dashboards.deneb]
channel = "stable"
target.dash0 = { api = "https://api.us-east-1.aws.dash0.com", dataset = "deneb", credential = "dash0-dashboards", dashboards = ["dashboards"] }
```

- `api` is `https://` followed by a host, with no whitespace. The token
  goes there, so an `http://` address would send it in the clear.
- `dataset` is non-empty, with no whitespace, no `/`, and no leading
  `-`.
- `credential` is a non-empty name with no whitespace.
- `dashboards` is a non-empty list of the component's output names, each
  once.

Any other key, a missing key, or one of another form is unreadable at
every operation, naming the environment, the key and the four the table
reads, and nothing runs (45). No token, path or profile can sit in the
table.

`dashboards` follows the Nix cache's `closures`. A component's version
names other artifacts beside its outputs' files, its notes among them,
so the table says which files are dashboards. Each named output is one
file of one or more dashboard documents. A repository may give each
dashboard an output of its own, or one output whose build task joins
them into one file with `---` between them. The file projection packs
either as it packs any file.

Not taken:

- Every `file` artifact of the version as a dashboard file. A component
  that also packs a file of another kind would hand Dash0 a document it
  never meant for it.
- The output named `dashboards` by convention. That is a name the
  repository's authors can't choose.

### D3. The credential is the host's, named by the table, handed by its path

`bindings/mod.rs` reads `[credentials]` generically. Each key is a name.
Each value is a string, a path absolute or beginning `~/`, expanded as
the file's other paths are. It becomes `Credentials { file, paths }`,
the bindings file's path and the map. A value of another kind or form
makes every command bound by the file unreadable, naming the file and
`[credentials] <name>`. The file binds nothing in its place (rule
core.53). A file with no `[credentials]` binds none.

The target family's `bound` takes the credentials and hands them to
each profile that reads one, which today is the Dash0 profile alone.
The profile looks up the table's `credential` at each operation that
reaches Dash0: current, apply and report. A name the map lacks is
unreadable, naming the credential and the bindings file, with nothing
run. timber never opens, stats or copies the file a path names. The
CLI alone reads it, and only when it calls Dash0's API (64, rule
core.21).

The module names no tool and no service (rule core.1).

### D4. Every run of the CLI is given each setting and nothing of the host's

Every run is the program `dash0`, as found on `PATH`, as the Nix
profiles find attic and clan. It runs in a folder the operation makes
under the system's temporary directory and removes after, holding an
empty `config/`. Its arguments are:

```
dash0 --agent-mode --no-skill-hint --config-dir <run>/config \
      --auth-token-file <path> <subcommand…> --api-url <api> --dataset <dataset> …
```

A dry run takes no `--auth-token-file`, `--api-url` or `--dataset`.
Every variable beginning `DASH0_` is removed from the run's environment
(the run module's `env_drop`). With an empty configuration folder there
is no profile, and every setting is a flag, so nothing of the host's own
`dash0` is read. `--agent-mode` gives JSON output and JSON errors with
no prompt, whatever the host's environment detects. A run that exits
other than as the operation allows answers unreadable with the CLI's
stderr verbatim (59, rule core.19).

`dash0` on `PATH` is the repository's own pinned CLI where the host runs
`timber apply` in the repository's shell. So the CLI that validated a
dashboard in the repository's checks applies it, as swancloud record
0062 asks. On deneb, the `dash0` on a person's path is swancloud's
wrapper, which hands the read-only token; a run through it is refused
by Dash0 at the first write, and timber reports that apply failed. It
never writes with a token the table didn't name.

Not taken:

- The CLI built from the revision's flake by timber. That ties a Dash0
  target to Nix and builds a tool at every apply.
- A path to the CLI in the host's bindings file. The host's choice of
  CLI would then differ from the one the repository pins and checks
  with.
- Dash0's `apply --since <ref>`. It takes its removals from the git
  history under `-f`, and the run's folder has none. It would also
  remove what an earlier commit held but timber never applied.

### D5. The version's identifiers come from the CLI's own dry run

Apply writes each named output's bytes into the run's folder as
`dashboards/<n>.yaml`, `<n>` the output's place in the table's list, so
no output's name becomes a path. Then it runs `dash0 … apply -f
<run>/dashboards --dry-run`. It reads the JSON's files and changes, and
maps each file back to its output. It refuses the version, naming the
output and the document where there is one, and running nothing more,
in each of these cases:

- A change whose `kind` is neither `Dashboard` nor `PersesDashboard`.
- A dashboard whose `originOrId` is empty, which would be created anew
  at every apply.
- An identifier two documents give.
- The identifier of the environment's marker (D6).
- An output whose file holds no dashboard.
- A dry run the CLI refuses, carried in its own words.

The dry run is also the validation that the apply's own run repeats.
Not taken: a YAML parser of timber's own. That would be a second reading
of Dash0's form, and a crate the CLI makes unneeded.

### D6. The marker is a dashboard of timber's own in the dataset

A dashboard's metadata keeps no field of timber's, and the token writes
dashboards and nothing else. So the marker is a dashboard:

- Its identifier is `timber-` and the first 32 hexadecimal characters
  of the SHA-256 of `<owner>/<name>/<component>/<environment>`. The
  owner and name are the checkout's repository, as the clan push profile
  names its marker's (the clan design's D4).
- Its kind is `Dashboard`.
- `metadata.annotations["dash0.com/folder-path"]` is `/timber`.
- `spec.display.name` is `timber: <owner>/<name> <component>
  <environment>`.
- `spec.display.description` is the record, one JSON object:

```json
{ "marker": "timber/dash0/v1",
  "repository": "madswan-dev/dashboards", "component": "dashboards", "environment": "deneb",
  "index": "sha256:…", "version": "1.3.0",
  "dashboards": ["overview", "sluice"],
  "applying": { "index": "sha256:…", "version": "1.4.0" } }
```

`index` and `version` name the version last applied in full, and are
absent where none was. `dashboards` names every identifier timber may
have applied to the dataset for this environment and not removed.
`applying` is present only while an apply runs, or after one that
didn't finish. Apply writes the marker with `dash0 … apply -f
<run>/marker.yaml`, the document written as JSON. It reads it with
`dash0 … dashboards get <id> -o json`.

A run that fails as `dashboard "<id>" not found` is no marker. A marker
Dash0 holds deleted still names what timber applied. Its
`dashboards` counts for removals, and apply writes it back, which
restores it. Any other failure is unreadable with the CLI's words.

The marker sits in the dataset beside the dashboards it records, so
every asker that reads the dataset reads it, and it lasts with them (20,
119).

Not taken:

- A label or annotation on each dashboard. Dash0 drops them, and
  writing one means rewriting the repository's files.
- Each dashboard's `dash0Extensions.tags`. That needs a Perses document
  rewritten into Dash0's own kind.
- A marker on the applying host. That is the host's state, not the
  target's (119), and it is lost when another host applies.
- A view or another asset kind as the marker. The write token is the
  narrowest that writes dashboards to the dataset.
- The removals read from the version the marker names, through the
  store. An apply that stopped after creating a dashboard would leave
  that dashboard in no version the marker names.

### D7. Apply records before it acts and after it converges

Apply follows these steps, and stops at the first that fails, answering
it:

1. Read the table and the credential (D2, D3). Read the index by digest.
   Refuse an index naming another publishing repository than the
   checkout's, or none (D6). Read each named output's file artifact by
   digest (42, rule core.4).
2. Run the dry run (D5). Nothing has reached Dash0.
3. Read the marker. *Before* is its `dashboards`, or none.
4. Write the marker with `index` and `version` as read, `dashboards`
   *before* together with the version's identifiers, and `applying`
   naming the version.
5. Apply every file with one `dash0 … apply -f <run>/dashboards`.
6. For each identifier of *before* the version doesn't hold, in order,
   run `dash0 … dashboards delete <id> --force`. A dashboard already gone
   counts as removed.
7. Write the marker with the version's `index`, `version` and
   identifiers, and no `applying`.

The marker names every dashboard before it is applied. So an apply that
stops anywhere leaves each dashboard it applied named for the next
apply's removals, and nothing timber didn't apply is ever named (rule
core.75). A failed step 7 leaves `applying`, so current says the apply
didn't finish.

### D8. current and report are read from the marker

`current` reads the marker and answers:

- the marker's `index`, where it names this repository, component and
  environment, is held undeleted, and has no `applying`;
- unread naming the dataset, where there is no marker: the dataset holds
  nothing timber applied for the environment, or it was applied
  otherwise (119);
- unread naming when Dash0 says it was deleted, where the marker is
  deleted;
- unread naming both indexes, or the one being applied over none, where
  it has `applying`;
- unread naming what it holds, where it is not this form or names
  another repository, component or environment. It is never guessed.

`report` answers the same digest with `converged_at` the marker's
`metadata.updatedAt` as Dash0 answers it, the dataset's own clock at the
write that recorded the convergence. As in the clan design, that orders
nothing: the core orders reports by the apply's `at`. A marker with no
`updatedAt` is unread.

`desired` reads the table and answers the head the core hands it (88).

### D9. Proofs

- **A scripted Dash0.** It holds dashboards and the marker in memory,
  answers each operation of the private trait (`validate`, `marker`,
  `put_marker`, `apply`, `delete`) as a test sets, and records what
  was asked. The profile's refusals, its order of steps and the target
  group run over it in `check-fast`.
- **A stand-in `dash0`.** The real trait's runs are checked in
  `check-fast` against a shell script given as the program. The script
  records its arguments, its environment and the configuration folder
  it was handed, and prints the JSON a test sets. This holds the flags,
  their order around the subcommand, the removed `DASH0_*` variables,
  the empty folder of the run's own, the token by path alone, and the
  not-found wording.
- **Against Dash0, by hand.** `crates/timber/tests/dash0.rs` sits behind
  the `tools` feature. It runs under the `dash0` task of
  `crates/timber/moon.yml`, tagged `check-dash0`, with
  `$TIMBER_TOOLS_DASH0` among its inputs. The setting is
  `<api>#<dataset>#<token file>`: a throwaway dataset, and a file
  holding a token that may write dashboards to it. A run without the
  setting fails naming it. With it, the run takes these steps:
  1. Publish three versions into a layout in the test's directory
     through the fakes' build: A with `timber-proof-a` and
     `timber-proof-b`, B with `timber-proof-a` and `timber-proof-c`,
     and C with `timber-proof-a` alone.
  2. Create `timber-proof-hand` with the CLI directly.
  3. Run the target group over the profile.
  4. Apply A, B, A and C, checking after each the dataset's dashboards,
     `current` and `report`. After B, `-b` is gone; after A again, `-c`
     is gone; and `-hand` stays throughout.
  5. Delete the marker and check that `current` is unread.
  6. Remove every dashboard the run made.

  The CLI it runs is `TIMBER_TOOLS_DASH0_CLI`, Dash0's flake at v1.20.0,
  which `devenv.nix` pins, since on deneb the `dash0` on a person's path
  is swancloud's wrapper. No gate task runs it, since the token is a
  file a person makes (rule core.38).

Not taken: a local Dash0 API in the tools layer. There is none to run,
and a stand-in server of the test's own would prove the CLI against
timber's idea of Dash0. The stand-in program already proves what timber
hands the CLI.

### D10. Records

Record 0034 holds D4 to D8, each with the path not taken, citing the
statement at aaa66ca, record 0033 and swancloud record 0062. Rule
core.75 holds D7's bound on removals.

## Risks / Trade-offs

- **The marker is visible.** One dashboard per environment shows in
  Dash0, in the folder `/timber`. → It is the record a reader needs
  with no timber running (20). Its name says whose it is.
- **Dash0 keeps a dashboard's description as given.** The marker relies
  on it. → The by-hand proof reads the marker back after each apply. If
  Dash0 rewrites the field, the marker's record moves to a Markdown
  panel of the same dashboard, and the record changes with it.
- **The CLI's wording of a missing dashboard.** A release that rewords
  `dashboard "<id>" not found` makes the read unreadable rather than
  no marker. → apply stops before anything reaches Dash0, and the
  stand-in and the proof pin the wording at the release the tree pins.
- **A dashboard edited in Dash0's UI.** It lasts until the next apply
  replaces it. → The repository is the dashboards' one source (swancloud
  record 0062).
- **An environment moved to another dataset.** The old dataset keeps
  its dashboards and marker. → Removing them is a person's act in that
  dataset, and the marker there names them.
- **Two hosts applying one environment at once.** The marker's writes
  race. → One host holds the write token, and the core starts nothing
  (rule core.52).
- **The wrong `dash0` on `PATH`.** → Dash0 refuses a write with a
  read-only token, and apply fails with Dash0's words. It never writes
  with another token.

## Migration Plan

Nothing published changes. A bindings file with no `[credentials]`
reads as before. The Nix profiles answer as before through the family
module. `madswan-dev/dashboards` adopts the target in its own unit:
its policy file declares the component, its outputs and a `target.dash0`
environment, and the host's `[credentials]` names `dash0-dashboards`.
Rollback is a promotion back and an apply, which removes what the
newer version added (34).
