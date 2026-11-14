//! The Dash0 target over a scripted Dash0: its reads, its refusals, its
//! applies and the target group (design D9).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

use serde_json::{json, Value};
use timber_fakes::FakeStore;
use timber_seams::{
    Artifact, ArtifactMeta, ChannelId, ChannelName, ComponentName, Ecosystem, EcosystemName,
    Envelope, EnvironmentName, Held as Holds, IndexMeta, Manifest, Name, Put, Version,
};

use super::scripted::{Asked, Scripted, CLOCK};
use super::*;

const COMPONENT: &str = "dashboards";
const API: &str = "https://api.us-east-1.aws.dash0.com";
const DATASET: &str = "deneb";
const CREDENTIAL: &str = "dash0-dashboards";
const TOKEN: &str = "/run/secrets/dash0-dashboards";

fn repository() -> RepositoryName {
    RepositoryName {
        owner: "madswan-dev".into(),
        name: "dashboards".into(),
    }
}

fn environment(name: &str, settings: Value) -> Environment {
    Environment {
        name: EnvironmentName::new(name),
        channel: ChannelId::new(
            Some(ComponentName::new(COMPONENT)),
            ChannelName::new("stable"),
        ),
        waits: false,
        settings,
    }
}

/// The table of `deneb`, applying the outputs `dashboards`.
fn table(dashboards: &[&str]) -> Value {
    json!({
        "api": API,
        "dataset": DATASET,
        "credential": CREDENTIAL,
        "dashboards": dashboards,
    })
}

fn deneb() -> Environment {
    environment("deneb", table(&[COMPONENT]))
}

/// The marker of `environment`.
fn marker_of(environment: &Environment) -> Marker {
    Marker::of(&repository(), environment)
}

/// A dashboard of identifier `id`, as a test's stand-in file holds one.
fn dashboard(id: &str) -> Value {
    json!({ "kind": "Dashboard", "name": format!("The {id}"), "id": id })
}

/// A stand-in dashboard file holding `documents`.
fn file(documents: &[Value]) -> String {
    Value::from(documents.to_vec()).to_string()
}

fn index(n: u8) -> Digest {
    Digest::of(&[b'i', n])
}

/// The host's credentials, naming the path of the token's file.
fn credentials() -> Credentials {
    Credentials {
        file: PathBuf::from("/home/a/.config/timber/config.toml"),
        paths: BTreeMap::from([(CREDENTIAL.to_string(), PathBuf::from(TOKEN))]),
    }
}

/// A store that answers a read of a digest it was told stands for another
/// with what that other holds, so the target group can hand the target a
/// version at a digest of its own choosing, as the core hands one after
/// its verify.
#[derive(Clone, Default)]
struct Aliased {
    store: FakeStore,
    aliases: Rc<RefCell<BTreeMap<Digest, Digest>>>,
}

impl Store for Aliased {
    fn put_bytes(&self, bytes: &[u8]) -> Answer<Put> {
        self.store.put_bytes(bytes)
    }
    fn put_artifact(&self, manifest: &Manifest) -> Answer<Put> {
        self.store.put_artifact(manifest)
    }
    fn attach(&self, subject: &Digest, attestation: &Envelope) -> Answer<Put> {
        self.store.attach(subject, attestation)
    }
    fn claims_of(&self, digest: &Digest) -> Answer<Vec<Envelope>> {
        self.store.claims_of(digest)
    }
    fn name(&self, name: &Name, to: &Digest, expected: Option<&Digest>) -> Answer<()> {
        self.store.name(name, to, expected)
    }
    fn resolve(&self, name: &Name) -> Answer<Digest> {
        self.store.resolve(name)
    }
    fn read(&self, digest: &Digest) -> Answer<Holds> {
        let to = self.aliases.borrow().get(digest).cloned();
        self.store.read(to.as_ref().unwrap_or(digest))
    }
    fn versions(&self, ecosystem: &Ecosystem, name: &EcosystemName) -> Answer<Vec<Version>> {
        self.store.versions(ecosystem, name)
    }
}

