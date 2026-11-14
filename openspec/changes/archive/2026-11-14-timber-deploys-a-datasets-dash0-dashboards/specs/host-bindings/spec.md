## ADDED Requirements

### Requirement: The file names the credentials a target's tool reads

The file SHALL name each credential a target needs in `[credentials]`
(118): each key a credential's name, as an environment's target table
gives it, and each value the path of a file holding only that
credential, absolute or beginning with `~/`, expanded as the file's
other paths are. The file SHALL never hold a credential, and timber
SHALL never open the file a path names (45, 64, rule core.21). A file
with no `[credentials]` SHALL name none.

#### Scenario: A credential named by its path

- **WHEN** the file names `[credentials] dash0-dashboards = "~/.local/share/secrets/dash0-dashboards"`, and `timber apply` runs
- **THEN** the targets are handed `dash0-dashboards` with that path expanded against the home directory, and the file at that path is not opened

#### Scenario: No credentials table

- **WHEN** the file names no `[credentials]`, and `timber publish` runs
- **THEN** it is bound as before, and the targets are handed no credential

### Requirement: A broken credentials table binds nothing

A `[credentials]` value that is not a string, or a path neither
absolute nor beginning with `~/`, SHALL make every command bound by the
file unreadable, naming the file and the key, and SHALL bind nothing in
its place (rule core.53).

#### Scenario: A credential in place of its path

- **WHEN** the file names `[credentials] dash0-dashboards = "auth_abc"`
- **THEN** every command bound by the file answers unreadable naming the file and `[credentials] dash0-dashboards`, and nothing is read or written

### Requirement: The binding hands the targets the host's credentials

The binding SHALL hand every target profile the credentials
`[credentials]` names and the bindings file's path. The bindings module
SHALL read the table generically and name no tool or service (rule
core.1).

#### Scenario: An apply's targets

- **WHEN** the file names `[credentials] dash0-dashboards`, and `timber apply` binds its targets
- **THEN** the target family is handed that credential's path and the bindings file's path
