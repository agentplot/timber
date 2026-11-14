# timber

**Phase 1 is built: v0.6.0 holds its six bolts.** It publishes a
landing as a signed version in a store you own, replays what the
landing's checks built from the same store, accepts and promotes
through a gate and a signed ledger, serves npm and Cargo from it under a
policy, and pushes a version to a Nix fleet. The design is settled in
[boomdrivedev/blueprints](https://github.com/boomdrivedev/blueprints); the
[roadmap](https://boomdrivedev.github.io/timber/roadmap/) says what each
bolt delivered and what phases 2 and 3 add.

**Build once, sign everything, promote with proof, in your own account.**

timber delivers what the merge queue lands. After
[sluice](https://github.com/agentplot/sluice) lands a change, timber
versions, packages, signs and pushes what the build left as OCI artifacts
into a registry you own: ECR, GHCR, Harbor, zot, or a file-based OCI
layout on disk. To each artifact's digest it attaches attestations as OCI
referrers: the build's provenance, sluice's signed check verdicts for the
landed commit, and the test results. The same content-addressed store is
a build cache, an artifact repository and a release system, and the bytes
that passed the gate are the bytes that run. Your artifacts, in your
account.

- **The site:** https://boomdrivedev.github.io/timber
- **The design book:** https://boomdrivedev.github.io/blueprints/timber/
- **The statement**, timber's invariants as numbered clauses, is
  `design/timber/requirements.md` in boomdrivedev/blueprints.

## What it does

- **publish.** Offers a landed revision's outputs as a version of each
  component it publishes: the version derived from history and claimed
  before anything is packed, each artifact with its provenance beside
  it, the index last, and the host's gate told where its bindings name
  one. Where the landing has verdicts it builds nothing: a host that
  holds none of the outputs offers the cache entries they name.
- **verify.** Says which signer the policy names made each claim on a
  version, and whether each counts.
- **push.** Writes the cache entries a landing's build left on this
  host to the store after the landing; with `--left`, those a check
  job's run left, before it.
- **sync.** Renders into a checkout the cache entries a landed
  revision's verdicts name, where moon reads them, and each layout of
  the versions its lockfiles pin that the policy admits, refusing the
  rest with the reason.
- **serve.** The host's one serving process, at one fixed address: npm
  and Cargo read the checkout's layouts from it, and every repository
  the host knows is a page with its JSON beside it.
- **promote.** Moves a channel such as `stable` to a version by a
  signed claim appended to a chain held in a git ref or in the store; a
  rollback is a promotion back.
- **apply.** Converges one environment on its channel's head, the
  claims the channel's policy requires judged first: a Nix binary cache
  through attic, or one machine of a clan fleet, whose report is a
  claim beside the version.
- **status.** Where each channel is and the chain that put it there,
  and what each environment runs and should run, read on any machine.
- **policy.** The policy in force at the branch's landed head.
- **rebuild.** Makes the host's record of what it published again from
  the store and git.

Beside the command line, a Claude Code plugin refuses an ecosystem's
own publish command in a timber repository and names `timber publish`,
and a herdr pane shows every repository the host knows.

One store at three levels of trust: a cache hit says someone built it, an
artifact says the merge queue built and signed it, and a release says it
was promoted with proof. Environments subscribe to a channel and a
reconciler converges them, push or pull. Trust is centralized only as a
thin layer of identity, policy and a signed ledger; execution stays on
the machines the agents run on.

timber is written in Rust, as sluice is.

timber is one of a family: headwater reports releases, eddy lists
meetings, millpond keeps the inbox, flume launches the work, sounding
works a question, sluice lands each change, and
[Boomdrive](https://github.com/agentplot/boomdrive-web) runs the teams
that make them. Headwater and eddy feed millpond, millpond feeds flume,
flume's agents submit to sluice, sluice lands, and timber delivers to
the environments. timber reads sluice's refs and moon's task hashes;
neither of them knows timber exists.

## This repository

| path | what |
|---|---|
| `crates/` | timber's five crates: the command line, the core, the seams, the fakes and the conformance suite |
| `plugins/` | the Claude Code plugin and the herdr pane |
| `openspec/specs/` | what each built part must do, changed only through an OpenSpec change |
| `site/` | the site, Astro with Starlight (`site/README.md`) |
| `docs/architecture/core.md` | the rules every change here holds to |
| `docs/adr/` | the decisions behind the rules, one record each |
| `devenv.nix` | the shell: moon through proto, bun, and Playwright with its Chromium |
| `.config/kit.toml` | the kit standard's profile this repository follows |
| `set-version.sh` | writes a release's version into every file that carries timber's |
| `release-commit.sh` | makes a release's commit, one id wherever it runs |

```sh
devenv shell -- moon run site:build   # the site, with every internal link checked
devenv shell -- site-dev              # http://localhost:4321/timber/
```

CI releases each push to main that git-cliff finds a new version in.
main's tree names `0.0.0`. A release's tag names a release commit, cut
from the pushed commit, that writes the version into every file that
carries timber's (`release-commit.sh`, `set-version.sh`), so a build of
the tag says that version with nothing given (record 0031).
`devenv shell -- release-preview` shows the next release.

## License

[MIT](LICENSE)
