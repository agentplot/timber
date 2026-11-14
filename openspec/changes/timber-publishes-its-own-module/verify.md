# Verify: timber-publishes-its-own-module

This report checks the work on `change/timber-publishes-its-own-module`
at `4d8717f` against this change's proposal, design, delta spec
`host-bindings-module` and tasks. The branch was cut from
`bolt/timber-deploys-dash0-dashboards`.

**The work falls short in one requirement, and the change is not
archived.** The whole checks pass and every task is ticked, but one
sentence of the requirement "A credential and a key are named by their
paths, outside the store" does not hold: a Nix path value fails
evaluation naming the option, and the failure does not say that the store
is read by every user. Every scenario of the delta spec is met.

## The checks

Run once each, on the branch as it stood at `4d8717f`, the tree clean, as
`CLAUDE.md`'s layers name them.

| check | answer |
|---|---|
| `devenv shell -- moon run ':#check-fast'` | exit 0. 26 tasks completed, 4 of them replayed from moon's cache, which keys each by its inputs: `nix:typecheck` and `nix:test`, each replaying `nix: <check>: every case holds`, `root:adrs` and `site:build`. Run afresh and passed: `nix:lint`, `root:format`, `root:lint`, `root:toolchain`, `root:versions`, `root:release`, `root:hooks`, `root:compile-server`, `root:marketplace`, `site:toolchain`, `site:install`, `conformance:coverage`, `conformance:test`, `core:test`, `fakes:test`, `seams:test`, the plugins' five tasks and `cli:test`, whose main binary ran 621 tests with 0 failed, `bindings::tests::the_modules_files_are_read_as_written` among them. |
| `devenv shell -- moon run ':#check-tools'` | exit 0. 1 task completed, `cli:tools`, run afresh in 2m 43s; every test binary passed with 0 failed. |
| `devenv shell -- openspec validate timber-publishes-its-own-module --strict` | valid |

## The tasks

Tasks 1.1 to 4.3 are all ticked, 10 of them, and each one's Verify holds:

- 1.1: `flake.nix` gives aarch64-darwin, aarch64-linux and x86_64-linux
  and takes nix-darwin, following nixpkgs, as the tests' input;
  `flake.lock` locks it. `devenv.nix` pins nix 2.34.8 and statix
  0.5.8-unstable-2026-07-17 by assertion, statix on `PATH` and nix as
  `TIMBER_FLAKE_NIX`, and sets `TIMBER_FLAKE_INPUTS` from
  `nix/inputs.nix`. `nix/moon.yml` holds `nix:lint`, and
  `.moon/workspace.yml` lists `nix`. `nix:lint` passed.
- 1.2: `nix/check.sh` evaluates `nix/tests` offline and read-only against
  `dummy://` with nix's configuration, caches and state in a scratch
  folder; `nix/tests/typecheck.nix` holds "Every system's package", "The
  nix-darwin module" and "The NixOS module". `nix:typecheck` passed.
- 2.1: `nix/modules/toml.nix` and `nix/modules/options.nix` are written,
  the assertions on `signing.key` and each `credentials.<name>` among
  them, and every null pruned before rendering. `nix/tests/file.nix`
  holds the thirteen named cases, and `treefmt.nix` excludes
  `nix/tests/files/**` with its reason. `nix:test` and `root:format`
  passed.
- 2.2: `the_modules_files_are_read_as_written` reads `laptop.toml` and
  `fleet.toml` through the bindings reader as the spec says, and
  `cli:test`'s inputs name `/nix/tests/files/**/*`. It passed.
- 2.3: `nix/modules/activate.sh link <file>` and `nix/tests/activate.sh`,
  run by `nix/check.sh test`, with "Nothing there", "A link there", "A
  file there", "A file there and one aside already" and a usage case.
  `nix:test` passed.
- 3.1: `nix/modules/darwin.nix`, `moduleOf` and `darwinModules.timber`
  and `default` in `flake.nix`, `darwinSystem` in `nix/tests/lib.nix`,
  and `nix/tests/darwin.nix` with its seven cases. `nix:typecheck` and
  `nix:test` passed.
- 3.2: `nix/modules/nixos.nix` with `services.timber.user` and no
  default, `nixosModules.timber` and `default`, `nixosSystem`, and
  `nix/tests/nixos.nix` with its seven cases. `nix:typecheck` and
  `nix:test` passed.
