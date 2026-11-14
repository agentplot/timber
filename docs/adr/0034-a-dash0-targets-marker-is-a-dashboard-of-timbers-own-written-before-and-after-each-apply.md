---
number: 34
title: A Dash0 target's marker is a dashboard of timber's own, written before and after each apply
status: accepted
date: 2026-11-14
decision-makers:
- the construct stage of timber-deploys-a-datasets-dash0-dashboards
---

# A Dash0 target's marker is a dashboard of timber's own, written before and after each apply

Status: accepted on 2026-11-14 as the construct stage's own choice, made on the rules written and cheap to change: the statement and record 0033 fix what the Dash0 target does, and this record fixes how the profile does it.

Deciders: the construct stage of the change `timber-deploys-a-datasets-dash0-dashboards`, writing its design (D4 to D8).

Sources: `boomdrivedev/blueprints:design/timber/requirements.md` 118 and C.12, at aaa66ca on branch `design/timber-deploys-dash0-dashboards`; record 0033; rule core.21; afterthought/swancloud's record 0062 as amended on 2026-11-14; Dash0's CLI, `dash0hq/dash0-cli` v1.20.0, read at its source on 2026-11-14.

## Context and problem statement

C.12's Dash0 row says what the target does. apply runs `dash0 apply` on each dashboard file the version holds, into the dataset at the API the environment's table names. It removes each dashboard an earlier version applied that this one doesn't hold, and never one timber didn't apply. The token is the host's credential, handed by its path. current is the version the dataset's dashboards were last applied from, which apply records with them.

The row doesn't say four things: how a version holds its dashboard files, how apply learns their identifiers, where apply records what it applied, and which `dash0` runs.

Dash0's CLI settles what can be built. A dashboard keeps no label or annotation of the user's, only Dash0's own. A dry run of `apply` calls no API, and in agent mode prints each document's kind and identifier as JSON. `dashboards get` answers a deleted dashboard too, annotated `dash0.com/deleted-at`. `--api-url` and `--dataset` are each subcommand's own flags.

## Considered options

**What records what timber applied.**

1. **A dashboard of timber's own in the dataset (chosen).** One per repository, component and environment, its identifier derived from the four, in the folder `/timber`. Its description holds the record: the version last applied in full, every identifier timber may have applied there, and an apply in progress.
2. **A label or annotation on each dashboard.** Not taken: Dash0 drops them, and writing one means rewriting the repository's files.
3. **Each dashboard's tags.** Not taken: a Perses document would have to be rewritten into Dash0's own kind.
4. **A file on the applying host.** Not taken: that is the host's state and not the target's (119), and it is lost when another host applies.
5. **A view or another asset kind.** Not taken: the write token is the narrowest that writes dashboards.

**What a removal removes.**

1. **What the marker names and the version doesn't hold, the marker naming each dashboard before it is applied (chosen).** apply writes the marker in progress, applies, removes, then writes it converged. An apply that stops anywhere leaves every dashboard it applied named for the next apply's removals.
2. **What the version the marker names held, read from the store.** Not taken: a dashboard created by an apply that stopped is in no version the marker names, and would never be removed.
3. **Dash0's `apply --since <ref>`.** Not taken: it reads removals from git history, and it would remove what an earlier commit held but timber never applied.

**How apply learns the version's identifiers.**

1. **The CLI's own dry run, in agent mode, with no token (chosen).** A document that is no dashboard, a dashboard with no identifier, or one identifier given twice is refused before anything reaches Dash0.
2. **A YAML parser of timber's own.** Not taken: it is a second reading of Dash0's form.

**How a version holds its dashboard files.** The table's `dashboards` names the component's outputs whose files are dashboard files, as the Nix cache's `closures` names closure outputs. Each file holds one dashboard or several. Not taken: every `file` artifact of the version, which would hand Dash0 any file the component packs.

**Which `dash0` runs.**

1. **`dash0` as found on `PATH`, as attic and clan are (chosen).** The host runs `timber apply` in the repository's shell, so the CLI the repository pins and checks with applies, as swancloud record 0062 asks.
2. **The CLI built by timber from the revision's flake.** Not taken: it ties the target to Nix and builds a tool at every apply.
3. **A path in the host's bindings file.** Not taken: the host's CLI would then differ from the repository's.

## Decision outcome

Option 1 of each, as the change's design D4 to D8 gives them:

- Every run gets `--agent-mode`, `--no-skill-hint` and a `--config-dir` of the run's own, empty. A run that calls Dash0 also gets `--auth-token-file` before its subcommand, and `--api-url` and `--dataset` after it. Every `DASH0_*` variable is removed from its environment.
- current answers the marker's version applied in full. It is unread where there is no marker, where it is deleted, or where it names an apply in progress. report's `converged_at` is when Dash0 says the marker was last written.

Rule core.75 holds the bound on removals.

### Consequences

- Good: a dashboard made by hand in the dataset is never timber's to remove, and an interrupted apply leaves nothing timber can't later remove.
- Good: what timber applied is readable in Dash0 with no timber process running (20).
- Good: the target's identifiers and validation are the CLI's own, so timber parses no YAML.
- Bad: one dashboard per environment shows in Dash0, in `/timber`.
- Bad: the read of a missing marker rests on the CLI's wording, `dashboard "<id>" not found`. A rewording makes the read unreadable, and apply stops before anything reaches Dash0.
- Bad: the wrong `dash0` on `PATH`, deneb's wrapper with its read-only token, makes apply fail with Dash0's refusal. It never writes with another token.
