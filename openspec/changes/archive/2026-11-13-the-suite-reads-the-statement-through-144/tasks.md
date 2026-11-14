# Tasks

Every command runs from the place's root, inside `devenv shell`.

## 1. The suite reads the statement through 144

- [x] 1.1 The suite reads the statement at `boomdrivedev/blueprints a8234b3` and holds 1 to 144, 144 after 20 in A.3 (clause-coverage spec). Verify with `moon run conformance:test`.
- [x] 1.2 The register names 144 with phase 2's link of a publish and its reason. Verify with `moon run conformance:test conformance:coverage`, whose output lists 144 as pending.
- [x] 1.3 `docs/architecture/_open.md` names publish as code that stands against 144. Verify with `moon run root:format`.

## 2. The specs

- [x] 2.1 The delta spec moves the suite's clauses to 1 to 144 and adds 144's register entry. Verify with `openspec validate the-suite-reads-the-statement-through-144 --strict`, then archive with `openspec archive the-suite-reads-the-statement-through-144 --yes`.
- [x] 2.2 The gate holds. Verify with `moon run ':#check-fast'`.
