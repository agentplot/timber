## MODIFIED Requirements

### Requirement: The environments' words are read

An environment SHALL be declared as `[environments.<name>]` where no
component is declared and `[environments.<component>.<name>]` where
components are. Its keys SHALL be C.10's words and no others: its
`channel`, `policy` (`auto` or `manual`, `auto` where unwritten),
`after`, a list of environment names of the same component, `target`,
and, where the environment is manual, `approvers`, the signers that may
say yes, read as a channel's `approvers` is (C.10, as ruled at
boomdrivedev/blueprints 088430c). `target` SHALL be a table of exactly one
key, the profile of the environment's target, whose value is a table,
the target's own settings, which the profile SHALL carry as written and
read nothing of (C.10, 136). Any other `policy`, or a `channel` the file
does not declare, give or make, SHALL be unreadable naming the table. A
`target` naming no profile or two, or whose profile's value is not a
table, SHALL be unreadable naming the table and `target`. `approvers` on
an environment that is not manual SHALL be unreadable naming the table
and the key, since no one says yes to it. Any other key SHALL be
unreadable naming the table and the key, since a target's own settings
sit in the target's table and nowhere else (C.10).

#### Scenario: An environment naming its target

- **WHEN** `[environments.site]` names `channel = "stable"` and `target.nix_cache = { attic = "fleet:site" }`
- **THEN** the channels answer is read, and `site` makes no channel

#### Scenario: An environment with a target's own key

- **WHEN** `[environments.site]` names `channel = "stable"` and `bucket = "example-site"`
- **THEN** the channels are unreadable naming `[environments.site]` and `bucket`, and saying a target's settings sit in the target's table

#### Scenario: A target naming two profiles

- **WHEN** `[environments.site]` names `channel = "stable"` and a `target` table with the keys `nix_cache` and `bucket`
- **THEN** the channels and the environments are unreadable naming `[environments.site]` and `target`

#### Scenario: A target whose profile names no table

- **WHEN** `[environments.site]` names `target = { nix_cache = "fleet:site" }`
- **THEN** the channels and the environments are unreadable naming `[environments.site]` and `target`

#### Scenario: Approvers in a manual environment's table

- **WHEN** `[signers.ci]` is declared and `[environments.prod]` names `channel = "stable"`, `policy = "manual"` and `approvers = ["ci"]`
- **THEN** the channels answer lists `prod` bound to `stable`, and the policy for `prod` names `ci` as the one signer that may ask

#### Scenario: Approvers on an environment that moves on its own

- **WHEN** `[environments.site]` names `channel = "stable"` and `approvers = ["ci"]`, and no `policy`
- **THEN** the channels are unreadable naming `[environments.site]` and `approvers`

#### Scenario: An unknown policy word

- **WHEN** `[environments.prod]` names `policy = "sometimes"`
- **THEN** the channels are unreadable naming `[environments.prod]` and `sometimes`

### Requirement: An environment's channel is named for it

The channel a manual or after environment is given SHALL take the
environment's name, in its component, and a line under it SHALL add
claims after the gate's acceptance (C.10). A channel so made that
shares a name with a channel the fragment holds, held or not, or with
another made channel, or a line under it naming another binding or
`on_each_publish`, SHALL be unreadable naming the table; so SHALL a
line under it naming `approvers`, saying that who may ask for it is
written on the environment, since the made channel takes its
environment's (C.10, as ruled at boomdrivedev/blueprints 088430c).

#### Scenario: A manual environment named beta

- **WHEN** `[environments.beta]` names `channel = "stable"` and `policy = "manual"`
- **THEN** the channels are unreadable naming `[environments.beta]`

#### Scenario: A line that restates prod's binding

- **WHEN** the manual environment `prod` names `stable`, and `[channels.prod]` names `bound_to = "stable"` and a `require`
- **THEN** `prod` is listed bound to `stable`, and its policy requires the gate's acceptance and then the line's claims

#### Scenario: Approvers under a made channel

- **WHEN** the manual environment `prod` names `stable`, and `[channels.prod]` names `approvers = ["ci"]`
- **THEN** the channels are unreadable naming `[channels.prod]` and `approvers`, and saying who may ask for `prod` is written on `[environments.prod]`

#### Scenario: A manual environment named for a held given

- **WHEN** `[environments.lts]` names `channel = "stable"` and `policy = "manual"`, and the file writes no `[channels.lts]`
- **THEN** the channels are unreadable naming `[environments.lts]`
