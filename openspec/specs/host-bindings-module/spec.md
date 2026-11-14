# host-bindings-module Specification

## Purpose
timber's own module for a host configured by Nix: its flake gives a
nix-darwin and a NixOS module, `services.timber`, that writes the host's
bindings file, `~/.config/timber/config.toml`, from options named as the
file names them, holds no credential or key, writes no other tool's file
and starts nothing of timber's (118 as amended at
`boomdrivedev/blueprints` 573ec1a; record 0035; rule core.76).

## Requirements

### Requirement: timber's flake gives a module for nix-darwin and for NixOS

timber's flake SHALL give `darwinModules.timber` and
`nixosModules.timber`, each also `default`, declaring
`services.timber.enable`, `services.timber.package` and
`services.timber.settings`, and `services.timber.user` on NixOS. Each
SHALL evaluate in a system of its platform with the module enabled and
no other flake's module (118; boomdrive record 0023; record 0035; rule
core.76).

#### Scenario: The nix-darwin module

- **WHEN** an aarch64-darwin system whose primary user is `alice`, home
  `/Users/alice`, imports `darwinModules.default` and sets
  `services.timber.enable = true`
- **THEN** the system evaluates to a derivation ("The nix-darwin module"
  in `nix/tests/typecheck.nix`)

#### Scenario: The NixOS module

- **WHEN** an x86_64-linux system with the normal user `alice` imports
  `nixosModules.default` and sets `services.timber.enable = true` and
  `services.timber.user = "alice"`
- **THEN** the system's toplevel evaluates to a derivation ("The NixOS
  module" in `nix/tests/typecheck.nix`)

### Requirement: A module imported twice is taken once

`darwinModules.timber` and `nixosModules.timber`, and each `default`,
SHALL carry the key
`github:boomdrivedev/timber#<platform>Modules.timber`, `darwin` or
`nixos`, so a system that imports the module itself and through another
flake's module declares `services.timber` once (boomdrive record 0023;
record 0035).

#### Scenario: The nix-darwin module imported twice

- **WHEN** the nix-darwin system of "The nix-darwin module" imports
  `darwinModules.timber` again under another module
- **THEN** `darwinModules.timber` and `darwinModules.default` each carry
  the key `github:boomdrivedev/timber#darwinModules.timber`, the system
  evaluates to a derivation, and its activation links the file once
  ("The module imported twice on nix-darwin" in `nix/tests/darwin.nix`)

#### Scenario: The NixOS module imported twice

- **WHEN** the NixOS system of "The NixOS module" imports
  `nixosModules.timber` again under another module
- **THEN** `nixosModules.timber` and `nixosModules.default` each carry
  the key `github:boomdrivedev/timber#nixosModules.timber`, the system's
  toplevel evaluates to a derivation, and alice's user activation links
  the file ("The module imported twice on NixOS" in
  `nix/tests/nixos.nix`)

### Requirement: The flake gives three systems

The flake SHALL give its package for aarch64-darwin, aarch64-linux and
x86_64-linux and for no other system, each evaluating to a derivation
(record 0035).

#### Scenario: Every system's package

- **WHEN** the flake's `packages` are evaluated
- **THEN** it names aarch64-darwin, aarch64-linux and x86_64-linux and
  no other system, and each one's `default` evaluates to a derivation
  ("Every system's package" in `nix/tests/typecheck.nix`)

### Requirement: The settings are the host's bindings file under its own names

`services.timber.settings` SHALL render the text of the host's bindings
file, each key named as the file names it. The module SHALL declare
every table and key timber's reader takes, `store`, `landing`,
`ledger`, `signing`, `gate`, `trust.<signer>` and `credentials.<name>`,
each with its TOML type and no default (92, 118, C.1, C.2, C.5 to C.8,
C.13, C.14, C.16; records 0013, 0033, 0035).

#### Scenario: A laptop's file

- **WHEN** the module is enabled with `store.layout =
  "~/.local/share/timber/layout"`, `landing.branch = { }`,
  `ledger.git_ref.remote = "origin"`, `signing` with `signer = "host-a"`
  and `key = "~/.config/timber/signing"`, `gate.publish = { }` and
  `trust.host-b.ssh` a public key's line
