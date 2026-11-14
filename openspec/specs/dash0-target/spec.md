# dash0-target Specification

## Purpose
The target profile bound as `dash0` (B.7, C.12): one dataset of Dash0
converged on the dashboards an environment's version holds, each by its
fixed identifier, with Dash0's own CLI from the host that holds the
dataset's write token, the token handed by its path; every dashboard an
earlier version applied that this one does not hold removed, and none
timber did not apply; and what timber applied recorded in a marker
dashboard in the dataset, which current and report read.

## Requirements

### Requirement: The Dash0 target is a target profile of its own

The Dash0 target SHALL implement the target seam for one dataset of
Dash0, pushed to from the host that holds its credential (B.7, C.12).
It SHALL be bound under the profile name `dash0`, among every target
profile the binding holds. It SHALL be reached through the target seam
alone, and SHALL name Dash0 and its CLI only under `profiles/target/`
(rules core.1, core.2).

#### Scenario: An environment names the Dash0 target

- **WHEN** the environment `deneb` writes `target.dash0 = { api = "https://api.us-east-1.aws.dash0.com", dataset = "deneb", credential = "dash0-dashboards", dashboards = ["dashboards"] }`
- **THEN** apply reaches the Dash0 target with `deneb`'s settings

### Requirement: The table names the API, the dataset, the credential and the dashboards

The profile's table SHALL hold four required keys (C.10, C.12): `api`,
`https://` followed by a host; `dataset`, non-empty with no whitespace,
no `/` and no leading `-`; `credential`, a non-empty name with no
whitespace; and `dashboards`, a non-empty list of the component's output
names, each once. A missing key or one of another form SHALL make every
operation unreadable naming the environment and the key, with nothing
run.

#### Scenario: An address in the clear

- **WHEN** the table names `api = "http://api.us-east-1.aws.dash0.com"`
- **THEN** apply is unreadable naming `api`, and the CLI is not run

#### Scenario: No dashboards named

- **WHEN** the table names `dashboards = []`
- **THEN** apply is unreadable naming `dashboards`, and the CLI is not run

### Requirement: The table reads no other key

Any key of the table but the four SHALL make desired, current, apply
and report unreadable, naming the key and the four the table reads,
with nothing run and the key's value in no words. No token, path or
profile SHALL be read from the table (45, rule core.21).

#### Scenario: A token in the table

- **WHEN** the table names its four keys and `token = "auth_abc"`
- **THEN** apply is unreadable naming `token` and the four keys, its words hold no `auth_abc`, and the CLI is not run

### Requirement: The credential is the host's, handed by its path

The table's `credential` SHALL name a credential of the host's
`[credentials]` (118). current, apply and report SHALL hand the CLI the
path it names as `--auth-token-file`, and timber SHALL never open, copy
or pass the credential's contents (64, rule core.21). A credential the
host's file does not name SHALL make them unreadable, naming the
credential and the bindings file, with nothing run. desired SHALL read
no credential.

#### Scenario: A credential the host does not name

- **WHEN** the table names `credential = "dash0-dashboards"` and the bindings file's `[credentials]` names no `dash0-dashboards`
- **THEN** apply is unreadable naming `dash0-dashboards` and the bindings file's path, and the CLI is not run

#### Scenario: The token goes by its path

- **WHEN** `[credentials]` maps `dash0-dashboards` to a file, and apply runs
- **THEN** every run that reaches Dash0's API is handed `--auth-token-file` with that path, and no argument or variable of any run holds the file's contents

### Requirement: Every run of the CLI takes its settings as flags

Every run SHALL be `dash0` as found on `PATH`, given `--agent-mode`,
`--no-skill-hint` and `--config-dir` before its subcommand. A run that
reaches Dash0's API SHALL also be given `--auth-token-file` before its
subcommand, and the table's `--api-url` and `--dataset` after it
(record 0033).

#### Scenario: Flags around the subcommand

- **WHEN** current runs against a stand-in `dash0`
- **THEN** it runs `dash0 --agent-mode --no-skill-hint --config-dir <dir> --auth-token-file <path> dashboards get <marker> -o json --api-url <api> --dataset <dataset>`

### Requirement: No run of the CLI reads the host's own settings

Every variable whose name begins `DASH0_` SHALL be removed from each
run's environment. Each run's `--config-dir` SHALL name an empty folder
the operation made for it alone and removes when it ends, so no profile
of the host's own `dash0` is read (record 0033, rule core.21).

#### Scenario: The host's own settings are not read

- **WHEN** timber's environment sets `DASH0_AUTH_TOKEN`, `DASH0_PROFILE` and `DASH0_CONFIG_DIR`, and apply runs against a stand-in `dash0`
- **THEN** no run sees any `DASH0_` variable, and each is handed a `--config-dir` that is empty and is not `~/.dash0`

