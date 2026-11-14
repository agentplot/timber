# Verify: timber deploys a dataset's Dash0 dashboards

Checked on `change/timber-deploys-a-datasets-dash0-dashboards` at
`0bac0f7e`, cut from `bolt/timber-deploys-dash0-dashboards`. The code
stage archived the change at that commit, as task 6.1 of the plan asks,
so this report sits in the archived change's directory.

**Verdict: the whole checks pass and the work holds.** Every task is
ticked, and every requirement and scenario of the three deltas is met in
the code and exercised by a test that ran green in `check-fast`. The
exceptions are the two host-bindings scenarios noted below, which the
code meets and which are tested in two halves.

## The whole checks

The layers are the ones `CLAUDE.md` names, each run once inside
`devenv shell` on `0bac0f7e`, with a clean tree:

| layer | command | outcome |
|---|---|---|
| `check-fast` | `moon run ':#check-fast'` | passed, exit 0: 23 tasks completed, 16 replayed from moon's cache of identical inputs, in 58s |
| `check-tools` | `moon run ':#check-tools'` | passed, exit 0: `cli:tools`, 1 task, in 3m 5s |

`cli:test` ran live. It passed 675 unit tests with 1 ignored,
`profiles::target::dash0::cli::tests::a_child_runs_every_operation`. That
test is ignored by design: `the_hosts_own_dash0_settings_reach_no_run`
runs it in a child process that holds the `DASH0_*` variables. The run
included 46 `dash0::` tests, the four `credentials_` tests and the
`file_artifact` tests.

A first run of the two layers was stopped and its result discarded. A
command of the verify stage's own detached the worktree's HEAD at
`3833781b` for some seconds while `check-fast` was running. The branch
was restored at `0bac0f7e`, and the layers above are the one run made on
a tree that held still.

`check-dash0`, the proof against a live dataset, was not run. It needs a
token file a person makes and runs outside the gate (rule core.38). The
plan leaves it to the operator before the change lands.

The gate does not validate OpenSpec. Run by hand,
`openspec validate --type spec --strict` passes `dash0-target`. It fails
`host-bindings` (9 warnings) and `conformance-suite` (7 warnings), and
the same specs at `3833781b`, before this change, give the same counts:
the warnings are long requirements that were there already, and this
change adds none.

## Tasks

All 15 tasks, 1.1 to 6.1, are ticked, and the tree holds what each one
names. `file_artifact.rs` is the family module. `[credentials]` is read
into `Credentials` and handed to `profiles::target::bound`. The
`dash0/` profile has `table`, `marker`, `cli`, `scripted` and `tests`,
with the `reads`, `refusals`, `applies` and `group` submodules. Dash0's
CLI is pinned at `github:dash0hq/dash0-cli/v1.20.0` in `devenv.yaml`,
with `TIMBER_TOOLS_DASH0_CLI` set in `devenv.nix`.
`crates/timber/tests/dash0.rs` runs behind `tools`, and the `cli:dash0`
task in `crates/timber/moon.yml` is tagged `check-dash0` alone. The
site's targets page, integrations index and self-hosting guide document
the target. `openspec/specs/` holds `dash0-target` and both deltas.

## dash0-target

