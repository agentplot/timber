# The nix-darwin module, darwinModules.default: services.timber's file
# linked at ~/.config/timber/config.toml for nix-darwin's primary user,
# the user who runs timber, and timber in the system profile (record 0035,
# core.76). timber reads the file at its next command, and the serving
# process at its next request, so the module adds no agent, no daemon and
# nothing else for timber to run: the first asker that finds no serving
# process starts it (114, core.36), and timber makes its given layout and
# host key under the data directory itself (core.54).
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.timber;
  file = pkgs.writeText "timber-config.toml" cfg.text;

  user = config.system.primaryUser;
  home = if user == null then null else config.users.users.${user}.home or null;
in
{
  imports = [ (import ./options.nix { inherit packages; }) ];

  config = lib.mkIf cfg.enable {
    system.requiresPrimaryUser = [ "services.timber.enable" ];

    assertions = [
      {
        assertion = user == null || home != null;
        message = "services.timber links ~/.config/timber/config.toml for nix-darwin's primary user ${toString user}, whose home users.users.${toString user}.home is not set: set it, as the home the file is linked in.";
      }
    ];

    environment.systemPackages = lib.optional (cfg.package != null) cfg.package;

    # Activation runs as root, with root's HOME, so the link is made as the
    # primary user under their own home. A link that fails says why and
    # stops nothing else of the system's activation.
    system.activationScripts.postActivation.text = ''
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} /bin/sh ${./activate.sh} link ${file} || true
    '';
  };
}
