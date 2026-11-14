# Tasks

Every command runs from the place's root, as written. Build the tasks
in order: each group's checks read what the groups before it write. The
flake's checks evaluate offline and read-only against a dummy store,
with nix's configuration, caches and state in a scratch folder, and
touch nothing of the host's: no test reads or writes the host's own
`~/.config/timber`, its data directory or its nix store. The design's
Task notes give each task's files and names. Commit by pathspec,
Conventional Commits scoped to the part of the tree, citing clauses by
number and rules by id. Submit nothing: Flume submits the place once its
last stage delivers.

## 1. The flake's checks

- [x] 1.1 Give the flake its checks' tools, inputs and project, as the design's Decisions 1 and 8 and Task note 1.1 say. `flake.nix` gives aarch64-darwin, aarch64-linux and x86_64-linux and takes nix-darwin as the tests' input, and `flake.lock` locks it. `nix/inputs.nix` is sluice's at 7ce112b4. `devenv.nix` pins statix and nix by version, statix on `PATH` and nix by `TIMBER_FLAKE_NIX`, and sets `TIMBER_FLAKE_INPUTS`. `nix/moon.yml` holds the project `nix` with its `lint` task, and `.moon/workspace.yml` lists it (host-bindings-module spec, "The flake gives three systems"). Verify with `devenv shell -- moon run nix:lint`.
- [x] 1.2 Add the type check and the test's frame, as Decision 8 and Task note 1.2 say: `nix/check.sh`, `nix/tests/default.nix`, `nix/tests/lib.nix`, and `nix/tests/typecheck.nix` with the spec's "Every system's package"; `nix/moon.yml` gains `typecheck` and `test`. Verify with `devenv shell -- moon run nix:typecheck`.

## 2. The module's settings and file

- [x] 2.1 Write `services.timber`'s shared options and the file they render, as Decisions 2 to 5 and Task note 2.1 say: `nix/modules/toml.nix`, sluice's renderer at 7ce112b4 naming timber's module; `nix/modules/options.nix`, with the assertions on `signing.key` and each `credentials.<name>` and the taking out of every null; `nix/tests/settings.nix` and `nix/tests/files/empty.toml`, `laptop.toml`, `fleet.toml` and `credentials.toml`; and `nix/tests/file.nix` with the spec's "No settings", "A laptop's file", "A fleet's file", "A host's credentials", "A table set empty", "A key the module does not declare", "A value the file cannot hold", "A signing key in the store", "A signing key's text", "A credential in the store", "A credential's text", "A key given as a Nix path" and "A credential given as a Nix path". Exclude `nix/tests/files/**` in `treefmt.nix` with its reason. Verify with `devenv shell -- moon run nix:test` and `devenv shell -- treefmt --ci nix treefmt.nix`.
- [x] 2.2 Show that timber reads what the module writes, as Decision 9 and Task note 2.2 say: `the_modules_files_are_read_as_written` in `crates/timber/src/bindings/tests.rs` reads `nix/tests/files/laptop.toml` and `fleet.toml` as the spec's "Reading the module's files" says, and `cli:test`'s inputs in `crates/timber/moon.yml` gain `/nix/tests/files/**/*`. Verify with `devenv shell -- cargo test -p timber --lib bindings::tests::the_modules_files_are_read_as_written`.
- [x] 2.3 Write the activation's link, as Decision 6 and Task note 2.3 say: `nix/modules/activate.sh link <file>`, and `nix/tests/activate.sh`, which `nix/check.sh test` runs after the evaluated cases, with the spec's "Nothing there", "A link there", "A file there" and "A file there and one aside already". Verify with `devenv shell -- moon run nix:test`.

## 3. The modules on each platform

- [x] 3.1 Write the nix-darwin module, as Decisions 1, 6 and 7 and Task note 3.1 say: `nix/modules/darwin.nix`, `moduleOf` with `darwinModules.timber` and `default` in `flake.nix`, and `darwinSystem` in `nix/tests/lib.nix`. Add "The nix-darwin module" to `nix/tests/typecheck.nix`, and `nix/tests/darwin.nix` with the spec's "On nix-darwin", "A primary user with no home", "The flake's timber", "A host's own timber", "No package", "Nothing else on nix-darwin" and "The module imported twice on nix-darwin". Verify with `devenv shell -- moon run nix:typecheck nix:test`.
- [x] 3.2 Write the NixOS module, as Decisions 1, 6 and 7 and Task note 3.2 say: `nix/modules/nixos.nix` with `services.timber.user`, `nixosModules.timber` and `default` in `flake.nix`, and `nixosSystem` in `nix/tests/lib.nix`. Add "The NixOS module" to `nix/tests/typecheck.nix`, and `nix/tests/nixos.nix` with the spec's "On NixOS", "A user the system does not declare", "The flake's timber", "A host's own timber", "No package", "Nothing else on NixOS" and "The module imported twice on NixOS". Verify with `devenv shell -- moon run nix:typecheck nix:test`.

## 4. Say it, and record it

- [x] 4.1 In `site/src/content/docs/guides/self-hosting.mdx`, "The host's bindings", add "On nix-darwin and NixOS", saying what timber's module writes and refuses, with a short example, and in `README.md` the sentence naming `services.timber` and the row for `nix/`, as Decision 10 says. Name no host, fleet or customer (core.24). Verify with `devenv shell -- moon run site:build`, and that `grep -c 'services.timber' README.md` prints at least `1`.
- [ ] 4.2 Write record 0035, timber's flake gives a module that writes the host's bindings file and nothing else, accepted on the brief, with Decisions 1 to 9 and the paths not taken, as Decision 11 and Task note 4.2 say. Verify with `devenv shell -- moon run root:adrs`.
- [ ] 4.3 In `docs/architecture/core.md`, add rule core.76 under `### The flake` with its details, name the flake's modules in core.1's details and the flake's `nix/` in core.25, as Decision 11 and Task note 4.3 say. Verify that `grep -c 'core\.76' docs/architecture/core.md` prints `2` and `grep -c 'record 0035' docs/architecture/core.md` prints at least `2`.