/// A store, a scripted Dash0 and the target over them.
struct World {
    store: Aliased,
    dash0: Scripted,
    target: Dash0,
}

impl World {
    fn new() -> World {
        World::of(Ok(repository()), &credentials())
    }

    fn of(repository: Answer<RepositoryName>, credentials: &Credentials) -> World {
        let store = Aliased::default();
        let dash0 = Scripted::new(&Dash0::source());
        let target = Dash0::with_backend(
            Box::new(store.clone()),
            repository,
            credentials,
            Box::new(dash0.clone()),
        );
        World {
            store,
            dash0,
            target,
        }
    }

    /// Version `version` of the component published from `repository`,
    /// each of `files` an artifact of the ecosystem it names packed as the
    /// file projection packs one, beside its notes; and the version
    /// verified.
    fn publish_from(
        &self,
        version: &str,
        files: &[(&str, &str, String)],
        repository: Option<RepositoryName>,
    ) -> Verified {
        let store = &self.store.store;
        let mut artifacts = Vec::new();
        let notes = ("file", "notes", "# 1.4.0\n".to_string());
        for (ecosystem, name, text) in files.iter().chain([&notes]) {
            let layer = store.put_bytes(text.as_bytes()).unwrap().digest;
            let meta = ArtifactMeta::new(
                Ecosystem::new(*ecosystem),
                EcosystemName::new(*name),
                Version::new(version),
            )
            .unwrap();
            let manifest = Manifest::Artifact {
                meta: meta.clone(),
                layers: vec![layer],
            };
            let digest = store.put_artifact(&manifest).unwrap().digest;
            artifacts.push(Artifact { digest, meta });
        }
        let index = IndexMeta::new(
            ComponentName::new(COMPONENT),
            Version::new(version),
            artifacts,
        )
        .unwrap()
        .with(Some(timber_seams::Revision::new("4f2a9c1")), repository);
        let digest = store.put_artifact(&Manifest::Index(index)).unwrap().digest;
        Verified::new(digest, Version::new(version))
    }

    /// Version `version` whose one output, `dashboards`, holds a dashboard
    /// of each of `ids`.
    fn version(&self, version: &str, ids: &[&str]) -> Verified {
        let documents: Vec<Value> = ids.iter().map(|id| dashboard(id)).collect();
        self.publish_from(
            version,
            &[("file", COMPONENT, file(&documents))],
            Some(repository()),
        )
    }

    /// The record the marker of `environment` holds.
    fn record(&self, environment: &Environment) -> Record {
        let marker = marker_of(environment);
        let held = self.dash0.held(&marker.id()).expect("no marker is held");
        marker.read(&held).unwrap()
    }
}

/// The words of a refusal by the target.
fn refused_said<T: std::fmt::Debug>(answer: Answer<T>) -> String {
    match answer {
        Err(Not::Refused(r)) => {
            assert_eq!(r.source, Dash0::source());
            assert_eq!(r.why, Why::Said);
            r.said.expect("the target's words")
        }
        other => panic!("answered {other:?}, not refused"),
    }
}

/// What an unreadable answer by the target names, and its words.
fn unreadable<T: std::fmt::Debug>(answer: Answer<T>) -> (String, String) {
    match answer {
        Err(Not::Unreadable(u)) => {
            assert_eq!(u.source, Dash0::source());
            (u.what, u.said.expect("the words"))
        }
        other => panic!("answered {other:?}, not unreadable"),
    }
}

fn applying(version: &Verified) -> Applying {
    Applying {
        index: version.index().clone(),
        version: version.version().clone(),
    }
}

