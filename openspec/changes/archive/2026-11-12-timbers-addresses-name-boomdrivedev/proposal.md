# timber's addresses name boomdrivedev

## Why

blueprints, crew, flume, timber and weir moved from the agentplot owner
on GitHub to boomdrivedev. Their sites moved with them: the timber site
answers at https://boomdrivedev.github.io/timber and the design book at
https://boomdrivedev.github.io/blueprints/timber/, while
https://agentplot.github.io/timber answers 404 (both read on
2026-11-12). sluice, boomdrive, boomdrive-web, boomdrive-cloud,
site-theme and boom stay agentplot's.

Nine of timber's specs still cite the statement at `agentplot/blueprints`
and one tells a host to add the marketplace `agentplot/timber`. Each
commit they cite is the same commit in boomdrivedev/blueprints, so a
citation moves its owner and keeps its revision.

## What Changes

- Every requirement that cites the statement at `agentplot/blueprints`
  cites it at `boomdrivedev/blueprints`, at the same revision:
  channel-movement, clause-coverage, command-line, git-ref-ledger,
  policy-file, promotion, seam-contracts and store-ledger. No clause,
  scenario or behaviour changes.
- The Claude Code plugin's marketplace is added as
  `/plugin marketplace add boomdrivedev/timber` (claude-code-plugin).
- The suite reads the statement at `boomdrivedev/blueprints 90cf023`
  (`crates/timber-conformance/src/statement.rs`), which clause-coverage's
  scenarios name.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `channel-movement`, `clause-coverage`, `command-line`,
  `git-ref-ledger`, `policy-file`, `promotion`, `seam-contracts`,
  `store-ledger`: the statement's citations name boomdrivedev/blueprints.
- `claude-code-plugin`: the marketplace is boomdrivedev/timber.

## Impact

- The specs' fixtures, a checkout of `agentplot/consumer` or of
  `agentplot/timber` at `example.com` and the paths the serving process
  answers for them, name no tool's address and stay as written.
- The names timber writes and reads that say agentplot stay, as record
  0032 decides: the claim kinds under
  `https://agentplot.github.io/timber/claim/`, the media types and
  annotation keys, the herdr plugin's id and the Claude Code marketplace's
  name.
- release-version's Purpose cites `agentplot/blueprints` principle 14.
  A delta reaches a requirement and no Purpose, so that line stays until
  a change that rewrites the capability's Purpose.
