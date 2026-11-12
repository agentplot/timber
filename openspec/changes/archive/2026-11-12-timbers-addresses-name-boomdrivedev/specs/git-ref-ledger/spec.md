## MODIFIED Requirements

### Requirement: A remote that does not answer is read as last fetched

Where the remote cannot be reached, chain SHALL answer the claims of
the repository's own copy of the ref as last fetched or appended, saying
when it was last written and carrying git's words of why the remote did
not answer (60, C.13); where the copy last read the channel holding no
claim, chain SHALL answer it empty, saying when it was read and
carrying git's words, as any other chain as last fetched (60, as
amended at boomdrivedev/blueprints b64d367); with no copy of the channel
at all, the chain SHALL be unreadable with git's words. Append SHALL be
unreadable with git's words, and SHALL never append to the copy alone
where a remote is bound.

#### Scenario: The remote gone after a read

- **WHEN** a clone has read `stable`, and its remote's path is then removed
- **THEN** the chain answers the claims it read, the time of that read and git's words, and an append is unreadable

#### Scenario: An empty channel as last fetched

- **WHEN** a clone has read `beta` while its remote held no promotion of it, and its remote's path is then removed
- **THEN** the chain answers no claims, the time of that read and git's words, and not absent

#### Scenario: The keeper's group

- **WHEN** the keeper's conformance group runs over the git ref ledger with a bare remote in a temporary directory
- **THEN** every scenario passes