/// The marker `world` holds for `environment`, set to `record` as an
/// earlier apply left it.
fn left(world: &World, environment: &Environment, record: &Record) {
    world.dash0.holds_marker(
        &marker_of(environment).id(),
        Held {
            description: Some(serde_json::to_string(record).unwrap()),
            updated_at: Some(CLOCK.into()),
            deleted_at: None,
        },
    );
}

mod reads {
    use super::*;

    // Requirement: desired is the channel's head

    #[test]
    fn desired_answers_the_head_and_an_empty_channels_absence_and_nothing_runs() {
        let world = World::new();
        assert_eq!(
            world.target.desired(&deneb(), &|| Ok(index(3))),
            Ok(index(3))
        );
        let empty = || -> Answer<Digest> {
            Err(Not::absent(
                &Source::new(Seam::Keeper, "fake"),
                "the head of the chain of dashboards/stable",
            ))
        };
        assert_eq!(world.target.desired(&deneb(), &empty), empty());
        assert!(world.dash0.ran().is_empty(), "{:?}", world.dash0.ran());
    }

    // Requirement: The credential is the host's, handed by its path

    #[test]
    fn a_credential_the_host_does_not_name_is_unreadable_naming_it_and_the_file_and_nothing_runs() {
        let file = PathBuf::from("/home/a/.config/timber/config.toml");
        let none = Credentials {
            file: file.clone(),
            paths: BTreeMap::new(),
        };
        let world = World::of(Ok(repository()), &none);
        let version = world.version("1.4.0", &["overview"]);
        let current = unreadable(world.target.current(&deneb()));
        let report = unreadable(world.target.report(&deneb()));
        let apply = unreadable(world.target.apply(&deneb(), &version));
        for (what, said) in [current, report, apply] {
            assert_eq!(
                what,
                "credential dash0-dashboards of environment deneb of dashboards"
            );
            assert!(
                said.starts_with(&format!(
                    "the bindings file {} names no dash0-dashboards in its [credentials]",
                    file.display()
                )),
                "{said}"
            );
        }
        assert_eq!(
            world.target.desired(&deneb(), &|| Ok(index(3))),
            Ok(index(3))
        );
        assert!(world.dash0.ran().is_empty(), "{:?}", world.dash0.ran());
    }

    // Requirement: current is unread where the marker does not say

    #[test]
    fn a_dataset_timber_never_applied_to_is_unread_naming_the_dataset() {
        let world = World::new();
        let id = marker_of(&deneb()).id();
        for (what, said) in [
            unreadable(world.target.current(&deneb())),
            unreadable(world.target.report(&deneb())),
        ] {
            assert_eq!(what, "what environment deneb of dashboards runs");
            assert_eq!(
                said,
                format!(
                    "the dataset deneb at {API} holds no marker {id} of timber's: it holds \
                     nothing timber applied for the environment, or what it holds was applied \
                     otherwise"
                )
            );
        }
        assert_eq!(world.dash0.ran(), ["marker", "marker"]);
    }

    #[test]
    fn a_deleted_marker_is_unread_naming_when_it_was_deleted() {
        let world = World::new();
        let version = world.version("1.4.0", &["overview"]);
        world.target.apply(&deneb(), &version).unwrap();
        let id = marker_of(&deneb()).id();
        world.dash0.deletes_marker(&id, "2026-11-14T11:00:00Z");
        for (_, said) in [
            unreadable(world.target.current(&deneb())),
            unreadable(world.target.report(&deneb())),
        ] {
            assert_eq!(
                said,
                format!(
                    "the marker {id} in the dataset deneb was deleted at 2026-11-14T11:00:00Z, as \
                     Dash0 says, and the next apply writes it back"
                )
            );
            assert!(!said.contains(version.index().as_str()), "{said}");
        }
    }

