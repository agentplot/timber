## MODIFIED Requirements

### Requirement: The suite knows the clauses of Parts A and B

The suite SHALL hold the numbers of every clause of Parts A and B, by
section, with the statement's revision they were read at. A clause the
statement adds or removes SHALL change this list in the commit that
adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list of clauses is read
- **THEN** it holds every number from 1 to 144 exactly once, each under its section of Part A or Part B as boomdrivedev/blueprints a8234b3 places it, 141, 142 and 143 after 13 in A.2 and 144 after 20 in A.3, and names that revision as the one it was read at

## ADDED Requirements

### Requirement: The clause the statement added at 144 waits named

While no scenario cites 144, it SHALL be named in the register from
the commit that adopts the statement holding it, with phase 2's link of
a publish and its reason (rule core.33, section 12).

#### Scenario: The register after a8234b3 is adopted

- **WHEN** the coverage check runs after the commit that adopts boomdrivedev/blueprints a8234b3
- **THEN** 144 is pending with phase 2's link of a publish and its reason, 141 to 143 are pending as before, and the check passes
