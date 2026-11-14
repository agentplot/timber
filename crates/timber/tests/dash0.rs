//! Spec dash0-target, its run against one dataset (design D9): the
//! throwaway dataset the host names in `env.TIMBER_TOOLS_DASH0` of its
//! `devenv.local.nix`, as `<api>#<dataset>#<token file>`, where `<api>` is
//! the address of Dash0's API, `<dataset>` a dataset no one uses, and
//! `<token file>` a file holding only a token that may write dashboards to
//! it, which a person makes. The CLI is the one the tree pins,
//! `TIMBER_TOOLS_DASH0_CLI`, never the `dash0` on a person's path.
//!
//! Three versions of one component are published into a layout in the
//! test's directory, each output's dashboard file packed by the file
//! projection as a publish packs it: A holding `timber-proof-a` and
//! `timber-proof-b`, B `timber-proof-a` and `timber-proof-c`, C
//! `timber-proof-a` alone. `timber-proof-hand` is made with the CLI
//! directly, as a person makes one in Dash0. The target group runs over
//! the profile; then A, B, A and C are applied, the dataset's dashboards,
//! current and report checked after each: after B, `-b` is gone; after A
//! again, `-c` is gone; and `-hand` stays throughout. The marker deleted
//! leaves current unread. Every dashboard the run made is deleted at its
//! end, whatever it answered.
//!
//! Its token is a file a person makes, so it runs by hand under
//! `check-dash0` and in no gate task (rule core.38), and it refuses to
//! run, naming the setting, on a host that names no dataset, rather than
//! pass without running.

#![cfg(feature = "tools")]

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output as Ran, Stdio};
use std::rc::Rc;

use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use tempfile::TempDir;
use timber::bindings::Credentials;
use timber::profiles::projection::file::FileProjection;
use timber::profiles::store::layout::Layout;
use timber::profiles::target::dash0::{self, Dash0};
use timber_conformance::seams::target::{self, TargetRig};
use timber_conformance::Rig;
use timber_core::paths_hold;
use timber_seams::{
    Answer, Artifact, BuildId, ChannelId, ChannelName, Component, ComponentName, Digest, Ecosystem,
    EcosystemName, Envelope, Environment, EnvironmentName, Held, IndexMeta, Manifest, Name, Not,
    Output, OutputName, PackAt, Projection, Put, RepositoryName, Revision, Store, Target, Verified,
    Version,
};

/// The setting a host names its throwaway dataset by.
const SETTING: &str = "TIMBER_TOOLS_DASH0";

/// The setting naming the CLI the tree pins.
const CLI: &str = "TIMBER_TOOLS_DASH0_CLI";

/// The component the versions are of, and its one output, a dashboard
/// file.
const COMPONENT: &str = "timber-proof";
const OUTPUT: &str = "dashboards";

/// The credential's name, as the environments' tables give it.
const CREDENTIAL: &str = "timber-proof";

/// A credential whose file is not there, which the unreadable environment
/// names.
const NO_TOKEN: &str = "timber-proof-no-token";

/// The dashboards the versions hold, and the one a person makes.
const A: &str = "timber-proof-a";
const B: &str = "timber-proof-b";
const C: &str = "timber-proof-c";
const HAND: &str = "timber-proof-hand";

/// The repository the versions are published from, as the markers are
/// named for it, so no marker of the run meets one of a real repository's.
fn repository() -> RepositoryName {
    RepositoryName {
        owner: "timber-tools".into(),
        name: "dash0".into(),
    }
}

/// The dataset the host names: its API's address, the dataset, and the
/// token's file; a panic naming the setting where it names none.
struct Dataset {
    api: String,
    dataset: String,
    token: PathBuf,
}

fn setting() -> Dataset {
    let named = std::env::var(SETTING).unwrap_or_default();
    let parts: Vec<&str> = named.trim().splitn(3, '#').collect();
    match parts.as_slice() {
        [api, dataset, token] if !api.is_empty() && !dataset.is_empty() && !token.is_empty() => {
            Dataset {
                api: api.to_string(),
                dataset: dataset.to_string(),
                token: PathBuf::from(token),
            }
        }
        _ => panic!(
            "the Dash0 run names no dataset: set env.{SETTING} in devenv.local.nix to \
             `<api>#<dataset>#<token file>`, the address of Dash0's API, a throwaway dataset no \
             one uses, and a file holding only a token that may write dashboards to it"
        ),
    }
}

/// The CLI the tree pins; a panic naming its setting where the shell
/// names none.
fn cli() -> PathBuf {
    match std::env::var_os(CLI) {
        Some(cli) if !cli.is_empty() => PathBuf::from(cli),
        _ => panic!("{CLI} names no dash0: run the proof in the repository's devenv shell"),
    }
}