    #[test]
    fn an_apply_in_progress_is_unread_naming_both_versions_or_none_before() {
        let world = World::new();
        let (a, b) = (
            world.version("1.3.0", &["overview"]),
            world.version("1.4.0", &["overview"]),
        );
        let marker = marker_of(&deneb());
        let record = marker.record(
            Some(&applying(&a)),
            ["overview".to_string()],
            Some(applying(&b)),
        );
        left(&world, &deneb(), &record);
        let (what, said) = unreadable(world.target.current(&deneb()));
        assert_eq!(what, "what environment deneb of dashboards runs");
        assert_eq!(
            said,
            format!(
                "the marker {} in the dataset deneb names an apply of version 1.4.0 at {} over \
                 version 1.3.0 at {} that did not finish",
                marker.id(),
                b.index(),
                a.index()
            )
        );
        let first = marker.record(None, ["overview".to_string()], Some(applying(&b)));
        left(&world, &deneb(), &first);
        let (_, said) = unreadable(world.target.report(&deneb()));
        assert!(said.ends_with("over none that did not finish"), "{said}");
    }

    #[test]
    fn a_marker_of_another_environment_is_unread_saying_what_it_holds() {
        let world = World::new();
        let a = world.version("1.3.0", &["overview"]);
        let vega = environment("vega", table(&[COMPONENT]));
        let theirs = marker_of(&vega).record(Some(&applying(&a)), ["overview".to_string()], None);
        left(&world, &deneb(), &theirs);
        let (what, said) = unreadable(world.target.current(&deneb()));
        assert_eq!(what, "what environment deneb of dashboards runs");
        assert!(
            said.contains("its record names the environment vega of dashboards"),
            "{said}"
        );
        assert!(!said.contains(a.index().as_str()), "{said}");
    }

    #[test]
    fn a_converged_marker_answers_its_index_and_when_dash0_wrote_it() {
        let world = World::new();
        let version = world.version("1.4.0", &["overview"]);
        world.target.apply(&deneb(), &version).unwrap();
        assert_eq!(world.target.current(&deneb()), Ok(version.index().clone()));
        assert_eq!(
            world.target.report(&deneb()),
            Ok(Running {
                digest: version.index().clone(),
                converged_at: CLOCK.into(),
            })
        );
    }

    #[test]
    fn a_marker_with_no_updated_at_is_unread_at_report() {
        let world = World::new();
        let version = world.version("1.4.0", &["overview"]);
        world.target.apply(&deneb(), &version).unwrap();
        let id = marker_of(&deneb()).id();
        let held = world.dash0.held(&id).unwrap();
        world.dash0.holds_marker(
            &id,
            Held {
                updated_at: None,
                ..held
            },
        );
        assert_eq!(world.target.current(&deneb()), Ok(version.index().clone()));
        let (what, said) = unreadable(world.target.report(&deneb()));
        assert_eq!(what, "when environment deneb of dashboards converged");
        assert!(said.contains("no updatedAt"), "{said}");
    }
}

mod refusals {
    use super::*;

    /// Whether nothing but a dry run ran, which reaches no API.
    fn nothing_reached_dash0(world: &World) {
        let ran = world.dash0.ran();
        assert!(
            ran.iter().all(|r| *r == "validate"),
            "an operation reached Dash0: {ran:?}"
        );
    }

    // Requirement: Apply reads the version's dashboard files by digest

    #[test]
    fn an_output_with_no_artifact_is_refused_naming_it_and_the_version() {
        let world = World::new();
        let version = world.publish_from(
            "1.4.0",
            &[("file", "panels", file(&[dashboard("overview")]))],
            Some(repository()),
        );
        assert_eq!(
            refused_said(world.target.apply(&deneb(), &version)),
            "version 1.4.0 of dashboards names no file artifact of the output dashboards, so it \
             carries no dashboard of it"
        );
        assert!(world.dash0.ran().is_empty(), "{:?}", world.dash0.ran());
    }

