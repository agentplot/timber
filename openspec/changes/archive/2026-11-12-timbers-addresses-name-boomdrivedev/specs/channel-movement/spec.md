## MODIFIED Requirements

### Requirement: The givens are a fragment beneath the file

The given channels SHALL be a policy fragment in the policy file's own
form, shipped with timber as data in the policy profile's directory
and laid beneath the repository's file, the two read as one table in
force by every answer the profile gives (C.10). The fragment SHALL hold
`beta`, moving on each accepted publish with releases alone, and
`stable`, bound to `beta`; and, held until the repository switches them
on, `alpha`, moving on each accepted publish with prereleases too, and
`lts`, bound to `stable` and moved by no rule, so only an asking moves
it. The givens that are not held SHALL be declared, by the fragment:
every component SHALL have `beta` and `stable` with no line written,
listed like a channel declared by hand, with the policy for each
answering like any channel's, and never answered as not declared. A
repository that declares no channel, as 102 lets it, is one that writes
no channel line: its versions are offered and accepted as 102 says, and
it still has the givens, which the fragment declares (as ruled at
boomdrivedev/blueprints b64d367).

#### Scenario: A repository that writes no channel

- **WHEN** the policy file at a revision declares the components `lib` and `web` and no channel
- **THEN** the channels answer is `beta` of `lib` and of `web`, each moving on each publish, and `stable` of each, bound to its component's `beta`, and neither `alpha` nor `lts`

#### Scenario: A given's policy with no line written

- **WHEN** the policy file writes no channel and names `gate` as a signer that may claim acceptance, and the policy for `beta` and for `stable` is asked
- **THEN** each answers the gate's acceptance with `gate` as its promoter, `stable` naming `beta` as the channel it is bound to, and neither is absent

#### Scenario: A repository with no policy file

- **WHEN** the revision holds no policy file
- **THEN** the channels answer is `beta` and `stable` of the one component, `stable` bound to `beta`

#### Scenario: Another fragment

- **WHEN** the profile is given, in place of the shipped fragment, one holding only `edge` moving on each publish, and the revision holds no policy file
- **THEN** the channels answer is `edge` alone, and the gate moves `edge` on an accepted publish with nothing else changed

### Requirement: A held given is answered with the line that switches it on

The channels answer SHALL carry, apart from the channels in force,
every given the profile holds that the repository has not switched on,
for every component, marked held, with how it would move once switched
on and the line that would switch it on, read from the fragment:
`[channels.<name>]` where the repository declares no component and
`[channels.<component>.<name>]` where it declares components (C.10 as
amended at boomdrivedev/blueprints b1267d7, 131 as amended at 1ae1cc5). A
given the repository switches on SHALL leave the held givens and join
the channels in force, and one it does not SHALL never be among the
channels in force.

#### Scenario: Nothing switched on

- **WHEN** the policy file writes no channel and declares no component
- **THEN** the channels answer holds off `alpha`, moving on each publish with prereleases, with the line `[channels.alpha]`, and `lts`, bound to `stable`, with the line `[channels.lts]`

#### Scenario: One component's alpha switched on

- **WHEN** the file declares `lib` and `web` and holds `[channels.lib.alpha]`
- **THEN** the channels answer holds off `alpha` of `web` with the line `[channels.web.alpha]` and `lts` of each with its component's line, and carries `alpha` of `lib` among the channels in force alone

#### Scenario: Another fragment's held given

- **WHEN** the profile is given, in place of the shipped fragment, one holding `edge` moving on each publish and `canary` held, bound to `edge`
- **THEN** the channels answer holds off `canary` with the line `[channels.canary]`, and its channels in force are `edge` alone

### Requirement: A manual environment has a channel of its own

An environment whose `policy` is `manual` SHALL give it a channel of
its own, named for the environment, in the component of the channel it
names and bound to that channel, moved only at an asking (41, 97,
C.10). Who may ask for it SHALL be named by `approvers` on the
environment, `[environments.<environment>]`, which the made channel
takes as its own; never under the made channel, which the repository
does not write approvers on (C.10, as ruled at boomdrivedev/blueprints
088430c). An environment whose `policy` is `auto`, or unwritten, SHALL
make no channel.

#### Scenario: prod waits on stable

- **WHEN** the environment `prod` names the channel `stable` and `policy = "manual"`
- **THEN** the channels answer lists `prod`, bound to `stable`, beside `beta` and `stable`

#### Scenario: An environment that moves on its own

- **WHEN** the environment `site` names the channel `stable` and writes no `policy`
- **THEN** no channel named `site` is listed

#### Scenario: Who may say yes to prod

- **WHEN** the manual environment `prod` names `stable` and `approvers = ["ci"]`
- **THEN** `prod` is listed bound to `stable`, and its policy names `ci` as the one signer that may ask for it