/// The identifier of the marker of `environment`, as the profile derives
/// it.
fn marker_id(environment: &str) -> String {
    let repository = repository();
    let named = format!(
        "{}/{}/{COMPONENT}/{environment}",
        repository.owner, repository.name
    );
    let hash: String = Sha256::digest(named.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    format!("timber-{}", &hash[..32])
}

/// A dashboard of identifier `id`, as a repository's file holds one.
fn dashboard(id: &str) -> String {
    format!(
        "kind: Dashboard\n\
         metadata:\n  name: {id}\n  dash0Extensions:\n    id: {id}\n\
         spec:\n  display:\n    name: {id}\n    description: made by timber's proof against \
         Dash0, and removed by it\n  duration: 1h\n  panels: {{}}\n  layouts: []\n"
    )
}

/// The dashboard file of a version holding `ids`, one document each.
fn dashboard_file(ids: &[&str]) -> String {
    ids.iter()
        .map(|id| dashboard(id))
        .collect::<Vec<_>>()
        .join("---\n")
}

/// The dashboard file holding `ids`, written under `dir`, packed by the
/// file projection at `version` and put in `store` as a publish puts it:
/// its bytes, its artifact, and the version's index naming it, the
/// revision and the repository; the version verified, as the core makes
/// it after its verify (42).
fn publish(store: &Layout, dir: &Path, version: &str, ids: &[&str]) -> Verified {
    let file = dir.join(format!("dashboards-{version}.yaml"));
    fs::write(&file, dashboard_file(ids)).unwrap();
    let output = Output {
        name: OutputName::new(OUTPUT),
        files: vec![file.display().to_string()],
    };
    let component = Component {
        name: ComponentName::new(COMPONENT),
        outputs: vec![output.name.clone()],
        paths: vec!["dashboards/".into()],
        prefix: format!("{COMPONENT}-v"),
    };
    let holds = paths_hold(&component.paths).unwrap();
    let version = Version::new(version);
    let revision = "4f2a9c1";
    let packed = FileProjection::new()
        .pack(
            &output,
            &BuildId::new(revision),
            Some(&PackAt {
                version: &version,
                component: &component,
                outputs: std::slice::from_ref(&output),
                holds: &holds,
            }),
        )
        .unwrap_or_else(|not| panic!("the dashboard file packed: {not:?}"));
    let meta = packed.meta.expect("the file projection names its artifact");
    let layer = store.put_bytes(&packed.bytes).unwrap().digest;
    let digest = store
        .put_artifact(&Manifest::Artifact {
            meta: meta.clone(),
            layers: vec![layer],
        })
        .unwrap()
        .digest;
    let index = IndexMeta::new(
        component.name.clone(),
        version.clone(),
        vec![Artifact { digest, meta }],
    )
    .unwrap()
    .published_from(Revision::new(revision), Some(repository()));
    let index = store.put_artifact(&Manifest::Index(index)).unwrap().digest;
    Verified::new(index, version)
}

/// A layout that answers a read of a digest it was told stands for
/// another with what that other holds, so the suite can hand the target a
/// version at a digest of its own choosing.
#[derive(Clone)]
struct Aliased {
    layout: PathBuf,
    aliases: Rc<RefCell<BTreeMap<Digest, Digest>>>,
}

impl Aliased {
    fn store(&self) -> Layout {
        Layout::open(&self.layout).expect("the layout the test published into")
    }
}

impl Store for Aliased {
    fn put_bytes(&self, bytes: &[u8]) -> Answer<Put> {
        self.store().put_bytes(bytes)
    }
    fn put_artifact(&self, manifest: &Manifest) -> Answer<Put> {
        self.store().put_artifact(manifest)
    }
    fn attach(&self, subject: &Digest, attestation: &Envelope) -> Answer<Put> {
        self.store().attach(subject, attestation)
    }
    fn claims_of(&self, digest: &Digest) -> Answer<Vec<Envelope>> {
        self.store().claims_of(digest)
    }
    fn name(&self, name: &Name, to: &Digest, expected: Option<&Digest>) -> Answer<()> {
        self.store().name(name, to, expected)
    }
    fn resolve(&self, name: &Name) -> Answer<Digest> {
        self.store().resolve(name)
    }
    fn read(&self, digest: &Digest) -> Answer<Held> {
        let to = self.aliases.borrow().get(digest).cloned();
        self.store().read(to.as_ref().unwrap_or(digest))
    }
    fn versions(&self, ecosystem: &Ecosystem, name: &EcosystemName) -> Answer<Vec<Version>> {
        self.store().versions(ecosystem, name)
    }
}

/// The dataset the run stands on: the setting, the three versions
/// published, the target, and every dashboard the run made, deleted when
/// it is dropped.
struct Proof {
    dir: TempDir,
    dataset: Dataset,
    versions: [Verified; 3],
    store: Aliased,
    target: Dash0,
    made: RefCell<Vec<String>>,
}

impl Proof {
    fn new() -> Proof {
        let dataset = setting();
        let cli = cli();
        let dir = TempDir::new().unwrap();
        let layout = dir.path().join("layout");
        let made = Layout::create(&layout).expect("a layout made");
        let versions = [
            publish(&made, dir.path(), "0.1.0", &[A, B]),
            publish(&made, dir.path(), "0.2.0", &[A, C]),
            publish(&made, dir.path(), "0.3.0", &[A]),
        ];
        let store = Aliased {
            layout,
            aliases: Rc::default(),
        };
        let credentials = Credentials {
            file: dir.path().join("config.toml"),
            paths: BTreeMap::from([
                (CREDENTIAL.to_string(), dataset.token.clone()),
                (NO_TOKEN.to_string(), dir.path().join("no-token")),
            ]),
        };
        let target =
            Dash0::with_program(Box::new(store.clone()), Ok(repository()), &credentials, cli);
        Proof {
            dir,
            dataset,
            versions,
            store,
            target,
            made: RefCell::new(vec![A.into(), B.into(), C.into(), HAND.into()]),
        }
    }

    /// The environment `name`, its marker deleted with the run's
    /// dashboards at the end.
    fn environment(&self, name: &str, credential: &str) -> Environment {
        self.made.borrow_mut().push(marker_id(name));
        Environment {
            name: EnvironmentName::new(name),
            channel: ChannelId::new(
                Some(ComponentName::new(COMPONENT)),
                ChannelName::new("stable"),
            ),
            waits: false,
            settings: json!({
                "api": self.dataset.api,
                "dataset": self.dataset.dataset,
                "credential": credential,
                "dashboards": [OUTPUT],
            }),
        }
    }

    /// The CLI the tree pins run with `subcommand`, as the profile runs it:
    /// an empty configuration folder of its own, no `DASH0_*` variable,
    /// and the token by its path, `token` where it is given, else the
    /// host's.
    fn dash0(&self, token: Option<&Path>, subcommand: &[&str]) -> Ran {
        let config = TempDir::new().unwrap();
        let mut command = Command::new(cli());
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("DASH0_") {
                command.env_remove(name);
            }
        }
        command
            .args(["--agent-mode", "--no-skill-hint", "--config-dir"])
            .arg(config.path())
            .arg("--auth-token-file")
            .arg(token.unwrap_or(&self.dataset.token))
            .args(subcommand)
            .args([
                "--api-url",
                &self.dataset.api,
                "--dataset",
                &self.dataset.dataset,
            ])
            .stdin(Stdio::null())
            .output()
            .unwrap_or_else(|e| panic!("{} could not be started: {e}", cli().display()))
    }

    /// Whether the dataset holds the dashboard `id`, not deleted.
    fn holds(&self, id: &str) -> bool {
        let ran = self.dash0(None, &["dashboards", "get", id, "-o", "json"]);
        if !ran.status.success() {
            let words = String::from_utf8_lossy(&ran.stderr);
            assert!(
                words.contains(&format!("dashboard \\\"{id}\\\" not found"))
                    || words.contains(&format!("dashboard \"{id}\" not found")),
                "dashboards get {id}: {words}"
            );
            return false;
        }
        let held: Value = serde_json::from_slice(&ran.stdout).unwrap();
        held["metadata"]["annotations"]["dash0.com/deleted-at"].is_null()
    }

    /// The dashboard `HAND` made with the CLI directly, as a person makes
    /// one.
    fn made_by_hand(&self) {
        let file = self.dir.path().join("hand.yaml");
        fs::write(&file, dashboard(HAND)).unwrap();
        let ran = self.dash0(None, &["apply", "-f", file.to_str().unwrap()]);
        assert!(
            ran.status.success(),
            "the dashboard made by hand: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
    }

    /// The dataset holds `held` and none of `gone`.
    fn dataset_holds(&self, held: &[&str], gone: &[&str], after: &str) {
        for id in held {
            assert!(self.holds(id), "after {after}, the dataset lacks {id}");
        }
        for id in gone {
            assert!(!self.holds(id), "after {after}, the dataset holds {id}");
        }
    }
}

impl Drop for Proof {
    /// Every dashboard the run made deleted, whatever it answered.
    fn drop(&mut self) {
        for id in self.made.borrow().iter() {
            let ran = self.dash0(None, &["dashboards", "delete", id, "--force"]);
            if !ran.status.success() {
                eprintln!(
                    "the dashboard {id} was not deleted: {}",
                    String::from_utf8_lossy(&ran.stderr)
                );
            }
        }
    }
}

/// The target group's rig over the profile and the dataset: each
/// environment is one of its own in the dataset, the unreadable one names
/// a credential whose file is not there, whose words the rig reads from
/// the CLI itself, and the foreign settings add a `token`.
struct Dash0Rig {
    proof: Rc<Proof>,
    environments: Cell<u32>,
}

impl Dash0Rig {
    fn next(&self, credential: &str) -> Environment {
        self.environments.set(self.environments.get() + 1);
        self.proof
            .environment(&format!("group-{}", self.environments.get()), credential)
    }
}

impl Rig for Dash0Rig {
    fn profile(&self) -> String {
        dash0::PROFILE.into()
    }
}

impl TargetRig for Dash0Rig {
    fn target(&self) -> &dyn Target {
        &self.proof.target
    }

    /// Version A, read at `digest`.
    fn verified(&self, digest: &Digest) -> Verified {
        let a = &self.proof.versions[0];
        let mut aliases = self.proof.store.aliases.borrow_mut();
        aliases.insert(digest.clone(), a.index().clone());
        Verified::new(digest.clone(), a.version().clone())
    }

    fn environment(&self, _head: Option<&Digest>) -> Environment {
        self.next(CREDENTIAL)
    }

    fn unreadable_environment(&self) -> (Environment, String) {
        let environment = self.next(NO_TOKEN);
        let missing = self.proof.dir.path().join("no-token");
        let ran = self.proof.dash0(
            Some(&missing),
            &[
                "dashboards",
                "get",
                &marker_id(environment.name.as_str()),
                "-o",
                "json",
            ],
        );
        assert!(!ran.status.success(), "dash0 read a token from no file");
        let words = String::from_utf8_lossy(&ran.stderr).trim().to_string();
        (environment, words)
    }

    fn foreign_settings(&self) -> (Environment, String) {
        let mut environment = self.next(CREDENTIAL);
        environment.settings["token"] = json!("auth_not_a_token");
        (environment, "token".into())
    }
}

#[test]
fn the_target_group_passes_and_a_dataset_converges_on_a_b_a_and_c_never_losing_a_hand_made_one() {
    let proof = Rc::new(Proof::new());
    proof.made_by_hand();
    assert!(proof.holds(HAND), "the dashboard made by hand is not held");

    let report = timber_conformance::run(target::GROUP, &target::scenarios(), &|| {
        Box::new(Dash0Rig {
            proof: Rc::clone(&proof),
            environments: Cell::new(0),
        }) as Box<dyn TargetRig>
    });
    report.assert();
    let inapplicable: Vec<&str> = report.not_applicable.iter().map(|n| n.scenario).collect();
    assert_eq!(
        inapplicable,
        ["applied where no marker is held"],
        "{report}"
    );

    let environment = proof.environment("proof", CREDENTIAL);
    let marker = marker_id("proof");
    let steps: [(usize, &[&str], &[&str]); 4] = [
        (0, &[A, B, HAND], &[]),
        (1, &[A, C, HAND], &[B]),
        (0, &[A, B, HAND], &[C]),
        (2, &[A, HAND], &[B, C]),
    ];
    for (n, held, gone) in steps {
        let version = &proof.versions[n];
        let named = format!("the apply of {}", version.version());
        proof
            .target
            .apply(&environment, version)
            .unwrap_or_else(|not| panic!("{named}: {not:?}"));
        proof.dataset_holds(held, gone, &named);
        assert!(proof.holds(&marker), "after {named}, no marker is held");
        assert_eq!(
            proof.target.current(&environment),
            Ok(version.index().clone()),
            "{named}"
        );
        let running = proof
            .target
            .report(&environment)
            .unwrap_or_else(|not| panic!("report after {named}: {not:?}"));
        assert_eq!(&running.digest, version.index(), "{named}");
        assert!(!running.converged_at.is_empty(), "{named}");
    }

    let ran = proof.dash0(None, &["dashboards", "delete", &marker, "--force"]);
    assert!(
        ran.status.success(),
        "the marker deleted: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    match proof.target.current(&environment) {
        Err(Not::Unreadable(u)) => {
            let said = u.said.unwrap_or_default();
            assert!(
                said.contains("was deleted at") || said.contains("holds no marker"),
                "{said}"
            );
        }
        other => panic!("current with the marker deleted answered {other:?}"),
    }
}