    #[test]
    fn another_repositorys_index_or_one_naming_none_is_refused_naming_both() {
        let world = World::new();
        let files = [("file", COMPONENT, file(&[dashboard("overview")]))];
        let other = RepositoryName {
            owner: "someone".into(),
            name: "else".into(),
        };
        let elsewhere = world.publish_from("1.4.0", &files, Some(other));
        let said = refused_said(world.target.apply(&deneb(), &elsewhere));
        assert!(
            said.contains(
                "names the publishing repository someone/else, and this checkout is of \
                 madswan-dev/dashboards"
            ),
            "{said}"
        );
        let unnamed = world.publish_from("1.4.0", &files, None);
        let said = refused_said(world.target.apply(&deneb(), &unnamed));
        assert!(said.contains("names no publishing repository"), "{said}");
        assert!(world.dash0.ran().is_empty(), "{:?}", world.dash0.ran());
    }

    // Requirement: Apply refuses anything but dashboards with identifiers
    // of their own

    #[test]
    fn a_check_rule_among_the_dashboards_is_refused_naming_the_output_and_its_kind() {
        let world = World::new();
        // A document of `kind: CheckRule`, as the CLI's dry run names it.
        let rule = json!({ "kind": "Check rule", "name": "High latency", "id": "latency" });
        let version = world.publish_from(
            "1.4.0",
            &[("file", COMPONENT, file(&[dashboard("overview"), rule]))],
            Some(repository()),
        );
        assert_eq!(
            refused_said(world.target.apply(&deneb(), &version)),
            "the output dashboards holds a document of the kind Check rule, \"High latency\", \
             and the dash0 target applies dashboards alone, each a Dashboard or a PersesDashboard"
        );
        nothing_reached_dash0(&world);
        assert_eq!(world.dash0.ran(), ["validate"]);
    }

    #[test]
    fn a_dashboard_with_no_identifier_is_refused_naming_the_output_and_the_dashboard() {
        let world = World::new();
        let unnamed = json!({ "kind": "PersesDashboard", "name": "Hosts", "id": "" });
        let version = world.publish_from(
            "1.4.0",
            &[("file", COMPONENT, file(&[unnamed]))],
            Some(repository()),
        );
        let said = refused_said(world.target.apply(&deneb(), &version));
        assert!(
            said.starts_with(
                "the output dashboards holds the PersesDashboard \"Hosts\" with no identifier"
            ),
            "{said}"
        );
        nothing_reached_dash0(&world);
    }

    #[test]
    fn one_identifier_in_two_outputs_is_refused_naming_it_and_both() {
        let world = World::new();
        let version = world.publish_from(
            "1.4.0",
            &[
                ("file", "hosts", file(&[dashboard("overview")])),
                ("file", "services", file(&[dashboard("overview")])),
            ],
            Some(repository()),
        );
        let two = environment("deneb", table(&["hosts", "services"]));
        assert_eq!(
            refused_said(world.target.apply(&two, &version)),
            "the outputs hosts and services each give the identifier overview"
        );
        nothing_reached_dash0(&world);
        let once = world.publish_from(
            "1.4.0",
            &[(
                "file",
                COMPONENT,
                file(&[dashboard("overview"), dashboard("overview")]),
            )],
            Some(repository()),
        );
        assert_eq!(
            refused_said(world.target.apply(&deneb(), &once)),
            "the output dashboards gives the identifier overview to two documents"
        );
        nothing_reached_dash0(&world);
    }

    #[test]
    fn the_markers_identifier_is_refused() {
        let world = World::new();
        let id = marker_of(&deneb()).id();
        let version = world.version("1.4.0", &["overview", &id]);
        let said = refused_said(world.target.apply(&deneb(), &version));
        assert!(
            said.contains(&format!(
                "whose identifier {id} is the marker timber keeps for environment deneb of \
                 dashboards"
            )),
            "{said}"
        );
        nothing_reached_dash0(&world);
    }

