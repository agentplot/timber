# services.timber, the options both modules share (record 0035, core.76),
# given the flake's own `packages`: timber's package for the host's
# system, which a host may replace or set to null; and `settings`, the
# host's bindings file, ~/.config/timber/config.toml, under the file's own
# names, free-form TOML with every table and key timber's reader takes
# declared for its type (RFC 42). A table or key left unset is left out of
# the file, so the given binding holds (118); a table set empty, such as
# `landing.branch = { }`, is written, since it binds its profile. A key the
# module does not declare passes to the file, and timber, which refuses a
# table it does not bind, judges it. That a seam's table names one
# profile, that `kms` stands alone, and every given binding are timber's,
# refused or given at its read, naming the table. A signing key and a
# credential are named by their paths outside the store, since the file is
# in the store, which every user reads: one in the store, or one that is
# no path, fails evaluation, and the module reads and copies neither (45,
# 64, 118; record 0033; core.21). boomdrivedev/sluice's
# nix/modules/options.nix at 7ce112b4, for timber's file.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types;
  cfg = config.services.timber;
  toml = (pkgs.formats.toml { }).type;
  render = import ./toml.nix { inherit lib; };

  inherit (pkgs.stdenv.hostPlatform) system;
  flakes =
    packages.${system}.default
      or (throw "timber's flake gives no package for ${system}: it gives ${lib.concatStringsSep ", " (builtins.attrNames packages)}");

  # A key of the file: unset, and so left out, unless the host sets it.
  unset =
    type: description:
    mkOption {
      type = types.nullOr type;
      default = null;
      inherit description;
    };

  # A table of the file, free-form beside the keys it declares.
  table =
    options:
    types.submodule {
      freeformType = toml;
      inherit options;
    };

  # What is unset taken out before rendering: every null at any depth, and
  # nothing else. A table left with no key stays, since `branch = { }`
  # binds its profile with no options; and a list stays as written.
  prune =
    value:
    if builtins.isAttrs value then
      lib.filterAttrs (_: v: v != null) (builtins.mapAttrs (_: prune) value)
    else if builtins.isList value then
      map prune value
    else
      value;

  # A path's failures: none where it is unset, or a path, `/…` or `~/…`,
  # outside the store.
  outside =
    option: put: path:
    lib.optionals (path != null) [
      {
        assertion = lib.hasPrefix "/" path || lib.hasPrefix "~/" path;
        message = "${option} is no path, /… or ~/…, and the file that names it is in the Nix store, which every user reads: ${put}";
      }
      {
        assertion = !lib.hasPrefix (builtins.storeDir + "/") path;
        message = "${option} is in the Nix store, which every user reads: ${put}";
      }
    ];
  key = if cfg.settings.signing == null then null else cfg.settings.signing.key;
  keyOutside =
    outside "services.timber.settings.signing.key"
      "put the key in a file on the host, readable by the user who runs timber alone, by hand or by the host's handling of secrets, and name its path (118, C.8)"
      key;
  credentialsOutside = lib.concatLists (
    lib.mapAttrsToList (
      name:
      outside "services.timber.settings.credentials.${name}" "put the credential in a file on the host, readable by the user who runs timber alone, by hand or by the host's handling of secrets, and name its path (118)"
    ) (if cfg.settings.credentials == null then { } else cfg.settings.credentials)
  );

  remote = unset types.str "The remote the landing reads, by its name in the checkout. Where unset, the repository's own refs.";