- 4.1: the self-hosting guide's "On nix-darwin and NixOS" says what the
  module writes and refuses, with an example naming no host, fleet or
  customer (core.24). `grep -c 'services.timber' README.md` prints `2`,
  and the README has the row for `nix/`. `site:build` passed.
- 4.2: record 0035 is written and accepted on the brief. `root:adrs`
  passed.
- 4.3: `grep -c 'core\.76' docs/architecture/core.md` prints `2` and
  `grep -c 'record 0035' docs/architecture/core.md` prints `2`; core.1's
  details name the flake's modules under `nix/`, and core.25 names the
  flake's `nix/`.

## The requirements

Each requirement of `host-bindings-module` and each of its scenarios is
held by the case the scenario names, and every case holds:

- **A module for nix-darwin and for NixOS**: "The nix-darwin module" and
  "The NixOS module" in `typecheck.nix`. Met.
- **A module imported twice is taken once**: each module and its
  `default` carry `github:boomdrivedev/timber#<platform>Modules.timber`;
  the system imported twice evaluates, declares `services.timber.enable`
  once and holds the link's line. Met.
- **The flake gives three systems**: "Every system's package". Met.
- **The settings are the host's bindings file under its own names**: "A
  laptop's file", "A fleet's file" and "A host's credentials", byte for
  byte. Met.
- **What is unset is left out, and a table set empty is written**: "A
  table set empty" and "A key the module does not declare". Met.
- **The file's form**: "No settings" and "A value the file cannot hold",
  whose words are read through the renderer. Met.
- **timber reads the module's files as written**:
  `the_modules_files_are_read_as_written`. Met.
- **A credential and a key are named by their paths, outside the
  store**: **not met in one sentence.** Its four scenarios that set a
  string, "A signing key in the store", "A signing key's text", "A
  credential in the store" and "A credential's text", fail by the
  module's assertions, whose messages name the option and say "the Nix
  store, which every user reads". Its two scenarios that set a Nix path
  value, "A key given as a Nix path" and "A credential given as a Nix
  path", demand only a failure naming the option, and the cases test only
  that evaluation fails. The requirement says more: "A value otherwise,
  or a Nix path value, SHALL fail evaluation, naming the option, and the
  failure SHALL say that the store is read by every user." A Nix path
  value is refused by the option's type, `types.nullOr types.str`, before
  any assertion runs. Evaluated with the pinned nix against the dummy
  store, `settings.signing.key = /…/nix/tests/settings.nix` fails with
  `A definition for option 'services.timber.settings.signing.key' is not
  of type 'null or string'`, naming the option and saying nothing of the
  store. The design's Decision on credentials chooses that refusal ("A
  Nix path value, `./dash0-token`, fails `types.str` before anything
  copies it"), so the spec's sentence and the design disagree, and the
  spec, archived as written, would state a behavior the module does not
  have.
- **The file is linked at timber's path for the user who runs timber**:
  "On nix-darwin", "A primary user with no home", "On NixOS" and "A user
  the system does not declare", each line compared as the spec writes it,
  `|| true` after each link. Met.
- **What is at timber's path already**: the four activation cases. Met.
- **The module installs the package**: "The flake's timber", "A host's
  own timber" and "No package" on each platform. Met.
- **The module adds nothing else**: "Nothing else on nix-darwin" and
  "Nothing else on NixOS"; neither module declares a user, a unit, an
  agent or a file under `/etc`, and `activate.sh` writes only the link
  and, once, the file moved aside. Met.

## What returns to the code stage

The Nix path value's refusal does not say that the store is read by every
user. Either the module's refusal of a Nix path value for
`services.timber.settings.signing.key` and each
`services.timber.settings.credentials.<name>` says so, and the two cases
"A key given as a Nix path" and "A credential given as a Nix path" read
those words, or the design stage narrows the requirement's sentence to
what the design chose: a Nix path value fails naming the option, the
store's words being the assertions' alone.

## Beside the change

- `a4aa91c9`, `fix(site)`, moves `site/package.json` and `site/bun.lock`
  to `github:boomdrivedev/site-theme` at the same commit, outside the
  proposal's Impact. Without it `site:install` fails on a host without the
  old tarball cached, so task 4.1's Verify, `site:build`, needs it. It
  cites record 0032 and changes no requirement.