    #[test]
    fn an_output_of_no_dashboard_is_refused_naming_it() {
        let world = World::new();
        let version = world.publish_from(
            "1.4.0",
            &[
                ("file", "hosts", file(&[dashboard("overview")])),
                ("file", "services", file(&[])),
            ],
            Some(repository()),
        );
        let two = environment("deneb", table(&["hosts", "services"]));
        assert_eq!(
            refused_said(world.target.apply(&two, &version)),
            "the dashboard file of the output services in version 1.4.0 holds no dashboard"
        );
        nothing_reached_dash0(&world);
    }

    // Requirement: Apply reads the version's identifiers from the CLI's
    // dry run

    #[test]
    fn a_dry_run_the_cli_refuses_is_answered_in_its_words() {
        let world = World::new();
        let words = "{\"error\": \"validation failed:\\n  0.yaml: failed to parse YAML\"}";
        world.dash0.fails("validate", 0, words);
        let version = world.version("1.4.0", &["overview"]);
        let (_, said) = unreadable(world.target.apply(&deneb(), &version));
        assert_eq!(said, words);
        assert_eq!(world.dash0.ran(), ["validate"]);
    }
}

mod applies {
    use super::*;

    /// The records each marker write held, in order.
    fn writes(world: &World) -> Vec<Record> {
        world
            .dash0
            .asked()
            .into_iter()
            .filter_map(|asked| match asked {
                Asked::PutMarker(record) => Some(record),
                _ => None,
            })
            .collect()
    }

    // Requirement: Apply records every dashboard before it applies one

    #[test]
    fn a_first_apply_with_no_marker_removes_nothing_and_records_the_version() {
        let world = World::new();
        world.dash0.holds("mine");
        let version = world.version("1.4.0", &["overview", "sluice"]);
        assert_eq!(world.target.apply(&deneb(), &version), Ok(()));
        assert_eq!(
            world.dash0.ran(),
            ["validate", "marker", "put_marker", "apply", "put_marker"]
        );
        let marker = marker_of(&deneb());
        let [during, after] = <[Record; 2]>::try_from(writes(&world)).unwrap();
        assert_eq!(
            during,
            marker.record(
                None,
                ["overview".to_string(), "sluice".into()],
                Some(applying(&version))
            )
        );
        assert_eq!(
            after,
            marker.record(
                Some(&applying(&version)),
                ["overview".to_string(), "sluice".into()],
                None
            )
        );
        assert_eq!(world.record(&deneb()), after);
        assert_eq!(world.dash0.dashboards(), ["mine", "overview", "sluice"]);
    }

    // Requirement: Apply removes only what the marker names and the
    // version does not hold

    #[test]
    fn a_version_dropping_sluice_deletes_it_alone_and_never_one_made_by_hand() {
        let world = World::new();
        world.dash0.holds("mine");
        let a = world.version("1.3.0", &["overview", "sluice"]);
        let b = world.version("1.4.0", &["overview"]);
        world.target.apply(&deneb(), &a).unwrap();
        let before = world.dash0.asked().len();
        world.target.apply(&deneb(), &b).unwrap();
        let asked = world.dash0.asked()[before..].to_vec();
        let names: Vec<&str> = asked.iter().map(Asked::name).collect();
        assert_eq!(
            names,
            [
                "validate",
                "marker",
                "put_marker",
                "apply",
                "delete",
                "put_marker"
            ]
        );
        assert_eq!(asked[4], Asked::Delete("sluice".into()));
        let marker = marker_of(&deneb());
        assert_eq!(
            asked[2],
            Asked::PutMarker(marker.record(
                Some(&applying(&a)),
                ["overview".to_string(), "sluice".into()],
                Some(applying(&b))
            ))
        );
        assert_eq!(
            world.record(&deneb()),
            marker.record(Some(&applying(&b)), ["overview".to_string()], None)
        );
        assert_eq!(world.dash0.dashboards(), ["mine", "overview"]);
        assert_eq!(world.target.current(&deneb()), Ok(b.index().clone()));
    }

