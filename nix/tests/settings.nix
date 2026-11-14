# The host-bindings-module spec's settings, each rendered to its file
# under nix/tests/files (record 0035). Between them, laptop and fleet hold
# every key the module declares but `credentials`, each seam's two
# profiles split across them, since a seam's table names one; timber's
# reader reads both (the_modules_files_are_read_as_written).
{
  # "No settings": nothing set.
  empty = { };

  # "A laptop's file": a layout, a plain branch, the ledger on origin, a
  # host key, the host's own publish and one trusted signer.
  laptop = {
    store.layout = "~/.local/share/timber/layout";
    landing.branch = { };
    ledger.git_ref.remote = "origin";
    signing = {
      signer = "host-a";
      key = "~/.config/timber/signing";
    };
    gate.publish = { };
    trust.host-b.ssh = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGt host-b";
  };

  # "A fleet's file": a registry, sluice on origin, the ledger in the
  # store, a key service and a job that tells the gate.
  fleet = {
    store.registry = "registry.example.com/timber";
    landing.sluice.remote = "origin";
    ledger.store = { };
    signing.kms = "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab";
    gate.job = "the release job a push to main wakes";
  };

  # "A host's credentials": two credentials by their paths, and nothing
  # else.
  credentials.credentials = {
    dash0 = "/run/secrets/dash0-token";
    npm = "/run/secrets/npm-token";
  };
}