- **THEN** the file is byte for byte `nix/tests/files/laptop.toml`, its
  tables `[gate.publish]`, `[landing.branch]`, `[ledger.git_ref]`,
  `[signing]`, `[store]` and `[trust.host-b]` in that order ("A laptop's
  file" in `nix/tests/file.nix`)

#### Scenario: A fleet's file

- **WHEN** the module is enabled with `store.registry =
  "registry.example.com/timber"`, `landing.sluice.remote = "origin"`,
  `ledger.store = { }`, `signing.kms` a key's ARN and `gate.job = "the
  release job a push to main wakes"`
- **THEN** the file is byte for byte `nix/tests/files/fleet.toml` ("A
  fleet's file" in `nix/tests/file.nix`)

#### Scenario: A host's credentials

- **WHEN** the module is enabled with `credentials.dash0 =
  "/run/secrets/dash0-token"` and `credentials.npm =
  "/run/secrets/npm-token"`, and nothing else set
- **THEN** the file is byte for byte `nix/tests/files/credentials.toml`,
  its `[credentials]` holding `dash0` and then `npm`, each its path ("A
  host's credentials" in `nix/tests/file.nix`)

### Requirement: What is unset is left out, and a table set empty is written

A table or key left unset SHALL be left out of the file, so the given
binding holds. A table set empty SHALL be written, since it binds its
profile. A key the module does not declare SHALL pass to the file as
written, for timber to judge (118; records 0013, 0021, 0035).

#### Scenario: A table set empty

- **WHEN** the module is enabled with `landing.branch = { }` and nothing
  else set
- **THEN** the file is the header line, an empty line and
  `[landing.branch]`, and holds no other table ("A table set empty" in
  `nix/tests/file.nix`)

#### Scenario: A key the module does not declare

- **WHEN** the module is enabled with `settings.store.later = "soon"`
  beside `store.layout`
- **THEN** the file's `[store]` holds `later = "soon"` and then `layout`
  ("A key the module does not declare" in `nix/tests/file.nix`)

### Requirement: The file's form

The file SHALL open with one line saying the module wrote it from
`services.timber.settings`, then its keys before its tables, each sorted
by name, a table set empty written as its header alone. A value that is
not a string, a boolean, an integer, a list of strings, a table or a
list of tables SHALL fail evaluation, naming its option (record 0035).

#### Scenario: No settings

- **WHEN** the module is enabled with `settings = { }`
- **THEN** the file is the header line alone, byte for byte
  `nix/tests/files/empty.toml` ("No settings" in `nix/tests/file.nix`)

#### Scenario: A value the file cannot hold

- **WHEN** the module is enabled with `settings.store.share = 1.5`
- **THEN** evaluating the file fails, naming
  `services.timber.settings.store.share` ("A value the file cannot
  hold" in `nix/tests/file.nix`)

### Requirement: timber reads the module's files as written

The files the module renders for a laptop and for a fleet SHALL be read
by timber's own reader of the bindings file, which refuses a table it
does not bind, as the settings name each table (118; records 0013,
0035).

#### Scenario: Reading the module's files

- **WHEN** timber's reader reads `nix/tests/files/laptop.toml` and then
  `nix/tests/files/fleet.toml` as a host's bindings file, with a home of
  the test's own
- **THEN** it gives, as written (`the_modules_files_are_read_as_written`
  in `crates/timber/src/bindings/tests.rs`):
  - for the laptop, `[store] layout` at the home's
    `.local/share/timber/layout`, `[landing] branch` with no remote,
    `[ledger] git_ref` with the remote `origin`, the host key signer
    `host-a` with its key at the home's `.config/timber/signing`, the
    host's own publish telling its gate, and the one trusted signer
    `host-b` verified by its line;
  - for the fleet, `[store] registry` naming
    `registry.example.com/timber`, `[landing] sluice` with the remote
    `origin`, `[ledger] store`, the key service signer of the ARN, the
    job's words telling the gate, and no trusted signer.

### Requirement: A credential and a key are named by their paths, outside the store

Each `services.timber.settings.credentials.<name>`, and
`services.timber.settings.signing.key`, SHALL be a path that begins with
`/` or `~/`, outside the Nix store directory. A value
otherwise, or a Nix path value, SHALL fail evaluation, naming the
option, and the failure SHALL say that the store is read by every user.
The module SHALL read and copy no credential and no key (45, 64, 118;
record 0033; rules core.21, core.76).

#### Scenario: A signing key in the store

- **WHEN** the module is enabled with `settings.signing.key` a path
  under the store directory
- **THEN** evaluation fails, naming
  `services.timber.settings.signing.key` and the store ("A signing key
  in the store" in `nix/tests/file.nix`)

#### Scenario: A signing key's text

- **WHEN** the module is enabled with `settings.signing.key` set to
  `-----BEGIN OPENSSH PRIVATE KEY-----`
- **THEN** evaluation fails, naming
  `services.timber.settings.signing.key` ("A signing key's text" in
  `nix/tests/file.nix`)

#### Scenario: A credential in the store

- **WHEN** the module is enabled with `settings.credentials.dash0` a
  path under the store directory
- **THEN** evaluation fails, naming
  `services.timber.settings.credentials.dash0` and the store ("A
  credential in the store" in `nix/tests/file.nix`)

#### Scenario: A credential's text

- **WHEN** the module is enabled with `settings.credentials.dash0` set
  to `auth_0123456789abcdef`
- **THEN** evaluation fails, naming
  `services.timber.settings.credentials.dash0` and saying it is no path
  ("A credential's text" in `nix/tests/file.nix`)

#### Scenario: A key given as a Nix path

- **WHEN** the module is enabled with `settings.signing.key` a Nix path
  value
- **THEN** evaluation fails, naming
  `services.timber.settings.signing.key` and the store ("A key given as
  a Nix path" in `nix/tests/file.nix`)

#### Scenario: A credential given as a Nix path

- **WHEN** the module is enabled with `settings.credentials.dash0` a Nix
  path value
- **THEN** evaluation fails, naming
  `services.timber.settings.credentials.dash0` and the store ("A
  credential given as a Nix path" in `nix/tests/file.nix`)

### Requirement: The file is linked at timber's path for the user who runs timber

The module SHALL link `~/.config/timber/config.toml` of the user who
runs timber to the file it renders: on nix-darwin the system's primary
user, in the system's activation; on NixOS `services.timber.user`, which
has no default, in that user's activation alone. A failed link SHALL
stop nothing else of the system's activation (118; rule core.76).

#### Scenario: On nix-darwin

- **WHEN** the nix-darwin system of "The nix-darwin module" enables the
  module with "A laptop's file"'s settings
- **THEN** its activation holds
  `sudo --user=alice -- /usr/bin/env HOME=/Users/alice /bin/sh <activate.sh> link <file> || true`,
  where `<file>` is the store path of the text "A laptop's file"
  renders, and no assertion fails ("On nix-darwin" in
  `nix/tests/darwin.nix`)

#### Scenario: A primary user with no home

- **WHEN** a nix-darwin system enables the module and its primary user
  has no `users.users.<user>.home`
- **THEN** evaluation fails, naming that option ("A primary user with no
  home" in `nix/tests/darwin.nix`)

#### Scenario: On NixOS

- **WHEN** the NixOS system of "The NixOS module" enables the module
  with "A laptop's file"'s settings
- **THEN** its user activation `timber` holds
  `if [ "$(id -un)" = alice ]; then <shell> <activate.sh> link <file> || true; fi`,
  where `<file>` is the store path of the text "A laptop's file"
  renders, and no assertion fails ("On NixOS" in `nix/tests/nixos.nix`)

#### Scenario: A user the system does not declare

- **WHEN** a NixOS system enables the module with
  `services.timber.user = "carol"` and declares no `carol`
- **THEN** evaluation fails, naming `services.timber.user` and
  `users.users.carol` ("A user the system does not declare" in
  `nix/tests/nixos.nix`)

### Requirement: What is at timber's path already

A link at `~/.config/timber/config.toml` SHALL be replaced. A file or
folder there that is no link SHALL be moved once to
`config.toml.before-nix`, both paths named; where that path is taken
already, the activation SHALL change neither, name both and fail. It
SHALL say a line only where it moved or linked something, and SHALL
link the file alone, writing nothing else in `~/.config/timber/` (rules
core.54, core.76).

#### Scenario: Nothing there

- **WHEN** `activate.sh link <file>` runs in a home with no
  `.config/timber`
- **THEN** `.config/timber/config.toml` links to `<file>`, and one line
  names both ("Nothing there" in `nix/tests/activate.sh`)

#### Scenario: A link there

- **WHEN** `.config/timber/config.toml` links to another file, and
  `activate.sh link <file>` runs
- **THEN** it links to `<file>`; run again, it says nothing ("A link
  there" in `nix/tests/activate.sh`)

#### Scenario: A file there

- **WHEN** `.config/timber/config.toml` is a file and
  `.config/timber/signing` is beside it, and `activate.sh link <file>`
  runs
- **THEN** the file is at `.config/timber/config.toml.before-nix`, the
  path links to `<file>`, both are named, and `signing` is as it was ("A
  file there" in `nix/tests/activate.sh`)

#### Scenario: A file there and one aside already

- **WHEN** `.config/timber/config.toml` and
  `.config/timber/config.toml.before-nix` are both files, and
  `activate.sh link <file>` runs
- **THEN** both are as they were, both are named, and it exits 1 ("A
  file there and one aside already" in `nix/tests/activate.sh`)

### Requirement: The module installs the package

With `services.timber.enable`, the module SHALL put
`services.timber.package` in the system profile. Its default SHALL be
timber's flake's package for the host's system. A host MAY replace the
package, or set it to null to install none (boomdrive record 0023; rule
core.76).

#### Scenario: The flake's timber

- **WHEN** the nix-darwin system of "The nix-darwin module" enables the
  module and sets no package
- **THEN** its system profile holds the flake's timber for
  aarch64-darwin ("The flake's timber" in `nix/tests/darwin.nix`, and
  likewise for x86_64-linux in `nix/tests/nixos.nix`)

#### Scenario: A host's own timber

- **WHEN** it sets `services.timber.package` to a package of its own
- **THEN** the system profile holds that package and not the flake's,
  and the activation's line is unchanged ("A host's own timber" in
  `nix/tests/darwin.nix` and `nix/tests/nixos.nix`)

#### Scenario: No package

- **WHEN** it sets `services.timber.package = null`
- **THEN** the system profile holds no timber, and the activation still
  holds the link's line ("No package" in `nix/tests/darwin.nix` and
  `nix/tests/nixos.nix`)

### Requirement: The module adds nothing else

The module SHALL add no launchd agent or daemon and no systemd unit,
`timber serve` among them, no file under `/etc` and no user. It SHALL
write nothing under the user's data directory and, beside the link, no
file of another tool's (114, 118, C.2, C.8; boomdrive core.18 and
core.46; rules core.36, core.54, core.76).

#### Scenario: Nothing else on nix-darwin

- **WHEN** the nix-darwin system with the module enabled is compared
  with the same system with it not enabled
- **THEN** the names under `environment.etc`, `launchd.agents`,
  `launchd.daemons`, `launchd.user.agents` and `users.users` are equal,
  and the activation, with the link's line taken out, is equal
  ("Nothing else on nix-darwin" in `nix/tests/darwin.nix`)

#### Scenario: Nothing else on NixOS

- **WHEN** the NixOS system with the module enabled is compared with the
  same system with it not enabled
- **THEN** the names under `environment.etc`, `systemd.services` and
  `systemd.user.services` are equal, each user's `linger` is equal, and
  the user activations' names differ by `timber` alone ("Nothing else on
  NixOS" in `nix/tests/nixos.nix`)