    #[test]
    fn a_deleted_markers_dashboards_are_removed_and_the_marker_written_back() {
        let world = World::new();
        let a = world.version("1.3.0", &["overview", "sluice"]);
        let b = world.version("1.4.0", &["overview"]);
        world.target.apply(&deneb(), &a).unwrap();
        let id = marker_of(&deneb()).id();
        world.dash0.deletes_marker(&id, "2026-11-14T11:00:00Z");
        world.target.apply(&deneb(), &b).unwrap();
        assert!(world
            .dash0
            .asked()
            .contains(&Asked::Delete("sluice".into())));
        assert_eq!(world.dash0.held(&id).unwrap().deleted_at, None);
        assert_eq!(world.dash0.dashboards(), ["overview"]);
        assert_eq!(world.target.current(&deneb()), Ok(b.index().clone()));
    }

    // Requirement: Dash0's words are the answer

    #[test]
    fn a_failure_at_each_step_answers_dash0s_words_and_runs_nothing_after() {
        let words = "access denied; check your permissions";
        for (operation, nth, ran) in [
            ("marker", 1, vec!["validate", "marker"]),
            ("put_marker", 1, vec!["validate", "marker", "put_marker"]),
            (
                "apply",
                1,
                vec!["validate", "marker", "put_marker", "apply"],
            ),
            (
                "delete",
                1,
                vec!["validate", "marker", "put_marker", "apply", "delete"],
            ),
            (
                "put_marker",
                2,
                vec![
                    "validate",
                    "marker",
                    "put_marker",
                    "apply",
                    "delete",
                    "put_marker",
                ],
            ),
        ] {
            let world = World::new();
            let marker = marker_of(&deneb());
            let a = world.version("1.3.0", &["overview", "sluice"]);
            let b = world.version("1.4.0", &["overview"]);
            left(
                &world,
                &deneb(),
                &marker.record(
                    Some(&applying(&a)),
                    ["overview".to_string(), "sluice".into()],
                    None,
                ),
            );
            world.dash0.holds("sluice");
            world.dash0.fails(operation, nth, words);
            let (what, said) = unreadable(world.target.apply(&deneb(), &b));
            assert_eq!(said, words, "{operation} {nth}");
            assert_eq!(what, format!("dash0 {operation}"));
            assert_eq!(world.dash0.ran(), ran, "{operation} {nth}");
        }
    }

    #[test]
    fn a_token_that_may_not_write_applies_no_dashboard() {
        let world = World::new();
        let words = "access denied; check your permissions (status: 403)";
        world.dash0.fails("put_marker", 0, words);
        let version = world.version("1.4.0", &["overview"]);
        let (_, said) = unreadable(world.target.apply(&deneb(), &version));
        assert_eq!(said, words);
        assert!(world.dash0.dashboards().is_empty());
        assert!(!world.dash0.ran().contains(&"apply"));
    }

    // Requirement: Apply records every dashboard before it applies one

    #[test]
    fn an_apply_that_stopped_after_its_first_write_leaves_its_dashboards_for_the_next_to_remove() {
        let world = World::new();
        let marker = marker_of(&deneb());
        let a = world.version("1.3.0", &["overview", "sluice"]);
        let b = world.version("1.4.0", &["overview", "new"]);
        let c = world.version("1.5.0", &["overview"]);
        world.target.apply(&deneb(), &a).unwrap();
        // The apply of b writes the marker in progress and applies its
        // dashboards, and the run that deletes sluice fails.
        let words = "dial tcp: lookup api.us-east-1.aws.dash0.com: no such host";
        world.dash0.fails("delete", 1, words);
        assert_eq!(unreadable(world.target.apply(&deneb(), &b)).1, words);
        assert_eq!(world.dash0.dashboards(), ["new", "overview", "sluice"]);
        assert_eq!(
            world.record(&deneb()),
            marker.record(
                Some(&applying(&a)),
                ["new".to_string(), "overview".into(), "sluice".into()],
                Some(applying(&b))
            )
        );
        assert!(world.target.current(&deneb()).is_err());
        let before = world.dash0.asked().len();
        world.target.apply(&deneb(), &c).unwrap();
        let deleted: Vec<Asked> = world.dash0.asked()[before..]
            .iter()
            .filter(|a| a.name() == "delete")
            .cloned()
            .collect();
        assert_eq!(
            deleted,
            [Asked::Delete("new".into()), Asked::Delete("sluice".into())]
        );
        assert_eq!(world.dash0.dashboards(), ["overview"]);
        assert_eq!(
            world.record(&deneb()),
            marker.record(Some(&applying(&c)), ["overview".to_string()], None)
        );
        assert_eq!(world.target.current(&deneb()), Ok(c.index().clone()));
    }
}

