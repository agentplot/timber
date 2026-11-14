# timber publishes its own module

## Why

Clause 118, as amended at `boomdrivedev/blueprints` 573ec1a on the
branch `design/timber-deploys-dash0-dashboards`, says: "On nix-darwin
and NixOS, timber's own module, `services.timber`, writes the file from
`services.timber.settings`, one option per key, each named as the file
names it, so a host names a credential as
`services.timber.settings.credentials.<name> = "<path>"`. Boomdrive's
set takes that module from timber's flake and writes no file of
timber's." It cites boomdrive record 0023 as amended on 2026-11-13, and
boomdrive rules core.18 and core.46: each tool's own flake gives the
module that writes the tool's own file, and no module writes another
tool's.

timber's flake builds the package alone. A host configured by Nix that
wants a registry, a key service, a shared ledger or, with record 0033, a
target's credential writes `~/.config/timber/config.toml` by its own
means, outside anything of timber's. Boomdrive's set carries timber's
package by an option of its own, `services.boomdrive.timber.package`,
"until timber gives one" module of its own (boomdrive record 0023's
amendment of 2026-11-13).

sluice and Flume already give theirs, in one shape: sluice's record
0098, its `nix/` at `boomdrivedev/sluice` 7ce112b4. timber takes that
shape for its own file (CLAUDE.md, the family).

## What Changes

- **`services.timber`, for nix-darwin and NixOS.** timber's flake gives
  `darwinModules.timber` and `nixosModules.timber`, each also
  `default`, keyed `github:boomdrivedev/timber#<platform>Modules.timber`
  so a system that imports it directly and through Boomdrive's module
  declares it once. Their options are `enable`, `package` and
  `settings`, and `user` on NixOS.
- **`settings` is the host's bindings file, under its own names.** It
  is free-form TOML. The module declares every table and key timber's
  reader takes, with its TOML type: `store` (`layout`, `registry`),
  `landing` (`sluice`, `branch`, each with `remote`), `ledger`
  (`git_ref` with `remote`, `store`), `signing` (`signer`, `key`,
  `kms`), `gate` (`publish`, `job`), `trust.<signer>` and
  `credentials.<name>` (118, C.1, C.2, C.5 to C.8, C.13, C.14, C.16;
  records 0013, 0033). A key left unset is left out of the file, so the
  given binding holds (118). A table set empty, `branch = { }`, is
  written, since it binds its profile. A key the module does not
  declare passes to the file for timber to judge.
- **A credential and a key are named by their paths, outside the
  store.** A `credentials.<name>` that is no path, `/…` or `~/…`, or one
  in the Nix store, fails evaluation naming the option; so does a
  `signing.key` likewise.
  The module reads and copies no credential (45, 64, 118; rule core.21).
- **The file is rendered at evaluation and linked for the user who runs
  timber.** The activation links `~/.config/timber/config.toml` to it,
  for nix-darwin's primary user, or NixOS's `services.timber.user`. A
  file there that is no link is moved aside once.
- **The package, and nothing else.** The module puts the package in the
  system profile; a host may replace it, or set it to null. It declares
  no service, so no `timber serve` (114, rule core.36), and writes no
  other tool's file and nothing under the data directory, where timber
  makes its given key and layout itself (rule core.54).
- **timber reads what the module writes.** A unit test of the bindings
  reader reads the module's files through `bindings::read`.
- **The flake gives aarch64-darwin, aarch64-linux and x86_64-linux.**
  Its nixpkgs refuses x86_64-darwin since 26.11, so that system's package
  no longer evaluates, and no release builds one.
- **The flake is a moon project of its own, `nix`.** Its `lint`,
  `typecheck` and `test` evaluate offline against no store, with the nix
  and statix devenv.nix pins. Its format is `root:format`'s.
- The self-hosting guide and the README name the module. Record 0035
  and rule core.76 are written, and rules core.1 and core.25 name the
  flake's `nix/`.

## Capabilities

### New Capabilities

- `host-bindings-module`: timber's flake gives a nix-darwin and a NixOS
  module that writes the host's bindings file from options named as the
  file names them, holds no credential or key, writes no other tool's
  file and starts nothing of timber's.

### Modified Capabilities

None. The bindings file's form and its reading are the `host-bindings`
spec's, unchanged here.

## Impact

- New: `nix/modules/options.nix`, `toml.nix`, `darwin.nix`, `nixos.nix`,
  `activate.sh`; `nix/inputs.nix`, `nix/check.sh`, `nix/moon.yml`;
  `nix/tests/` (`default.nix`, `lib.nix`, `typecheck.nix`,
  `settings.nix`, `file.nix`, `darwin.nix`, `nixos.nix`, `activate.sh`,
  `files/*.toml`); `docs/adr/0035-…md`.
- Changed: `flake.nix` (the modules, the three systems, nix-darwin as
  the tests' input), `flake.lock`, `devenv.nix` (statix, and nix by
  path, each pinned; the flake's inputs for the checks),
  `.moon/workspace.yml` (the `nix` project), `treefmt.nix` (the rendered
  files excluded), `crates/timber/src/bindings/tests.rs` (the test that
  reads the module's files), `crates/timber/moon.yml` (those files among
  `cli:test`'s inputs).
- Docs: `site/src/content/docs/guides/self-hosting.mdx`, `README.md`,
  `docs/architecture/core.md` (core.76; core.1's details and core.25).
- Sources: clauses 45, 64, 92, 114, 118 as amended at
  `boomdrivedev/blueprints` 573ec1a, C.1, C.2, C.5 to C.8, C.13, C.14,
  C.16; boomdrive record 0023 as amended on 2026-11-13, boomdrive rules
  core.18 and core.46; sluice record 0098 and its `nix/` at 7ce112b4;
  records 0013, 0021, 0031, 0033; rules core.1, core.21, core.25,
  core.26, core.36, core.38, core.54, core.74.

### Out of scope

- **timber reading `[credentials]`.** The reader of the table, and the
  test that reads the module's `credentials.toml` through it, are the
  change `timber-deploys-a-datasets-dash0-dashboards`'s, which builds
  the Dash0 target and the table it reads (118, C.12; record 0033).
  This change writes the table as 118 gives it and holds its fixture.
- **Boomdrive's set taking the module.** Re-exporting
  `darwinModules.timber` and `nixosModules.timber` beside Flume's,
  sluice's and crew's, setting `services.timber.package` to the set's
  and `services.timber.user` to the runner's, and dropping
  `services.boomdrive.timber.package` wait on a release of timber that
  carries the module, and are `boomdrivedev/boomdrive`'s.
- **A host moving to the module.** afterthought/swancloud passes its
  hosts' bindings as `services.timber.settings` once a release carries
  it.
- **A home-manager module.** The family's modules are system modules,
  and the brief names nix-darwin and NixOS.
- **A file at a non-default `XDG_CONFIG_HOME`.** The link is made at
  `~/.config/timber/config.toml`. A user who points `XDG_CONFIG_HOME`
  elsewhere links that folder's file by hand.