in
{
  options.services.timber = {
    enable = lib.mkEnableOption "timber's bindings file, linked for the user who runs timber, and its package";

    package = mkOption {
      type = types.nullOr types.package;
      default = flakes;
      defaultText = lib.literalExpression "timber.packages.\${pkgs.stdenv.hostPlatform.system}.default";
      description = "The timber the system profile holds: timber's flake's for the host's system. `null` installs none, for a host that installs timber its own way.";
    };

    settings = mkOption {
      type = table {
        store =
          unset
            (table {
              layout = unset types.str "An OCI layout's path, absolute or under `~/`, made by the first publish or push where it is missing (C.2).";
              registry = unset types.str "One repository of a registry, `<host>/<repository>`, such as `localhost:5000/timber`, the declared repository the store's spaces hang under. The credential is never the file's: it is the host's Docker configuration and the credential helper it names for the registry, reached when the registry asks and kept nowhere (C.1, 64).";
            })
            "`[store]`: the store timber writes into and reads from, named by its one key, `layout` or `registry`. Where unset, an OCI layout at `timber/layout` under the user's data directory (118, C.2).";
        landing =
          unset
            (table {
              sluice = unset (table {
                inherit remote;
              }) "`sluice = { }`: sluice's landing, its records read from `remote` (C.5).";
              branch = unset (table {
                inherit remote;
              }) "`branch = { }`: a plain branch's landing, its refs read from `remote` (C.6).";
            })
            "`[landing]`: what says a revision landed, named by its one key, `sluice` or `branch`. Where unset, sluice where the repository's `HEAD` commit holds `.config/sluice.toml`, and a plain branch otherwise, over the checkout's `origin` or its one remote (118, C.5, C.6).";
        ledger =
          unset
            (table {
              git_ref =
                unset
                  (table {
                    remote = unset types.str "The remote the chain is pushed to and fetched from, with the host's own git credentials. Where unset, the repository's own refs.";
                  })
                  "`git_ref = { }`: each channel's chain in the repository's refs, `refs/timber/ledger/<channel>` (C.13).";
              store =
                unset (table { })
                  "`store = { }`: each channel's chain held as the promotion claims in the store `[store]` names (C.14).";
            })
            "`[ledger]`: where each channel's chain is held, named by its one key, `git_ref` or `store`. Where unset, the git ref on the landing's remote, or the repository's own refs where the landing has none (118, C.13).";
        signing =
          unset
            (table {
              signer = unset types.str "The name the host signs as, beside `key` (C.8).";
              key = unset types.str "The path of the signer's SSH private key on this host, absolute or under `~/`, outside the Nix store, which every user reads (C.8).";
              kms = unset types.str "A key in AWS KMS by its key ARN, alone in the table: the signer's name is the ARN. The credential is the host's AWS chain, found at each signature and kept nowhere (C.7, 64).";
            })
            "`[signing]`: the host's signer, `signer` and its `key`, or a key in AWS KMS, `kms`, alone. Where unset, for a publish, a promote or an apply, the host key at `timber/host/signing.key` under the user's data directory, made by the first claim it signs (118, C.8).";
        gate =
          unset
            (table {
              publish =
                unset (table { })
                  "`publish = { }`: this host's own publish tells its gate of each version it offered, in the same run (130).";
              job = unset types.str "The words naming a job outside timber that a forge's or a store's event wakes; timber runs nothing for it, and a publish says the gate is told by those words (130, C.16).";
            })
            "`[gate]`: what tells the host's gate of a version a publish offered, by one key, `publish` or `job`. Where unset, the host's own publish (118, 130, C.16).";
        trust = unset (types.attrsOf (types.attrsOf types.str)) "`[trust.<signer>]`: each signer the host trusts, by its name, a table of the one key its profile verifies it by, such as `ssh`, a public key and never a credential (118, C.10). Where unset, the host trusts its own signer alone.";
        credentials = unset (types.attrsOf types.str) "`[credentials]`: each credential by its name, as an environment's target table names it, the path of a file on the host that holds that credential alone, absolute or under `~/`, outside the Nix store, which every user reads. timber hands the path to the target's own tool, which alone reads it; the module reads and copies no credential (118; record 0033). Where unset, the host names no credential.";
      };
      default = { };
      description = "The host's bindings, `~/.config/timber/config.toml`, each table and key named as the file names it, rendered at evaluation. A table or key left unset is left out of the file, so the given binding holds (118); a table set empty, such as `landing.branch = { }`, is written, since it binds its profile; a key the module does not declare is written as given, for timber to judge. The file is in the Nix store, which every user reads: a signing key and a credential are named by their paths on the host, never held here.";
    };

    text = mkOption {
      type = types.str;
      internal = true;
      readOnly = true;
      description = "The file's text, rendered from `settings` with what is unset taken out, which each platform's module writes.";
    };
  };

  config = lib.mkMerge [
    { services.timber.text = render (prune cfg.settings); }
    (lib.mkIf cfg.enable { assertions = keyOutside ++ credentialsOutside; })
  ];
}
