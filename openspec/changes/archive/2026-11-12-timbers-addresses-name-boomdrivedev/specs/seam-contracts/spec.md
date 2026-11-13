## MODIFIED Requirements

### Requirement: The policy's contract

The policy SHALL answer policy-for, given a repository at a landed
revision and an audience of acceptance, a channel with its component,
or a layout, with the claims required, each with the signers that may
make it and what each may claim (82), or absent where no policy is
declared. The policy for a channel SHALL name the signers that may sign
its promotion (32) and the signers that may ask for it, none meaning any
asker the gate verifies (129, C.10), and the policy for a channel bound to another SHALL
name the channel it is bound to; the policy for acceptance SHALL name no
channel (83). The policy SHALL answer channels, given a repository at a
landed revision, with every channel its policy declares or gives, once
each, ordered by name and then by component, each with the one
component it belongs to (125) and how it moves: on each accepted
publish, with releases alone or prereleases too, by a promotion alone,
or bound to a channel of the same component (37, 83, 131, C.10); a
repository that declares one component SHALL
have every channel in it, and one that declares none SHALL have every
channel in that one, its channels spelled by name alone (123). The
same channels answer SHALL carry, apart from the channels in force,
every channel the profile gives but holds until the repository switches
it on, and the repository has not switched on, marked held, once each,
ordered by name and then by component, each with its component, how it
would move once switched on, and the line that would switch it on, in
the profile's own words; an empty list where it holds none back; and
never a channel in force (131 as amended at boomdrivedev/blueprints
1ae1cc5, C.10). The policy seam SHALL have no operation for the held
givens apart from channels (136).

#### Scenario: Acceptance names no channel

- **WHEN** policy-for is asked for acceptance
- **THEN** its answer names no channel, and requirements for acceptance cannot be made naming one

#### Scenario: The channels and policy-for agree on a binding

- **WHEN** channels answers a channel bound to another, and policy-for is asked for that channel at the same revision
- **THEN** both name the same channel it is bound to

#### Scenario: A channel's promoters

- **WHEN** policy-for is asked for a channel
- **THEN** it names the signers whose promotion of that channel counts, each with what it may claim

#### Scenario: A channel's approvers

- **WHEN** policy-for is asked for a channel whose policy names who may ask for it, and for one whose policy names none
- **THEN** the first names those signers, and the second names none, which is any asker the gate verifies

#### Scenario: A given held off

- **WHEN** the channels are asked at a revision whose policy holds a given the repository has not switched on
- **THEN** the one answer carries the channels in force and, apart from them, the held given marked held, naming its component, how it would move and the line that would switch it on, and no channel is both in force and held

#### Scenario: A channels answer written and read back

- **WHEN** a channels answer holding a channel in force and a given held off is written in its machine-readable form and read back
- **THEN** it is the same value, the held given naming the channel, its component or that the repository declares none, how it would move and its line, apart from the channels in force

### Requirement: The keeper's contract

The keeper SHALL answer append, given a channel with its component, a
promotion claim and the digest of the promotion it names as its
predecessor, with the claim held at the head of that channel's chain,
or refused naming the claim at the head (120); and chain, given a
channel with its component, with every promotion claim of it in order
from the first, or absent when none is held (121). Two components'
channels of one name SHALL be two chains (125). A chain that holds two
claims naming one predecessor SHALL be refused as a fork, naming both,
never answered by choosing one (105, 122). A chain answered from the
copy the host last read, because the source holding the ledger did not
answer, SHALL say when that copy was read and carry the source's words
of why it did not answer, a channel the copy holds no claim of answered
as a chain of no claims saying so, never as an absence; a chain read
from the source itself SHALL say so. An append SHALL never be held by
such a copy alone (60 as amended at boomdrivedev/blueprints b64d367, 61,
C.13, C.14).

#### Scenario: An append whose predecessor is not the head

- **WHEN** append is given a predecessor that is not the head of the channel's chain
- **THEN** it is refused naming the claim at the head, and the chain is unchanged

#### Scenario: A chain read as last read

- **WHEN** the source holding a chain the host has read cannot be reached, and the chain is asked
- **THEN** the answer is the claims as last read, the time of that read and the source's words, and an append is unreadable

#### Scenario: An empty chain read as last read

- **WHEN** the host last read a channel's chain holding no claim, and the source holding it then cannot be reached
- **THEN** the answer is a chain of no claims, the time of that read and the source's words, and not absent

#### Scenario: Two components' chains of one name

- **WHEN** a promotion is appended to `beta` of `lib` and the chain of `beta` of `web` is asked
- **THEN** the chain of `beta` of `web` is absent
