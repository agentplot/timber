## MODIFIED Requirements

### Requirement: A store that does not answer is read as last read

Each chain this host reads, an empty one among them, SHALL be kept,
with when it was read, under the repository's common git directory
beside the record of published versions (61, 62). Where the store
cannot be reached, chain SHALL answer the chain as kept, saying when it
was read and carrying the store's words, an empty chain kept as an
empty chain with that time and those words (60, as amended at
boomdrivedev/blueprints b64d367, C.14); with nothing kept, the chain SHALL
be unreadable with the store's words. Append SHALL be unreadable with
the store's words. What is kept SHALL hold no credential and nothing
the store holds that is not the chain (64).

#### Scenario: The store gone after a read

- **WHEN** a host has read `stable`'s chain from the layout, and the layout is then made unreadable
- **THEN** the chain answers the claims read, the time of the read and the store's words

#### Scenario: An empty channel as last read

- **WHEN** a host has read `beta`'s chain while the layout held no promotion of it, and the layout is then made unreadable
- **THEN** the chain answers no claims, the time of the read and the store's words, and not absent

#### Scenario: The keeper's group

- **WHEN** the keeper's conformance group runs over the store ledger bound to an OCI layout in a temporary directory
- **THEN** every scenario passes
