# The host-bindings-module spec's file (record 0035), over options.nix
# evaluated alone: each of the spec's settings renders the text of its
# file under nix/tests/files, byte for byte; a table set empty is written
# as its header; a key the module does not declare is written as given; a
# value the file cannot hold fails, naming its option; and a signing key
# or a credential in the store, holding a secret's text or given as a Nix
# path fails evaluation, naming its option and saying the store is read
# by every user. boomdrivedev/sluice's nix/tests/file.nix at 7ce112b4, for
# timber's file.
t:
let
  inherit (t) lib;
  settings = import ./settings.nix;
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;
  header = "# Written by timber's flake module from services.timber.settings; a change goes there.";

  # options.nix enabled with `given`, beside a system's `assertions`.
  evaluated =
    given:
    (lib.evalModules {
      modules = [
        (import ../modules/options.nix { inherit (t.outputs) packages; })
        {
          options.assertions = lib.mkOption {
            type = lib.types.listOf lib.types.unspecified;
            default = [ ];
          };
        }
        {
          _module.args = { inherit pkgs; };
          services.timber = {
            enable = true;
            settings = given;
          };
        }
      ];
    }).config;

  # The messages of the assertions `given` fails.
  failures = given: map (a: a.message) (lib.filter (a: !a.assertion) (evaluated given).assertions);
  says = given: words: lib.hasInfix words (toString (failures given));

  # The text the module writes for `given`.
  text = given: (evaluated given).services.timber.text;

  # The text of `name`'s settings, held to its file.
  rendersAs =
    name:
    t.expect (text settings.${name} == builtins.readFile ./files/${name}.toml)
      "${name}'s settings render ${
        builtins.toJSON (text settings.${name})
      }, not nix/tests/files/${name}.toml"
    ++
      t.expect (failures settings.${name} == [ ])
        "${name}'s settings fail: ${toString (failures settings.${name})}";

  # A float, which no key of the file takes. Evaluation can catch a
  # failure but not read its words, so the words are read from the
  # renderer imported with its `throw` writing them into the text.
  float = {
    store.share = 1.5;
  };
  cannot = builtins.tryEval (builtins.seq (text float) true);
  shown = builtins.scopedImport { throw = words: "<failure: ${words}>"; } ../modules/toml.nix {
    inherit lib;
  } (evaluated float).services.timber.settings;

  # A path under the store directory, and a secret's own text.
  inStore = "${builtins.storeDir}/0000000000000000000000000000000-secret";
  keyIn = {
    signing.key = inStore;
  };
  keyText = {
    signing.key = "-----BEGIN OPENSSH PRIVATE KEY-----";
  };
  credentialIn = {
    credentials.dash0 = inStore;
  };
  credentialText = {
    credentials.dash0 = "auth_0123456789abcdef";
  };

  # A Nix path value, a file that is there, which a string would copy into
  # the store.
  keyPath = {
    signing.key = ./settings.nix;
  };
  credentialPath = {
    credentials.dash0 = ./settings.nix;
  };

  refused =
    given: option: words:
    t.expect (failures given != [ ]) "${option} evaluates"
    ++ lib.concatMap (
      w: t.expect (says given w) "the failure names no ${w}: ${toString (failures given)}"
    ) ([ option ] ++ words);
in
{
  "No settings" =
    rendersAs "empty"
    ++ t.expect (
      text settings.empty == "${header}\n"
    ) "the empty settings render more than the header: ${builtins.toJSON (text settings.empty)}";

  "A laptop's file" = rendersAs "laptop";

  "A fleet's file" = rendersAs "fleet";

  "A host's credentials" = rendersAs "credentials";

  "A table set empty" =
    let
      given = {
        landing.branch = { };
      };
    in
    t.expect (
      text given == "${header}\n\n[landing.branch]\n"
    ) "landing.branch = { } alone renders ${builtins.toJSON (text given)}";

  "A key the module does not declare" =
    let
      given = {
        store = {
          later = "soon";
          layout = "~/.local/share/timber/layout";
        };
      };
    in
    t.expect (lib.hasInfix ''
      [store]
      later = "soon"
      layout = "~/.local/share/timber/layout"
    '' (text given)) "the file's [store] is not later, then layout: ${builtins.toJSON (text given)}";

  "A value the file cannot hold" =
    t.expect (!cannot.success) "a file with settings.store.share = 1.5 evaluates"
    ++ t.expect (lib.hasInfix "<failure: services.timber.settings.store.share is a float" shown) "the failure names no services.timber.settings.store.share: ${builtins.toJSON shown}";

  "A signing key in the store" = refused keyIn "services.timber.settings.signing.key" [
    "is in the Nix store, which every user reads"
  ];

  "A signing key's text" = refused keyText "services.timber.settings.signing.key" [
    "no path"
    "Nix store, which every user reads"
  ];

  "A credential in the store" = refused credentialIn "services.timber.settings.credentials.dash0" [
    "is in the Nix store, which every user reads"
  ];

  "A credential's text" = refused credentialText "services.timber.settings.credentials.dash0" [
    "no path"
    "Nix store, which every user reads"
  ];

  "A key given as a Nix path" = refused keyPath "services.timber.settings.signing.key" [
    "is a Nix path value"
    "Nix store, which every user reads"
  ];

  "A credential given as a Nix path" =
    refused credentialPath "services.timber.settings.credentials.dash0"
      [
        "is a Nix path value"
        "Nix store, which every user reads"
      ];
}
