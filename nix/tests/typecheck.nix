# nix:typecheck: everything the flake gives evaluates, each module in a
# system of its platform (record 0035).
t:
let
  inherit (t) lib outputs systems;

  # A system's `default`: a derivation whose path evaluates.
  evaluates =
    system:
    let
      package = outputs.packages.${system}.default or null;
      drvPath = builtins.tryEval (builtins.seq package.drvPath true);
    in
    if !lib.isDerivation package then
      [ "packages.${system}.default is not a derivation" ]
    else
      t.expect drvPath.success "packages.${system}.default does not evaluate to a derivation";
in
{
  "Every system's package" =
    t.expect (
      builtins.attrNames outputs.packages == lib.sort lib.lessThan systems
    ) "packages gives ${toString (builtins.attrNames outputs.packages)}, not ${toString systems}"
    ++ lib.concatMap (
      system: if outputs.packages ? ${system} then evaluates system else [ "packages gives no ${system}" ]
    ) systems;
}