### Requirement: desired is the channel's head

The profile SHALL answer desired with the head the core hands it, once
its table reads, and absent where the core's head is absent (31, 88).

#### Scenario: An empty channel

- **WHEN** desired is asked of an environment whose channel's head the core answers absent
- **THEN** desired is absent, and the CLI is not run

### Requirement: Apply reads the version's dashboard files by digest

apply SHALL read the verified version's index, and the file artifact
of each output the table's `dashboards` names, through the store by
digest (42, rule core.4). An index naming another publishing repository
than the checkout's, or none, SHALL be refused naming both. An output
the index names no file artifact of SHALL be refused naming the output
and the version. Each SHALL be answered before the CLI runs.

#### Scenario: An output with no artifact

- **WHEN** the table names `dashboards = ["dashboards"]` and the version's index names no file artifact of `dashboards`
- **THEN** apply is refused naming `dashboards` and the version, and the CLI is not run

#### Scenario: Another repository's version

- **WHEN** the index names the publishing repository `someone/else` and the checkout is `madswan-dev/dashboards`
- **THEN** apply is refused naming both, and the CLI is not run

### Requirement: Apply reads the version's identifiers from the CLI's dry run

apply SHALL write each named output's file into the run's folder and
read every document's kind and identifier from one dry run of the CLI,
handed no token, address or dataset, before any run reaches Dash0's
API. A dry run the CLI refuses SHALL be answered in its own words (59).

#### Scenario: A dry run the CLI refuses

- **WHEN** the dry run exits refusing a document that is no YAML
- **THEN** apply carries the CLI's words, and no run reaches Dash0's API

### Requirement: Apply refuses anything but dashboards with identifiers of their own

apply SHALL refuse the version, naming the output and the document,
with no run reaching Dash0's API, where a document is neither a
`Dashboard` nor a `PersesDashboard`, a dashboard has no identifier, two
documents give one identifier, a document gives the environment's
marker's identifier, or an output's file holds no dashboard.

#### Scenario: A check rule among the dashboards

- **WHEN** an output's file holds a dashboard and a `CheckRule`
- **THEN** apply is refused naming the output and `CheckRule`, and no run reaches Dash0's API

#### Scenario: A dashboard with no identifier

- **WHEN** an output's file holds a `PersesDashboard` with no `dash0.com/id` label
- **THEN** apply is refused naming the output and the dashboard, and no run reaches Dash0's API

#### Scenario: One identifier twice

- **WHEN** two outputs' files each hold a dashboard with the identifier `overview`
- **THEN** apply is refused naming `overview` and both outputs, and no run reaches Dash0's API

### Requirement: The marker is a dashboard of timber's own in the dataset

What timber applied to a dataset for an environment SHALL be recorded in
one dashboard of timber's own there, the marker (119): of kind
`Dashboard`, in the folder `/timber`, its display name naming the
repository, component and environment. Its identifier SHALL be
`timber-` followed by the first 32 hexadecimal characters of the
SHA-256 of `<owner>/<name>/<component>/<environment>`, the owner and
name the checkout's repository.

#### Scenario: The marker's identifier

- **WHEN** apply runs for the environment `deneb` of the component `dashboards` in the checkout of `madswan-dev/dashboards`
- **THEN** the marker it writes has the identifier `timber-` and the first 32 hexadecimal characters of the SHA-256 of `madswan-dev/dashboards/dashboards/deneb`

### Requirement: The marker's description is its record

The marker's description SHALL be one JSON object of the form
`timber/dash0/v1`. It names the repository, the component and the
environment; the index and version last applied in full, where there
is one; every identifier timber may have applied there and not
removed; and the index and version of an apply not finished, where
there is one.

#### Scenario: A record read back

- **WHEN** a marker written naming the index of `1.3.0`, `overview` and `sluice` is read back from Dash0
- **THEN** the record names the same index, version and identifiers, and no apply in progress

### Requirement: Apply records every dashboard before it applies one

After the dry run and before any dashboard is applied, apply SHALL read
the marker and write it naming the identifiers it named together with
every identifier of the version, the version being applied, and the
version it named as applied in full. apply SHALL then apply every file
with one `dash0 apply` of the run's folder (90).

#### Scenario: An apply that stops after the dashboards

- **WHEN** a version adds the dashboard `new`, and the run that deletes removed dashboards fails
- **THEN** the marker names `new` and the version being applied, and the next apply of a version without `new` removes it

### Requirement: Apply removes only what the marker names and the version does not hold

Once the files are applied, apply SHALL delete, by `dashboards delete
<id> --force`, each dashboard the marker named before this apply that
the version does not hold, and no other (C.12, rule core.75). A marker
Dash0 holds deleted SHALL still name what timber applied. Where there is
no marker, apply SHALL remove nothing.

#### Scenario: A dashboard taken out of the repository

