# Design

## Context

timber reads the host's bindings from
`$XDG_CONFIG_HOME/timber/config.toml`, `~/.config/timber/config.toml`
where that is unset or empty (`crates/timber/src/bindings/mod.rs`,
`path`). It reads the file at every command, and the serving process at
each request it binds for (`commands/serve/host.rs`), and never writes
it. A file that is not there binds the given profiles; a table the file
leaves out binds its given one, named `(given)` in every answer about
it; a table that is there binds exactly what it names (118; records
0013, 0021). The reader refuses a table it does not bind, naming it.

The tables, and what reads each (`bindings::read`):

| table | keys | read by |
|---|---|---|
| `[store]` | one of `layout` (a path), `registry` (`<host>/<repository>`) | `profiles/store`, C.1, C.2 |
| `[landing]` | one of `sluice`, `branch`, each a table with an optional `remote` | `profiles/landing`, C.5, C.6 |
| `[ledger]` | one of `git_ref`, a table with an optional `remote`, or `store`, an empty table | `profiles/ledger`, C.13, C.14 |
| `[signing]` | `signer` and `key` (a path), or `kms` alone (a key's ARN) | `profiles/signer::read`, C.7, C.8 |
| `[gate]` | one of `publish`, an empty table, or `job`, the job's words | `bindings::Teller::of`, 130, C.16 |
| `[trust.<signer>]` | one key, its text what the signer is verified by | `bindings::trusted`, 118 |

A path in the file is absolute or begins `~/` (`bindings::path_of`).
Clause 118 at `boomdrivedev/blueprints` 573ec1a, and record 0033, add
`[credentials]`: each credential's name, as an environment's target
table names it, and the absolute path of a file that holds only that
credential. timber's reader at this change's base does not read it yet:
the change `timber-deploys-a-datasets-dash0-dashboards`, beside this one
in the bolt `timber-deploys-dash0-dashboards`, builds it with the Dash0
target. Its design, D3, reads each value as the file's other paths are
read, absolute or beginning `~/`, and writes record 0034 and rule
core.75.

timber's `flake.nix` builds the package for four systems and gives
nothing else. Its nixpkgs, locked at 02c087f, refuses x86_64-darwin:
`nix eval .#packages.x86_64-darwin.default.drvPath` fails, read on
2026-11-14, pointing at nixpkgs' 26.11 release notes. CI's release
builds x86_64-linux, aarch64-linux and aarch64-darwin alone
(`.github/workflows/ci.yml`).

The family's modules have one shape. sluice's record 0098 and its `nix/`
at `boomdrivedev/sluice` 7ce112b4, itself Boomdrive's at 3f428cf:
`nix/modules/options.nix` holds the options, given the flake's packages;
`toml.nix` renders the file at evaluation; `darwin.nix` and `nixos.nix`
link it by `activate.sh link`; the checks are a moon project `nix` that
evaluates offline and read-only against a dummy store (`nix/check.sh`,
`nix/inputs.nix`, `nix/tests/`). Boomdrive's flake imports each tool's
module as `inputs.<tool>.<platform>Modules.<tool>`, sets its `package`
to the set's by `mkDefault`, and on NixOS sets its `user` to the
runner's (`boomdrivedev/boomdrive` `flake.nix` and
`nix/modules/tools.nix`, `nixos.nix` at 6808fbf3). This design takes
that shape for timber's file.

