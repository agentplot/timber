## ADDED Requirements

### Requirement: The Dash0 target runs the target group

The target group SHALL run over the Dash0 target, in its own tests
against a scripted Dash0, declaring that its target holds a marker. It
SHALL also run by hand against one dataset, under `check-dash0` (93,
rule core.38).

#### Scenario: The scripted Dash0

- **WHEN** the Dash0 target's tests run
- **THEN** the target group passes over the Dash0 target with the scripted Dash0
