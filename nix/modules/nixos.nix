# The NixOS module, nixosModules.default: services.timber's file linked at
# ~/.config/timber/config.toml for services.timber.user, the user who runs
# timber, in that user's own activation, which runs at their manager's
# start and at a switch, and timber in the system profile (record 0035,
# core.76). timber reads the file at its next command, and the serving
# process at its next request, so the module adds no unit, no lingering
# and nothing else for timber to run: the first asker that finds no
# serving process starts it (114, core.36), and timber makes its given
# layout and host key under the data directory itself (core.54).
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

  inherit (cfg) user;
  account =
    config.users.users.${user} or {
      isNormalUser = false;
      isSystemUser = false;
    };
in
{
  imports = [ (import ./options.nix { inherit packages; }) ];

  options.services.timber.user = lib.mkOption {
    type = lib.types.str;
    example = "alice";
    description = "The user who runs timber, whose `~/.config/timber/config.toml` is linked to the module's file in their own activation.";
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = account.isNormalUser || account.isSystemUser;
        message = "services.timber.user is ${user}, whom users.users.${user} does not declare: declare the user who runs timber, in whose home the file is linked.";
      }
    ];

    environment.systemPackages = lib.optional (cfg.package != null) cfg.package;

    # Run by every user's activation; the user who runs timber alone links.
    # A link that fails says why and stops nothing else of the activation.
    system.userActivationScripts.timber.text = ''
      if [ "$(id -un)" = ${lib.escapeShellArg user} ]; then ${pkgs.runtimeShell} ${./activate.sh} link ${file} || true; fi
    '';
  };
}