mod group {
    use std::cell::Cell;

    use timber_conformance::seams::target::{self, TargetRig};
    use timber_conformance::Rig;

    use super::*;

    /// The target group's rig over the profile and the scripted Dash0: the
    /// target holds a marker, each environment is one of its own in the
    /// one dataset, the unreadable one in a dataset whose marker read
    /// fails, and the foreign settings add a `token` (119, 45; design
    /// D9).
    struct Dash0Rig {
        world: World,
        environments: Cell<u32>,
    }

    impl Dash0Rig {
        const UNREACHABLE: &'static str = "gone";
        const WORDS: &'static str = "{\"error\": \"dial tcp: lookup api.us-east-1.aws.dash0.com: \
                                     no such host\"}";

        fn new() -> Dash0Rig {
            let world = World::new();
            world.dash0.unreachable(Self::UNREACHABLE, Self::WORDS);
            Dash0Rig {
                world,
                environments: Cell::new(0),
            }
        }

        fn next(&self, dataset: &str) -> Environment {
            self.environments.set(self.environments.get() + 1);
            let mut settings = table(&[COMPONENT]);
            settings["dataset"] = json!(dataset);
            environment(
                &format!("environment-{}", self.environments.get()),
                settings,
            )
        }
    }

    impl Rig for Dash0Rig {
        fn profile(&self) -> String {
            PROFILE.into()
        }
    }

    impl TargetRig for Dash0Rig {
        fn target(&self) -> &dyn Target {
            &self.world.target
        }

        /// A version of two dashboards published, and read at `digest` as
        /// a store holds an index at its own: the suite stands in the
        /// core's place, which makes a `Verified` after its verify (42).
        fn verified(&self, digest: &Digest) -> Verified {
            let published = self.world.version("1.4.0", &["overview", "sluice"]);
            let mut aliases = self.world.store.aliases.borrow_mut();
            aliases.insert(digest.clone(), published.index().clone());
            Verified::new(digest.clone(), published.version().clone())
        }

        /// The target is pushed to, handed the head at desired, so `head`
        /// is left unread.
        fn environment(&self, _head: Option<&Digest>) -> Environment {
            self.next(DATASET)
        }

        fn unreadable_environment(&self) -> (Environment, String) {
            (self.next(Self::UNREACHABLE), Self::WORDS.into())
        }

        fn foreign_settings(&self) -> (Environment, String) {
            let mut environment = self.next(DATASET);
            environment.settings["token"] = json!("auth_abc");
            (environment, "token".into())
        }
    }

    // Requirement: The profile passes the target group

    #[test]
    fn the_target_group_passes_over_the_scripted_dash0_holding_a_marker() {
        let report = timber_conformance::run(target::GROUP, &target::scenarios(), &|| {
            Box::new(Dash0Rig::new()) as Box<dyn TargetRig>
        });
        report.assert();
        let inapplicable: Vec<&str> = report.not_applicable.iter().map(|n| n.scenario).collect();
        assert_eq!(
            inapplicable,
            ["applied where no marker is held"],
            "{report}"
        );
    }
}
