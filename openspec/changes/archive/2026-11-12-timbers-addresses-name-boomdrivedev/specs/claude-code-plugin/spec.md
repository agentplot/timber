## MODIFIED Requirements

### Requirement: The plugin is installed from the repository's marketplace

The repository's root SHALL hold a Claude Code marketplace named
`timber` listing one plugin, `timber`, whose source is
`plugins/claude-code`, so `/plugin marketplace add boomdrivedev/timber`
then `/plugin install timber@timber` installs it; the marketplace and
the plugin's manifest SHALL pass the pinned Claude Code's `plugin
validate --strict` in the gate's fast layer (core.25, core.26).

#### Scenario: The manifests validate

- **WHEN** the gate's fast layer runs
- **THEN** the pinned Claude Code validates the marketplace and the plugin with `--strict`, with no user configuration or credential read

#### Scenario: A listed plugin with no manifest

- **WHEN** the marketplace lists a plugin whose source holds no `.claude-plugin/plugin.json`
- **THEN** the marketplace check fails naming the plugin
