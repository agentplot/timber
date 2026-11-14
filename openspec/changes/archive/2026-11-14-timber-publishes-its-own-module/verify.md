# Verify: timber-publishes-its-own-module

This report checks the work on `change/timber-publishes-its-own-module`
at `d3df11e` against this change's proposal, design, delta spec
`host-bindings-module` and tasks. The branch was cut from
`bolt/timber-deploys-dash0-dashboards`.

**The whole checks pass, every task is ticked, and every requirement and
scenario of the delta spec is met. The change is archived.**

## The checks

Run once each, on the branch as it stood at `d3df11e`, the tree clean,
as `CLAUDE.md`'s layers name them.

| check | answer |
|---|---|
| `devenv shell -- moon run ':#check-fast'` | exit 0. 26 tasks completed, 20 of them replayed from moon's cache, which keys each by its inputs, so a replay is the answer for the tree as it stands: `nix:lint`, `nix:typecheck` and `nix:test` among them, whose inputs hold `/nix/**/*`, each replaying `nix: <check>: every case holds`; `cli:test`, whose main binary ran 621 tests with 0 failed, `bindings::tests::the_modules_files_are_read_as_written` among them; `core:test`, `conformance:test`, `conformance:coverage`, `fakes:test`, `seams:test`, `root:lint`, `root:adrs`, `root:versions`, `root:release`, `root:marketplace`, `site:build` and the plugins' five tasks. Run afresh and passed: `root:format`, `root:toolchain`, `root:hooks`, `root:compile-server`, `site:toolchain` and `site:install`. |
| `devenv shell -- moon run ':#check-tools'` | exit 0. 1 task completed, `cli:tools`, replayed from moon's cache for its inputs, which this branch has not changed since it last ran afresh; every test binary passed with 0 failed. |
| `devenv shell -- openspec validate timber-publishes-its-own-module --strict` | valid |

## The tasks

Tasks 1.1 to 4.3 are all ticked, 11 of them, and none is open. Each
one's Verify holds:

- 1.1: `flake.nix` gives aarch64-darwin, aarch64-linux and x86_64-linux
  and takes nix-darwin, following nixpkgs, as the tests' input;
  `flake.lock` locks it. `devenv.nix` pins nix and statix by assertion,
  statix on `PATH` and nix as `TIMBER_FLAKE_NIX`, and sets
  `TIMBER_FLAKE_INPUTS` from `nix/inputs.nix`. `nix/moon.yml` holds
  `nix:lint`, and `.moon/workspace.yml` lists `nix`. `nix:lint` passed.
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
  file there" and "A file there and one aside already". `nix:test`
  passed.
- 2.4: `signing.key` and each `credentials.<name>` are of
  `options.nix`'s own type `pathOnHost`, which takes a string or a Nix
  path value, and `outside` tells a Nix path value apart by
  `builtins.isPath` before anything reads it as a string, failing by an
  assertion of its own that names the option and the Nix store, which
  every user reads. "A key given as a Nix path" and "A credential given
  as a Nix path" read those words through `refused`. Record 0035 says
  the same and names `types.str` among the paths not taken. `nix:lint`,
  `nix:typecheck`, `nix:test`, `root:adrs` and `root:format` passed, and
  the change validates.
- 3.1: `nix/modules/darwin.nix`, `moduleOf` and `darwinModules.timber`
  and `default` in `flake.nix`, `darwinSystem` in `nix/tests/lib.nix`,
  and `nix/tests/darwin.nix` with its seven cases. `nix:typecheck` and
  `nix:test` passed.
- 3.2: `nix/modules/nixos.nix` with `services.timber.user` and no
  default, `nixosModules.timber` and `default`, `nixosSystem`, and
  `nix/tests/nixos.nix` with its seven cases. `nix:typecheck` and
  `nix:test` passed.
- 4.1: the self-hosting guide's "On nix-darwin and NixOS" says what the
  module writes and refuses, a Nix path value among what fails, with an
  example naming no host, fleet or customer (core.24).
  `grep -c 'services.timber' README.md` prints `2`, and the README has
  the row for `nix/`. `site:build` passed.
- 4.2: record 0035 is written and accepted on the brief. `root:adrs`
  passed.
- 4.3: `grep -c 'core\.76' docs/architecture/core.md` prints `2` and
  `grep -c 'record 0035' docs/architecture/core.md` prints `2`; core.1's
  details name the flake's modules under `nix/`, and core.25 names the
  flake's `nix/`.

## The requirements

Each requirement of `host-bindings-module` and each of its scenarios is
held by the case the scenario names, every one of them present under its
name, and every case holds:

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
  store**: its six scenarios fail by the module's assertions, each
  message naming the option and saying "the Nix store, which every user
  reads": a string under the store directory, a string that is no path,
  and a Nix path value, `is a Nix path value, which would copy its file
  into the Nix store, which every user reads`. Met. Beyond the cases,
  which evaluate `options.nix` alone, a whole system was evaluated with
  the pinned nix against the read-only dummy store, as `nix/check.sh`
  does: the nix-darwin system of `darwinSystem` with
  `settings.signing.key` a Nix path value, and the NixOS system of
  `nixosSystem` with `settings.credentials.dash0` one, each fail at
  their toplevel with `Failed assertions:` and that assertion's words
  alone, the option named, and nothing copied into the store. The
  renderer, `toml.nix`, refuses a path by its type without coercing it
  to a string.
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

## Beside the change

- `a4aa91c9`, `fix(site)`, moves `site/package.json` and `site/bun.lock`
  to `github:boomdrivedev/site-theme` at the same commit, outside the
  proposal's Impact. Without it `site:install` fails on a host without the
  old tarball cached, so task 4.1's Verify, `site:build`, needs it. It
  cites record 0032 and changes no requirement.
