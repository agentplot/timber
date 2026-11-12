## MODIFIED Requirements

### Requirement: A channel is read from the ledger alone

The channel's state SHALL be answered from its chain through the ledger
seam alone: its head's version and index, the version it moved from,
and every promotion in order with what its claim records; a head that
holds no promotion claim of timber's form SHALL be unreadable naming
it. Where the ledger answered the chain as last read, the answer SHALL
say when and carry the source's words, an empty channel as any other
(60 as amended at boomdrivedev/blueprints b64d367). Reading a channel
SHALL ask the store nothing and write nothing (101, 60, core.32).

#### Scenario: A promotion read on another machine

- **WHEN** a promotion is made on one machine through a git ref ledger, and a second clone with an empty store reads the channel
- **THEN** the second clone answers the same head and the same chain, and its store is asked nothing

#### Scenario: A channel never promoted

- **WHEN** a channel whose chain is empty is read
- **THEN** the answer is that the channel is empty, which is an answer and not an error (31)

#### Scenario: An empty channel read as last read

- **WHEN** the ledger answers a channel's chain as last read, holding no claim, with the time of that read and its words
- **THEN** the answer is that the channel is empty, with that time and those words, and status shows it `empty, as last read` at that time with the words
