---
number: 32
title: The addresses of the moved tools name boomdrivedev, and the names timber writes stay
status: deprecated
date: 2026-11-12
---

# The addresses of the moved tools name boomdrivedev, and the names timber writes stay

Status: deprecated on 2026-11-13. It holds a one-off move, which is the plan's and no record's (crew's rule briefs.15 as amended 2026-11-13).
Sources: the brief; `gh repo list boomdrivedev` and `gh repo list agentplot`, read on 2026-11-12; the sites' answers on 2026-11-12; record 0025, whose rename is the precedent; afterthought/kit-standard `docs/orgs/`; the OpenSpec change timbers-addresses-name-boomdrivedev.

## Context and problem statement

blueprints, crew, flume, timber and weir moved from the agentplot owner on GitHub to boomdrivedev. sluice, boomdrive, boomdrive-web, boomdrive-cloud, site-theme, strategy, boom and the archived flywheel stay agentplot's. The repositories' pages moved with them: on 2026-11-12 https://boomdrivedev.github.io/timber/ and https://boomdrivedev.github.io/blueprints/timber/ answered 200 and https://agentplot.github.io/timber/ answered 404. A commit cited in agentplot/blueprints is the same commit in boomdrivedev/blueprints.

The tree said agentplot in four kinds of place: addresses of the moved tools; addresses of the tools that stay; names timber writes, reads or is installed by, which take agentplot as a namespace; and fixtures, the made-up checkouts the tests and specs use. Four questions are left to the fix:

1. Do the names that take agentplot as a namespace move?
2. Which records move their citations?
3. Do the fixtures move?
4. What of a spec's Purpose, which no delta reaches?

## Considered options

For the names, which are the claim kinds under `https://agentplot.github.io/timber/claim/`, the media types `application/vnd.agentplot.timber.*`, the annotation keys `io.github.agentplot.timber.*`, the herdr plugin's id `agentplot.timber` and its state directory, the Claude Code marketplace's name `agentplot`, the plugin's author, the kit standard's org and the license's holder:

1. Keep them.
2. Move them to boomdrivedev, as record 0025 moved the names it renamed.

For the records:

1. Move the citations in `docs/adr/`, and keep the archived OpenSpec changes as written.
2. Move every record, the archived changes too.
3. Move none.

For the fixtures, such as a checkout of `agentplot/consumer` at `example.com` and the paths the serving process answers for it:

1. Keep them.
2. Move them with the addresses.

## Decision outcome

The names stay, option 1. A name is no address: no claim kind's URI has ever answered a page (`https://boomdrivedev.github.io/timber/claim/report/v1` answers 404 as the old one did), and the brief moves addresses. Record 0025 moved names only once it found nothing held under the old ones. timber has released since, and this fix could not look into every store, ledger and host that may hold a claim, an index or a plugin under these names, so option 2 would risk claims no verdict counts and plugins installed twice. The kit standard knows an org `agentplot` and none named `boomdrivedev`, so `.config/kit.toml` keeps naming the profile it reads.

The records are option 1. An ADR is read for its citations, which a reader follows, and the moved sites answer only under boomdrivedev, so the citations move at the revisions they name. An archived change is the change as it was proposed and archived; record 0025's rename left the archive as written, and this fix does the same.

The fixtures stay, option 1. They address no tool: the specs and tests name a checkout at `example.com` and the serving address that checkout gets, and moving them would change specs and tests for no reader.

A spec's Purpose stays as written. OpenSpec's deltas reach requirements and no Purpose, so release-version's Purpose cites `agentplot/blueprints` principle 14 until a change rewrites that capability's Purpose.

### Consequences

- The README, CLAUDE.md, `openspec/config.yaml`, the rules' narrative and their citations, the crates' module comments, `cliff.toml`, the flake's header and homepage, the pages workflow, the site and both plugins' manifests and READMEs name boomdrivedev/timber, boomdrivedev/blueprints and their sites. The suite reads the statement at `boomdrivedev/blueprints 90cf023`.
- sluice's release in `devenv.nix`, its run kind `https://agentplot.github.io/sluice/run/v1`, `@agentplot/site-theme` and the links to agentplot/boomdrive and agentplot/boomdrive-web stay, those tools having stayed.
- A host installs the plugins from boomdrivedev/timber and finds them under the names it had: `timber@agentplot` and `agentplot.timber`.
- `openspec/config.yaml` keeps the local path of one host's checkout of blueprints, whose folder name is the host's.
- No rule changes: this record moves addresses once and binds no later diff.