| requirement | where it holds | tests |
|---|---|---|
| A target profile of its own | `profiles/target/mod.rs` binds `dash0`; Dash0 is named only under `profiles/target/` | `bindings::tests::an_apply_binds_every_target_built_through_a_store_it_never_makes` (`["clan", "dash0", "nix_cache"]`) |
| The table names the API, the dataset, the credential and the dashboards | `dash0/table.rs`, read first by every operation | `table::tests::*`: an address in the clear, empty or repeated `dashboards`, each key missing |
| The table reads no other key | `Table::read`, the value kept out of the words | `a_token_in_the_table_is_unreadable_naming_it_and_the_four_and_never_its_value`; `group`'s `foreign_settings` |
| The credential is the host's, handed by its path | `Dash0::token`; `cli::args` passes `--auth-token-file` | `reads::a_credential_the_host_does_not_name_…` (current, report, apply; desired reads none); `cli::tests::the_token_goes_by_its_path_…` |
| Every run takes its settings as flags | `Cli::args` | `cli::tests::the_marker_is_read_with_each_setting_as_a_flag_around_the_subcommand`, `…written_and_the_dashboards_applied_and_deleted_each_with_its_settings` |
| No run reads the host's own settings | `DASH0_*` removed by `run::Tool`'s prefix match; an empty `config/` in a folder made and removed per operation | `cli::tests::the_hosts_own_dash0_settings_reach_no_run` |
| desired is the channel's head | `Target::desired` | `reads::desired_answers_the_head_and_an_empty_channels_absence_and_nothing_runs` |
| Apply reads the dashboard files by digest | `Dash0::files` through `file_artifact` | `refusals::an_output_with_no_artifact_…`, `another_repositorys_index_or_one_naming_none_…` |
| Identifiers come from the CLI's dry run | `Cli::validate`, with no token, address or dataset | `cli::tests::a_dry_run_is_handed_the_files_and_no_token_address_or_dataset`; `refusals::a_dry_run_the_cli_refuses_…` |
| Only dashboards with identifiers of their own | `Dash0::identifiers` | `refusals::` check rule, no identifier, one identifier in two outputs, the marker's identifier, an output of no dashboard |
| The marker is timber's own dashboard | `marker.rs` `Marker::id`, `document` | `marker::tests::the_identifier_is_the_hash_…`, `the_document_is_a_dashboard_in_the_timber_folder_…` |
| The marker's description is its record | `Record`, `Marker::read` | `marker::tests::a_record_written_is_read_back_…`, `a_record_in_progress_…`, `…another_form_or_naming_another_environment_…` |
| Every dashboard is recorded before one is applied | `apply`: marker read, written in progress, one `dash0 apply` | `applies::an_apply_that_stopped_after_its_first_write_…` |
| Only what the marker names and the version lacks is removed | `apply`'s delete loop over `named` | `applies::a_version_dropping_sluice_deletes_it_alone_and_never_one_made_by_hand`, `a_first_apply_with_no_marker_removes_nothing_…`, `a_deleted_markers_dashboards_are_removed_…` |
| The version is recorded once the dataset converged | The last `put_marker` | `applies::a_first_apply_…`, `a_failure_at_each_step_answers_dash0s_words_and_runs_nothing_after` |
| current is the version the marker names | `Dash0::running` | `reads::a_converged_marker_answers_its_index_…` |
| current is unread where the marker does not say | `Dash0::running` | `reads::` no marker, deleted marker, apply in progress, a marker of another environment |
| report gives the marker's version and its `updatedAt` | `Target::report` | `reads::a_converged_marker_…`, `a_marker_with_no_updated_at_is_unread_at_report` |
| Dash0's words are the answer | `run::Tool::failed`, the stderr carried verbatim | `cli::tests::another_failure_is_unreadable_naming_the_command_and_carrying_its_words_verbatim`; `applies::a_token_that_may_not_write_applies_no_dashboard` |
| A version's files are read in one family module | `profiles/target/file_artifact.rs`; `nix_closure.rs` calls it | `file_artifact::tests::*`; the Nix cache's and clan's tests pass unchanged |
| The profile passes the target group | `dash0::tests::group` over `Scripted`, holding a marker | `the_target_group_passes_over_the_scripted_dash0_holding_a_marker` (only "applied where no marker is held" not applicable) |
| Against Dash0, by hand | `tests/dash0.rs`; `cli:dash0` tagged `check-dash0` alone | Read, not run (see above): it refuses naming `TIMBER_TOOLS_DASH0` where unset, and applies A, B, A and C with a dashboard made by hand |

## host-bindings

| requirement | where it holds | tests |
|---|---|---|
| The file names the credentials a target's tool reads | `bindings::credential`, `~/` expanded by `path_of`; no file opened | `credentials_a_home_path_is_expanded_and_the_file_at_it_never_opened` (a mode-000 file and a missing one), `credentials_no_table_names_none_and_publish_binds_as_before` |
| A broken credentials table binds nothing | `bindings::credential` | `credentials_a_token_in_place_of_a_path_…` (apply, publish, verify, status, with no layout made), `credentials_a_relative_path_or_a_value_that_is_no_string_…` |
| The binding hands the targets the host's credentials | `bind` passes `&bindings.credentials` to `profiles::target::bound` for an apply | See the note below |

Two scenarios are tested in halves: "A credential named by its path …
and `timber apply` runs" and "An apply's targets". The bindings tests
read `[credentials]` from a file, and the Dash0 tests hand the target a
`Credentials` directly. No test binds an apply from a file holding
`[credentials]` and sees the target given that path. The wiring between
them is one argument in `bind`, read here. This is a gap in coverage, not
in behavior, and it does not hold the change back.

## conformance-suite

| requirement | where it holds | tests |
|---|---|---|
| The Dash0 target runs the target group | `dash0::tests::group` in `check-fast`; `tests/dash0.rs` under `check-dash0` | The `group` test above |

## Rules

The diff keeps the rules the proposal names. Core.1: Dash0 is named only
under `profiles/target/`. The bindings module reads `[credentials]`
generically, and names Dash0 only in doc comments, an example key and
citations of the target's design, as it already cites the Nix cache
unit's. Core.2: no profile calls another, and files are read through
`file_artifact`. Core.4: the index and files are read by digest. Core.19:
Dash0's stderr is carried verbatim, and absent, refused and unreadable
stay apart. Core.21: the token's path alone reaches the CLI, with no
`DASH0_*` variable and no `~/.dash0`. Core.38: `check-dash0` sits in no
gate layer. Core.53: a broken `[credentials]` binds nothing. Core.75:
removals are read from the marker alone. Record 0034 and rule core.75
landed with the plan at `3833781b`.

## Left for the landing

- The operator runs `moon run ':#check-dash0'` against a throwaway
  dataset named in `TIMBER_TOOLS_DASH0`, and puts its answer in the
  landing's description (the plan's follow-up, rule core.38).
- The blueprints branch `design/timber-deploys-dash0-dashboards`
  (`aaa66ca`), which the proposal cites, is pushed and landed.