devenv.nix pins devenv-nixpkgs at c2f38fe, sluice's pin, whose nix is
2.34.8 and statix 0.5.8-unstable-2026-07-17. The shell carries no nix of
its own: the tools layer's Nix cache and fleet tests run the host's
(devenv.nix's comment on attic), whose `nix-store` the Nix cache target
calls by name (`profiles/target/nix_cache`).

## Goals / Non-Goals

**Goals:**

- A nix-darwin or NixOS host writes timber's bindings file as
  `services.timber.settings`, each key named as the file names it, from
  timber's own flake, with no Boomdrive.
- No credential and no key reaches the store by the module, and the
  module reads none.
- The module writes timber's file and nothing else, and starts nothing
  of timber's.
- The gate shows the module's text, its links and that timber reads
  what it writes, offline and with no store.

**Non-Goals:**

- timber reading `[credentials]`: the Dash0 change's.
- Any change to what timber reads or how.
- A home-manager module, or the Boomdrive flake taking this one.

## Decisions

### 1. The modules, the systems and the layout

`flake.nix` gives `darwinModules = rec { timber = moduleOf "darwin" ./nix/modules/darwin.nix; default = timber; }`,
and `nixosModules` likewise from `nixos.nix`. `moduleOf` is sluice's:
`{ _file = file; key = "github:boomdrivedev/timber#${platform}Modules.timber"; imports = [ (import file { inherit packages; }) ]; }`,
so a system that imports the module itself and through Boomdrive's
declares `services.timber` once. The key names `boomdrivedev`, as record
0032 has timber's addresses. Both modules import
`nix/modules/options.nix`, given `{ packages }`, the flake's own
`packages` output. Everything of the flake but `flake.nix` and
`flake.lock` is under `nix/`. `flake.nix`'s opening comment says it
gives the modules too, and keeps the two lines `root:release` reads,
`workspace = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package;`
and `inherit (workspace) version;` (record 0031, core.74).

The flake's `systems` become aarch64-darwin, aarch64-linux and
x86_64-linux, with a comment saying nixpkgs refuses x86_64-darwin since
26.11 and no release builds it. With x86_64-darwin listed, "Every
system's package" could not evaluate.

The tests take nix-darwin as an input of the flake,
`github:nix-darwin/nix-darwin` following its nixpkgs, with sluice's
comment: "The tests' nix-darwin; a host's configuration brings its
own". The modules import nothing of it. The outputs' argument set gains
`...`, since nix-darwin is no argument they use (deadnix).

Not taken: a home-manager module, which a nix-darwin or NixOS host need
not run; Boomdrive carrying timber's module, which boomdrive record 0023
rejects as its option 3.

### 2. `services.timber`'s options, named as the file names them

- `enable`: `lib.mkEnableOption "timber's bindings file, linked for the user who runs timber, and its package"`.
- `package`: `types.nullOr types.package`, default the flake's
  `packages.${pkgs.stdenv.hostPlatform.system}.default`, its
  `defaultText` naming that path. A system the flake gives no package
  for fails evaluation naming the systems it gives. `null` installs
  none.
- `settings`: `types.submodule { freeformType = (pkgs.formats.toml { }).type; options = …; }`,
  default `{ }`. Each table it declares is
  `types.nullOr (types.submodule { freeformType = toml; options = …; })`,
  and each key `types.nullOr <type>`, every one default `null`:
  - `store`: `layout` and `registry`, `types.str`.
  - `landing`: `sluice` and `branch`, each a table holding `remote`,
    `types.str`.
  - `ledger`: `git_ref`, a table holding `remote`, `types.str`; `store`,
    a table declaring nothing.
  - `signing`: `signer` and `kms`, `types.str`; `key`, a path on the
    host (Decision 4).
  - `gate`: `publish`, a table declaring nothing; `job`, `types.str`.
  - `trust`: `types.nullOr (types.attrsOf (types.attrsOf types.str))`,
    each signer by its name, a table of the one key its profile
    verifies it by.
  - `credentials`: `types.nullOr (types.attrsOf <a path on the host>)`,
    each credential by its name, the path of the file that holds it,
    absolute or under `~/` (Decision 4).

  Each description says what the key binds, what holds where the table
  is unset, and the clause it rests on, in the self-hosting guide's
  words.
- `user`, NixOS only: `types.str`, no default. The user whose
  `~/.config/timber/config.toml` is linked (Decision 6).
- `text`: internal, read-only, the file's text, which each platform's
  module writes (sluice's).

Types and nothing finer. That a seam's table names one profile, that
`kms` stands alone in `[signing]`, that a registry's name has a `/`, and
every given binding are timber's, refused or given at its read, naming
the table (118, 58, 59; records 0013, 0021). `settings` is free-form,
as RFC 42 has it: a key the module does not declare passes to the file,
and timber, which refuses a table it does not bind, judges it.

Not taken:

- **An option naming each seam's profile**, `store.profile = "layout"`
  beside `store.path`. Its name differs from the file's key, which
  boomdrive core.46 rules out.
- **The given bindings as the options' defaults.** A table the file
  names binds exactly what it names, and one it leaves out binds the
  given, which may differ per repository: the given landing is sluice
  only where the repository's `HEAD` commit holds sluice's file, and the
  given ledger is on the landing's remote (118). A module that wrote the
  given would make every table written, and change what it binds.
- **A closed set of options with timber's rules.** Every rule of the
  reader would live twice.

### 3. What is unset is left out, and an empty table set is written

Before rendering, `options.nix` takes out every null at any depth and
nothing else: unlike sluice's, it keeps a table left with no key.
`branch = { }`, `git_ref = { }`, `store = { }` and `publish = { }` each
bind their profile with no options (record 0013), and a host that sets
`landing.branch = { }` says something. Since every table option defaults
to null, a table no host set renders nothing, and `settings = { }`
renders the header alone. A list stays as written.

The renderer writes a table set empty as its header alone,
`[landing.branch]`, which TOML reads as `[landing] branch = {}`, and a
table holding only tables by theirs: `[landing]` holding `branch` writes
no `[landing]` line of its own. timber's reader takes either form
(Decision 9 shows it).

Not taken: sluice's prune unchanged, which drops `branch = { }` and so
binds the given landing in its place.

### 4. A credential and a key are named by their paths, outside the store

The store is read by every user. `options.nix` asserts, with the module
enabled:

- `settings.signing.key`, where set, and each
  `settings.credentials.<name>` begin with `/` or `~/`, as timber reads
  every path of the file (`bindings::path_of`, and the Dash0 change's
  D3 for `[credentials]`), and do not begin with
  `builtins.storeDir + "/"`.

Each failure is its own message, naming the option and saying why:
`services.timber.settings.credentials.dash0 is in the Nix store, which
every user reads: put the credential in a file on the host, readable by
the user who runs timber alone, by hand or by the host's handling of
secrets, and name its path (118)`. A value that is no path gets the same
message with "is no path, /… or ~/…", since a value that is no path may
be the secret's own text. A Nix path value, `./dash0-token`, would copy
its file into the store wherever it is read as a string, so `key` and
each credential are of a type of `options.nix`'s own, a path on the
host, which takes a string or a Nix path value, and a Nix path value
fails by its own message, told apart before anything reads it as a
string: `services.timber.settings.credentials.dash0 is a Nix path value,
which would copy its file into the Nix store, which every user reads:`
and the same advice. The renderer, which takes no path, refuses one
naming its key without copying it.

The module names the path and never reads, copies or checks the file:
the file is the host's, made by its handling of secrets, agenix's or
sops' decrypted file among them (record 0033), and timber's target hands
the path to its own tool at the moment of use (45, 64; rule core.21).
`[signing] kms` and `[trust]` hold a key's ARN and a public key, no
secret, and are asserted nothing.

Not taken: a credential's path refused unless it begins `/`, as 118's
words, "the absolute path", read alone: timber's reader takes `~/` in
every path of the file, and the module refuses only what may be a
secret's text or lies in the store, so it holds no rule of the reader's
twice (Decision 2); Boomdrive's
refusal of every key named `key`, `token`, `secret` or `password`, which
would refuse timber's own `key` path; a secrets integration of the
module's own; `types.str` refusing a Nix path value, whose failure, "is
not of type `null or string`", names the option and says nothing of the
store.

### 5. The file is rendered at evaluation

`nix/modules/toml.nix` is sluice's at 7ce112b4 with two changes: its
header line is
`# Written by timber's flake module from services.timber.settings; a change goes there.`,
and its failure names `services.timber.settings.<path>` and says
"timber's file takes strings, booleans, integers, lists of strings,
tables and lists of tables". Its rules are unchanged: top-level keys
first, then tables, each sorted by name; a null left out; a table that
holds only tables named by theirs; anything else failing with its path.
Its opening comment names sluice's file and revision, as sluice's names
Boomdrive's.

The file is `pkgs.writeText "timber-config.toml" cfg.text`. The header
is a TOML comment, which timber's reader passes over.

Why at evaluation: the gate evaluates against a dummy store and builds
nothing (Decision 8), so the text must exist at evaluation to be
compared, and one `settings` gives one text on either platform. Not
taken: importing sluice's renderer from its flake, which would make
timber's flake take sluice's as an input for one file.

### 6. The activation links the file for the user who runs timber

timber reads its bindings from the user's own home, so the module writes
into one user's home. `nix/modules/activate.sh link <file>` is sluice's
at 7ce112b4 for `$HOME/.config/timber/config.toml`, its words beginning
`timber:`:

- a link to `<file>` there already: nothing done, nothing said;
- any other link there: replaced;
- a file or folder there that is no link: moved once to
  `config.toml.before-nix`, both paths named;
- where `config.toml.before-nix` is there already: neither changed, both
  named, exit 1.

It makes `~/.config/timber` where it is missing and links the file
alone, never the folder, which may hold a host key the file names,
`~/.config/timber/signing` as the self-hosting guide writes it. It
prints usage and exits 2 on any other arguments.

- **nix-darwin**: in `system.activationScripts.postActivation`, as the
  primary user (`system.requiresPrimaryUser = [ "services.timber.enable" ]`):
  `sudo --user=<user> -- /usr/bin/env HOME=<home> /bin/sh <activate.sh> link <file> || true`.
  A primary user with no `users.users.<user>.home` fails evaluation,
  naming that option.
- **NixOS**: `system.userActivationScripts.timber`,
  `if [ "$(id -un)" = <user> ]; then <runtimeShell> <activate.sh> link <file> || true; fi`.
  A `services.timber.user` that `users.users` declares as neither a
  normal nor a system user fails evaluation, naming
  `services.timber.user` and `users.users.<user>`.

`|| true` on both: a link that fails says why and stops nothing else of
the activation. timber reads the file at its next command, and a
serving process at its next request, as after a hand edit; the module
restarts nothing.

Not taken: a file written in place, where a link names the generation
that wrote it; a timber flag or variable naming the file, which every
caller would have to pass; linking at a non-default `XDG_CONFIG_HOME`,
which the activation, outside the user's session, cannot read.

### 7. The package, and nothing else on the host

`environment.systemPackages = lib.optional (cfg.package != null) cfg.package`.
Beside the activation's line, the module adds nothing:

- no launchd agent or daemon and no systemd unit, so no `timber serve`:
  the first asker that finds none starts it (114, rule core.36;
  boomdrive core.46);
- no file under `/etc`, no user and no lingering;
- nothing under the user's data directory: the given layout and the
  given host key are timber's to make at first use, and the
  configuration directory holds nothing timber made (118, C.2, C.8; rule
  core.54);
- no file of sluice's, Flume's, Boomdrive's, the herdr plugin's or
  Claude Code's (boomdrive record 0023, core.18).

`package = null` is for a host that installs timber its own way.

### 8. The checks: a moon project `nix`

`nix/moon.yml`, `language: 'nix'`, holds three tasks, each tagged
`check-fast` and run from the workspace root:

- `lint`: `statix check flake.nix && statix check nix`;
- `typecheck`: `sh nix/check.sh typecheck`;
- `test`: `sh nix/check.sh test`.

The flake's format is `root:format`'s: treefmt's nixfmt and deadnix over
the `.nix` files, and shellcheck and shfmt over the `.sh` files.
`treefmt.nix` excludes `nix/tests/files/**`, "The module's rendered
files, compared byte for byte by the flake's cases; taplo would rewrite
them" (core.25).

`nix/check.sh` is sluice's at 7ce112b4, with three changes:

- `TIMBER_FLAKE_INPUTS` in place of `SLUICE_FLAKE_INPUTS`, and the store
  `dummy://?store=/timber-check/store`;
- it runs `"$TIMBER_FLAKE_NIX" eval …` in place of `nix eval …`, failing
  `nix: <check>: TIMBER_FLAKE_NIX is not set: the devenv shell sets it`
  where it is unset;
- no `SLUICE_VERSION` export: timber's version rule is `root:release`'s
  (record 0031), and the flake reads no variable.

It runs `"$TIMBER_FLAKE_NIX" eval --raw --offline --read-only --impure --store <dummy> --extra-experimental-features nix-command --file nix/tests <check>`,
with nix's configuration, caches and state in a scratch folder, so it
computes every path, writes none, and needs neither the network nor the
daemon. Its `test` also runs `sh nix/tests/activate.sh`. Each failure is
one line, `nix: <case>: <what>`, and every case runs.

`devenv.nix` gains, with a comment citing core.26 and record 0035:

- `flakeNix = assert pkgs.nix.version == "2.34.8"; pkgs.nix;` and
  `env.TIMBER_FLAKE_NIX = "${flakeNix}/bin/nix";`, by path and not on
  `PATH`, so the tools layer keeps the host's nix;
- `pkgs.statix`, asserted to `0.5.8-unstable-2026-07-17`, in `packages`;
- `env.TIMBER_FLAKE_INPUTS = builtins.toJSON (import ./nix/inputs.nix { }).paths;`.

`nix/inputs.nix` is sluice's at 7ce112b4, its comment naming timber's
shell and checks and `TIMBER_FLAKE_INPUTS`. It turns `flake.lock` into
the inputs `flake.nix`'s outputs take: by `builtins.fetchTree` in the
shell, and by the store paths of `TIMBER_FLAKE_INPUTS` in the check.

`nix/tests/` follows sluice's: `default.nix` (`typecheck` and `test`,
each the lines of its cases), `lib.nix` (the flake, the lock's inputs,
`outputs`, `self` by its path, `lib`, `systems`, `expect`, `failures`,
`darwinSystem`, `nixosSystem`), `typecheck.nix`, `settings.nix`,
`file.nix`, `darwin.nix`, `nixos.nix`, `activate.sh` and `files/`.

Each task's inputs are what its outcome depends on: `/flake.nix`,
`/flake.lock`, `/nix/**/*`, `/devenv.nix`, `/devenv.lock`, and, for
`typecheck`, `/Cargo.toml`, `/Cargo.lock`, `/.prototools` and
`/crates/**/*`, which the package's derivation reads.

`.moon/workspace.yml` lists `nix: 'nix'`, and its opening comment names
the flake's checks among the projects.

Rules: core.25 (a part with its own language is a project, one task per
check, its rendered files excluded by name with their reason), core.26
(nix and statix are the versions the tree pins), core.38 (no check needs
a credential or the network: the shell fetches the inputs, outside the
gate's sandbox, as it fetches every input of devenv.lock).

Not taken: `pkgs.nix` on the shell's `PATH`, as sluice's shell has it,
which would hand the tools layer's Nix cache and fleet tests a nix
client other than the host's; a build of the file or a system in the
gate, where the sandbox gives no daemon and a darwin system cannot be
built on Linux.

### 9. timber reads what the module writes

`crates/timber/src/bindings/tests.rs` gains the unit test
`the_modules_files_are_read_as_written`. For each of
`nix/tests/files/laptop.toml` and `fleet.toml`, by `include_str!`, it
writes the text as the `Host` helper's bindings file, reads it with
`Host::read`, which gives the home, and asserts what each table binds
(the spec's "Reading the module's files"). `crates/timber/moon.yml`'s
`test` task gains `/nix/tests/files/**/*` among its inputs.

Between them the two files hold every key the module declares but
`credentials`, each table's two profiles split across them, since a
seam's table names one. `nix/tests/files/credentials.toml`, the
`[credentials]` the module renders, is compared byte for byte by the
flake's cases, and read by no test of this change: timber reads no
`[credentials]` here. The test that reads it through timber's reader
comes with that reader, the Dash0 change's (Out of scope in the
proposal).

### 10. What the operator reads

- `site/src/content/docs/guides/self-hosting.mdx`, "The host's
  bindings": a subsection "On nix-darwin and NixOS" covering
  `services.timber.settings`, each key named as in the file; a table
  left unset left out, so the given holds; a credential and a key named
  by their paths and kept out of the store, evaluation failing
  otherwise; the file linked at `~/.config/timber/config.toml`, a file
  there moved aside once; the package in the system profile,
  `package = null` installing none; nothing else written or started.
  With a short example naming a registry, a key service and one
  credential. The `[credentials]` table itself is the Dash0 change's to
  describe on this page.
- `README.md`: after the paragraph on the Claude Code plugin and the
  herdr pane, one sentence naming `services.timber`; in "This
  repository", a row for `nix/`.

Neither names a host, a fleet or a customer (core.24). nix-darwin and
NixOS are named as products.

### 11. The record and the rule

Record 0035, "timber's flake gives a module that writes the host's
bindings file and nothing else", holds Decisions 1 to 9: its ask is
118's as amended at `boomdrivedev/blueprints` 573ec1a; the rest are the
construct stage's choices of 2026-11-14, made within records 0013 and
0033 and the rules, each cheap to change, with the paths not taken.

The Dash0 change of the same bolt writes record 0034 and rule core.75,
so this change takes the numbers after them.

Rule core.76, under a new heading `### The flake`, and its details:

> **[core.76]** The flake's module writes the host's bindings file
> alone, `~/.config/timber/config.toml`, from options named as the file
> names them, a key left unset left out of the file; it names a
> credential or a key only by a path outside the Nix store, writes no
> other tool's file, and starts nothing of timber's.

Details: rules out an option named otherwise than the file's key; a
given binding or a rule of the reader's held in the module; a credential
or a `[signing] key` under the store directory, holding a secret's text
or given as a Nix path value; the module reading or copying a
credential's file; a launchd agent or daemon, a systemd unit or a file
under `/etc` for timber, `timber serve` among them (114, core.36); the
module linking `~/.config/timber/` itself, or writing under the data
directory (core.54); a file of another tool's. Not ruled out: the
package in the system profile, replaced or null; a key the module does
not declare passed to the file for timber to judge; the file a link into
the store, a file there moved aside once; Boomdrive's module setting
`services.timber`'s options and re-exporting the module (boomdrive
record 0023). Source: 45, 64, 92, 114, 118 as amended at 573ec1a;
boomdrive record 0023 as amended on 2026-11-13, its core.18 and core.46;
records 0013, 0033, 0035.

Rule core.1's details gain, under "Not ruled out", "the flake's modules
under `nix/`, a surface outside the core that names the bindings file's
keys as the file names them (record 0035)". Rule core.25's text names
the flake's `nix/` beside `site/` and each crate and plugin directory.

### Rules relied on

core.1, core.21, core.24, core.25, core.26, core.36, core.38, core.54,
core.74.

### Departures

None.

## Risks / Trade-offs

- **`[credentials]` before timber reads it.** Until the Dash0 change
  lands, timber refuses a file holding `[credentials]`, naming it as no
  table of the bindings file. Both changes land in one bolt, so no
  release carries the module without the reader.
- **Two writers of one path while a host moves.** A host that enables
  the module while another writer, a home-manager file among them, still
  writes `~/.config/timber/config.toml` has two writers; the last
  activation wins. The host drops the other in the deploy that enables
  the module.
- **The first shell after `flake.lock` moves fetches the inputs.**
  devenv's evaluation fetches nixpkgs, rust-overlay and nix-darwin once
  per lock. An entry with no network fails the flake's checks for want
  of them.
- **nix-darwin's head against the locked nixpkgs.** `nix flake lock`
  takes nix-darwin's newest commit, which may ask for a newer nixpkgs
  than 02c087f. Where "The nix-darwin module" fails on that, the lock
  names the newest nix-darwin commit that evaluates against it.
- **The renderer and the link are copies of sluice's.** A fix to one is
  not a fix to the other. Each file's opening comment names the revision
  it was taken from.
- **A path timber cannot read still evaluates.** The module refuses a
  store path and a value that is no path. A path to a missing file is
  timber's to answer at its read, as for a hand-written file.

## Task notes

**1.1** `flake.nix`: the opening comment (Decision 1); `systems` the
three, with the comment; `nix-darwin` as an input with sluice's comment;
`...` in the outputs' arguments. `flake.lock` from
`devenv shell -- "$TIMBER_FLAKE_NIX" flake lock` once devenv.nix sets the
variable, or the host's `nix flake lock`. `nix/inputs.nix`: sluice's at
7ce112b4. `devenv.nix`: Decision 8's three additions. `nix/moon.yml`
with its opening comment, sluice's in timber's words, and `lint`.
`.moon/workspace.yml`: `nix: 'nix'` and the comment.

**1.2** `nix/check.sh` (Decision 8). `nix/tests/default.nix`, with
`test` an empty list of files until 2.1; `nix/tests/lib.nix`, sluice's
with `systems` the three, `darwinSystem` and `nixosSystem` added in 3.1
and 3.2; `nix/tests/typecheck.nix` with "Every system's package".
`nix/moon.yml`: `typecheck` and `test`.

**2.1** `nix/modules/toml.nix` (Decision 5). `nix/modules/options.nix`
(Decisions 2 to 4), its opening comment sluice's in timber's words.
`nix/tests/settings.nix`:

```nix
{
  empty = { };
  laptop = {
    store.layout = "~/.local/share/timber/layout";
    landing.branch = { };
    ledger.git_ref.remote = "origin";
    signing = {
      signer = "host-a";
      key = "~/.config/timber/signing";
    };
    gate.publish = { };
    trust.host-b.ssh = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGt host-b";
  };
  fleet = {
    store.registry = "registry.example.com/timber";
    landing.sluice.remote = "origin";
    ledger.store = { };
    signing.kms = "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab";
    gate.job = "the release job a push to main wakes";
  };
  credentials.credentials = {
    dash0 = "/run/secrets/dash0-token";
    npm = "/run/secrets/npm-token";
  };
}
```

`nix/tests/files/empty.toml`, `laptop.toml`, `fleet.toml` and
`credentials.toml`: the text the renderer gives, written once, read and
checked by hand against the self-hosting guide's example and the table
in Context, then committed. `laptop.toml` reads:

```toml
# Written by timber's flake module from services.timber.settings; a change goes there.

[gate.publish]

[landing.branch]

[ledger.git_ref]
remote = "origin"

[signing]
key = "~/.config/timber/signing"
signer = "host-a"

[store]
layout = "~/.local/share/timber/layout"

[trust.host-b]
ssh = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGt host-b"
```

`nix/tests/file.nix`, over `options.nix` evaluated alone with
`lib.evalModules` beside an `assertions` option, as sluice's is. Its
cases are the spec's: "No settings", "A laptop's file", "A fleet's
file", "A host's credentials", "A table set empty", "A key the module
does not declare",
"A value the file cannot hold" (`builtins.tryEval` of the text and the
failure's words), and "A signing key in the store", "A signing key's
text", "A credential in the store", "A credential's text", "A key given
as a Nix path" and "A credential given as a Nix path" (by the
assertions' messages). `nix/tests/default.nix` takes `file.nix` into `test`.
`treefmt.nix`: Decision 8's exclude.

**2.2** `crates/timber/src/bindings/tests.rs`:
`the_modules_files_are_read_as_written` (Decision 9), citing 118 and
record 0035, beside `each_table_is_read`, using its `Remote` and the
`Host` helper. For `laptop.toml`: `[store] layout` at the home's
`.local/share/timber/layout`; `[landing] branch` with no remote;
`[ledger] git_ref` with the remote `origin`; `Signing::HostKey` as
`host-a` with its key at the home's `.config/timber/signing`;
`Teller::of` the gate is `Teller::Publish`; `trust` the one signer
`host-b` verified by its line. For `fleet.toml`: `[store] registry` with
its value; `[landing] sluice` with the remote `origin`; `[ledger]
store`; `Signing::Kms` of the ARN; the gate `Teller::Job` with its
words; no trust. `crates/timber/moon.yml`: the input.

**2.3** `nix/modules/activate.sh` (Decision 6). `nix/tests/activate.sh`,
which `nix/check.sh test` runs after the evaluated cases, each case in a
scratch `HOME`, printing `nix: activate: <case>: <what>` for each
failure: "Nothing there", "A link there", "A file there" (with
`.config/timber/signing` beside the file, left as it was) and "A file
there and one aside already".

**3.1** `nix/modules/darwin.nix` (Decisions 6 and 7), given
`{ packages }`, importing `options.nix`. `flake.nix`: `moduleOf` and
`darwinModules`. `nix/tests/lib.nix`: `darwinSystem`, sluice's,
aarch64-darwin, primary user alice at `/Users/alice`,
`system.stateVersion = 6`, `services.timber.enable = true`, the module
`given`. `nix/tests/typecheck.nix`: "The nix-darwin module".
`nix/tests/darwin.nix`: "On nix-darwin", "A primary user with no home",
"The flake's timber", "A host's own timber"
(`pkgs.writeTextDir "bin/timber" ""`), "No package", "Nothing else on
nix-darwin" and "The module imported twice on nix-darwin". `default.nix`
takes it into `test`.

**3.2** `nix/modules/nixos.nix`, as 3.1 for NixOS, with `user` and its
assertion. `flake.nix`: `nixosModules`. `nix/tests/lib.nix`:
`nixosSystem`, sluice's, x86_64-linux, normal users alice and bob, the
module enabled for alice. `nix/tests/typecheck.nix`: "The NixOS module".
`nix/tests/nixos.nix`: "On NixOS", "A user the system does not declare"
(`carol`), "The flake's timber", "A host's own timber", "No package",
"Nothing else on NixOS" and "The module imported twice on NixOS".
`default.nix` takes it into `test`.

**4.1** The guide's subsection and the README's sentence and row
(Decision 10).

**4.2** `docs/adr/0035-timbers-flake-gives-a-module-that-writes-the-hosts-bindings-file-and-nothing-else.md`,
MADR, its frontmatter and status line as record 0031's, accepted on the
brief, as Decision 11 says.

**4.3** `docs/architecture/core.md`: Decision 11's rule core.76 under
`### The flake`, after `### The release`, and its details after the
last rule's; core.1's details and core.25's text as Decision 11 says.