- **WHEN** the marker names `overview` and `sluice` as applied, and the version holds `overview` alone
- **THEN** apply deletes `sluice` and no other dashboard

#### Scenario: A dashboard made by hand

- **WHEN** the dataset holds `mine`, made in Dash0's UI, and the marker does not name it
- **THEN** no apply deletes `mine`

#### Scenario: No marker

- **WHEN** the dataset holds no marker for the environment, and apply runs
- **THEN** apply deletes no dashboard

### Requirement: Apply records the version once the dataset converged

Once every removal has run, apply SHALL write the marker naming the
version's index, its version and its identifiers alone, and no apply in
progress. A failure of that write SHALL be apply's answer, the marker
still naming the apply in progress.

#### Scenario: A converged apply

- **WHEN** apply of version `1.4.0` holding `overview` converges
- **THEN** the marker names the index of `1.4.0`, `1.4.0`, `overview` alone, and no apply in progress

### Requirement: current is the version the marker names

current SHALL read the marker and answer the index it names as applied
in full, where the marker is held undeleted, is of the form
`timber/dash0/v1`, names the checkout's repository and the
environment's component and name, and names no apply in progress (89).

#### Scenario: After a converged apply

- **WHEN** the marker names the index `A` as applied in full and no apply in progress
- **THEN** current answers `A`

### Requirement: current is unread where the marker does not say

current SHALL answer unread with the reason, never a digest, where
there is no marker, naming the dataset; where it is deleted, naming
when Dash0 says it was; where it names an apply in progress, naming
its index and the one before or none; and where it is of another form
or names another repository, component or environment, saying what it
holds (119).

#### Scenario: A dataset timber never applied to

- **WHEN** Dash0 answers `dashboard "<marker>" not found` for the marker
- **THEN** current is unread naming the dataset, and is no digest

#### Scenario: An apply that did not finish

- **WHEN** the marker names the index `B` in progress over `A`
- **THEN** current is unread naming `B` and `A`

#### Scenario: A deleted marker

- **WHEN** Dash0 answers the marker with a `dash0.com/deleted-at` annotation
- **THEN** current is unread naming when it was deleted, and the next apply writes the marker back

### Requirement: report is the marker's version and when Dash0 recorded it

report SHALL answer the digest current answers, with `converged_at` the
marker's `metadata.updatedAt` as Dash0 answers it (91). A marker with
no `updatedAt` SHALL be unread. Where current is unread, report SHALL be
unread with the same reason (119).

#### Scenario: Reported after a converged apply

- **WHEN** apply converges and Dash0 answers the marker with `updatedAt` `2026-11-14T10:00:00Z`
- **THEN** report answers the applied index and `2026-11-14T10:00:00Z`

### Requirement: Dash0's words are the answer

A run of the CLI that exits other than as its operation allows SHALL
make that operation unreadable, naming the command run and carrying
the CLI's stderr verbatim, and nothing after it SHALL run (59, rule
core.19).

#### Scenario: A token that may not write

- **WHEN** Dash0 answers the marker's write with `access denied; check your permissions`
- **THEN** apply is unreadable carrying those words, and no dashboard is applied

### Requirement: A version's files are read in one family module

The reading of a verified version's index, of an output's file artifact
in it, and of that artifact's bytes by digest SHALL be one module every
target profile shares, `profiles/target/file_artifact.rs`, each answer
naming the artifact in the asking profile's words. The Nix profiles
SHALL read through it and answer as they answer today, and no profile
SHALL call another (rule core.2).

#### Scenario: The Nix profiles' answers stand

- **WHEN** the Nix cache's and the clan push profile's tests run
- **THEN** each passes as it is, every answer naming a closure file as before

### Requirement: The profile passes the target group

The Dash0 target SHALL pass the conformance suite's target group,
declaring that it holds a marker, in its own tests against a scripted
Dash0 that answers each operation of the profile as the CLI would, in
`check-fast` (93, rule core.33).

#### Scenario: The target group over a scripted Dash0

- **WHEN** the Dash0 target's tests run the target group over the scripted Dash0
- **THEN** every scenario of the group that applies to a target holding a marker passes

### Requirement: Against Dash0, by hand

A test under its own tag, `check-dash0`, SHALL run the target group and
applies of three versions against a dataset the operator names in
`TIMBER_TOOLS_DASH0`, with the CLI the tree pins, checking that a
dashboard a later version dropped is gone and one made by hand stays.
It SHALL refuse to run without the setting, naming it, and no gate task
SHALL run it, since its token is a file a person makes (rule core.38).

#### Scenario: Run without the setting

- **WHEN** `moon run cli:dash0` runs with `TIMBER_TOOLS_DASH0` unset
- **THEN** it fails naming `TIMBER_TOOLS_DASH0`, and nothing reaches Dash0
