# Tasks

Every command runs from the place's root, inside `devenv shell`.

## 1. The addresses of the moved tools name boomdrivedev

- [x] 1.1 The README, CLAUDE.md, `openspec/config.yaml`, the rules in `docs/architecture/core.md`, the crates' module comments, `cliff.toml`, the flake's header and homepage, the pages workflow, the site's configuration and pages, and both plugins' manifests and READMEs name boomdrivedev/timber, boomdrivedev/blueprints, https://boomdrivedev.github.io/timber and https://boomdrivedev.github.io/blueprints/timber/. sluice's, site-theme's, boomdrive's and boomdrive-web's addresses stay agentplot's. Verify that `git grep -n -E 'agentplot(/|\.github\.io/)(blueprints|timber|flume|weir|crew)' -- ':!openspec/changes/archive' ':!openspec/specs/release-version' ':!docs/adr/0032-*'` prints only claim kinds under `https://agentplot.github.io/timber/claim/`, the tests' and specs' fixtures, and the local path of one host's checkout in `openspec/config.yaml` (record 0032).
- [x] 1.2 The suite reads the statement at `boomdrivedev/blueprints 90cf023` (clause-coverage spec). Verify with `moon run conformance:test`.
- [x] 1.3 The records' citations of the statement and the design book name boomdrivedev/blueprints at the revisions they cite. Verify with `moon run root:adrs`.
- [x] 1.4 Record 0032 records which addresses move and which names stay. Verify with `moon run root:adrs`.

## 2. The specs

- [x] 2.1 The delta specs move each requirement's citation and the marketplace's address. Verify with `openspec validate timbers-addresses-name-boomdrivedev --strict`, then archive with `openspec archive timbers-addresses-name-boomdrivedev --yes`.
- [x] 2.2 The gate holds. Verify with `moon run ':#check-fast'`.
