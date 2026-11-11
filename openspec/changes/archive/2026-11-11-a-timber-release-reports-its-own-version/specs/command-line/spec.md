## ADDED Requirements

### Requirement: The binary says its version, its revision and its build

`timber --version` SHALL print one line,
`timber <version>, revision <revision>, built by <build>`: the version
its tree's `Cargo.toml` gives, `0.0.0` on main and the release's at a
release's tag; the revision it was built from; and the compiler and
target that made it. Nothing but the tree SHALL give the version
(release-version; record 0031).

#### Scenario: The built binary names itself

- **WHEN** the built binary runs `timber --version`
- **THEN** it prints `timber <the workspace's version>, revision <the revision it was built from>, built by <compiler> for <target>` on one line and exits 0
