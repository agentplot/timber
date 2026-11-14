//! The profile's runs of Dash0's CLI, behind a trait of the profile's own,
//! which is no seam: the target seam stays the one interface (rule core.2;
//! design D1, D4). [`Cli`] runs `dash0` as found on `PATH`, which where the
//! host runs `timber apply` in the repository's shell is the CLI the
//! repository pins and checks its dashboards with. A test's scripted Dash0
//! answers in its place.
//!
//! Every run is given each setting and nothing of the host's (record
//! 0033, rule core.21): it runs in a folder the operation makes under the
//! system's temporary directory and removes when it ends, holding an
//! empty `config/`, and is given
//!
//! ```text
//! dash0 --agent-mode --no-skill-hint --config-dir <run>/config \
//!       --auth-token-file <path> <subcommand…> --api-url <api> --dataset <dataset>
//! ```
//!
//! a dry run, which reaches no API, taking no `--auth-token-file`,
//! `--api-url` or `--dataset`. Every variable whose name begins `DASH0_`
//! is removed from its environment, and with an empty configuration
//! folder there is no profile, so no setting of the host's own `dash0` is
//! read; timber never opens the token's file, which the CLI alone reads.
//! `--agent-mode` gives JSON output and JSON errors with no prompt. A run
//! that exits other than as its operation allows is unreadable, naming the
//! command run and carrying the CLI's stderr verbatim (59, rule core.19).

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;
use timber_seams::{Answer, Not, Source};

use super::marker::{Held, Marker, Record};
use crate::profiles::run::{Ran, Tool};

/// The variables removed from every run's environment: every one of the
/// host's own `dash0` settings.
const DASH0_ENV: &[&str] = &["DASH0_*"];

/// Where an operation reaches Dash0: the address of its API, the dataset,
/// and the path of the file holding the token, which the CLI alone reads.
#[derive(Clone, Copy, Debug)]
pub(super) struct At<'a> {
    pub api: &'a str,
    pub dataset: &'a str,
    pub token: &'a Path,
}

/// One document a dry run read: the file it is in, by its place among the
/// files handed, its kind as the CLI names it, its name and its
/// identifier, empty where it has none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Document {
    pub file: usize,
    pub kind: String,
    pub name: String,
    pub id: String,
}

/// What the Dash0 target asks of Dash0.
pub(super) trait Backend {
    /// Every document of `files`, read by one dry run of the CLI, which
    /// reaches no API and is handed no token, address or dataset; the
    /// CLI's words where it refuses one (design D5).
    fn validate(&self, files: &[Vec<u8>]) -> Answer<Vec<Document>>;

    /// The dashboard `id`, the marker, as Dash0 at `at` holds it, deleted
    /// or not; none where Dash0 says it holds no such dashboard (design
    /// D6).
    fn marker(&self, at: &At<'_>, id: &str) -> Answer<Option<Held>>;

    /// The marker written, holding `record` (design D6).
    fn put_marker(&self, at: &At<'_>, marker: &Marker, record: &Record) -> Answer<()>;

    /// Every document of `files` applied by its identifier, in one run
    /// (90).
    fn apply(&self, at: &At<'_>, files: &[Vec<u8>]) -> Answer<()>;

    /// The dashboard `id` deleted; one already gone counts as deleted
    /// (C.12).
    fn delete(&self, at: &At<'_>, id: &str) -> Answer<()>;
}

/// Dash0's own CLI.
pub(super) struct Cli {
    dash0: Tool,
    source: Source,
}

/// One file of a dry run's JSON: its path under the folder handed, and the
/// changes its documents would make.
#[derive(Deserialize)]
struct DryFile {
    path: String,
    changes: Vec<DryChange>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DryChange {
    op: String,
    kind: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    origin_or_id: String,
}

/// The folder of one run, removed when it is dropped.
struct Folder(tempfile::TempDir);

impl Folder {
    fn path(&self) -> &Path {
        self.0.path()
    }

    fn config(&self) -> PathBuf {
        self.path().join("config")
    }
}

impl Cli {
    /// `program`, `dash0` as found on `PATH` or a path to it, answering as
    /// `source`.
    pub fn new(program: impl Into<PathBuf>, source: &Source) -> Cli {
        Cli {
            dash0: Tool::new(program, source.clone()),
            source: source.clone(),
        }
    }

    /// Unreadable naming `what`, for `said`.
    fn unreadable(&self, what: impl Into<String>, said: String) -> Not {
        Not::unreadable(&self.source, what, Some(said))
    }

    /// A folder of the run's own under the system's temporary directory,
    /// holding an empty `config/`.
    fn folder(&self) -> Answer<Folder> {
        let made = tempfile::Builder::new()
            .prefix("timber-dash0-")
            .tempdir()
            .map_err(|e| {
                self.unreadable(
                    format!(
                        "a folder for a run of dash0 under {}",
                        std::env::temp_dir().display()
                    ),
                    e.to_string(),
                )
            })?;
        let folder = Folder(made);
        fs::create_dir(folder.config())
            .map_err(|e| self.unreadable(folder.config().display().to_string(), e.to_string()))?;
        Ok(folder)
    }

    /// `bytes` written at `path`.
    fn write(&self, path: &Path, bytes: &[u8]) -> Answer<()> {
        fs::write(path, bytes)
            .map_err(|e| self.unreadable(path.display().to_string(), e.to_string()))
    }

    /// Each of `files` written into the run's folder as
    /// `dashboards/<n>.yaml`, `<n>` its place, so no output's name becomes
    /// a path; and that directory.
    fn dashboards(&self, folder: &Folder, files: &[Vec<u8>]) -> Answer<PathBuf> {
        let dir = folder.path().join("dashboards");
        fs::create_dir(&dir)
            .map_err(|e| self.unreadable(dir.display().to_string(), e.to_string()))?;
        for (n, bytes) in files.iter().enumerate() {
            self.write(&dir.join(format!("{n}.yaml")), bytes)?;
        }
        Ok(dir)
    }

    /// The arguments of a run in `folder`: the root's flags, the token's
    /// path where the run reaches `at`, the subcommand, and the API's
    /// address and the dataset after it.
    fn args(folder: &Folder, at: Option<&At<'_>>, subcommand: &[&OsStr]) -> Vec<OsString> {
        let mut args: Vec<OsString> = vec![
            "--agent-mode".into(),
            "--no-skill-hint".into(),
            "--config-dir".into(),
            folder.config().into(),
        ];
        if let Some(at) = at {
            args.push("--auth-token-file".into());
            args.push(at.token.into());
        }
        args.extend(subcommand.iter().map(|arg| arg.to_os_string()));
        if let Some(at) = at {
            args.extend(["--api-url", at.api, "--dataset", at.dataset].map(OsString::from));
        }
        args
    }

    /// A run in `folder` with no `DASH0_*` variable, answering what it
    /// printed whatever it exits with.
    fn ran(
        &self,
        folder: &Folder,
        at: Option<&At<'_>>,
        subcommand: &[&OsStr],
    ) -> Answer<(Vec<OsString>, Ran)> {
        let args = Cli::args(folder, at, subcommand);
        let ran = self.dash0.ran(None, &args, DASH0_ENV)?;
        Ok((args, ran))
    }

    /// A run in `folder` that must exit 0, answering what it printed.
    fn run(&self, folder: &Folder, at: Option<&At<'_>>, subcommand: &[&OsStr]) -> Answer<Ran> {
        let (args, ran) = self.ran(folder, at, subcommand)?;
        if ran.code == Some(0) {
            return Ok(ran);
        }
        Err(self.dash0.failed(&args, &ran))
    }
}

/// Whether `stderr`, what a `dashboards get` of `id` printed, says Dash0
/// holds no such dashboard: `dashboard "<id>" not found`, in agent mode's
/// `error` or as it printed it.
fn not_found(stderr: &[u8], id: &str) -> bool {
    let printed = String::from_utf8_lossy(stderr);
    let words = serde_json::from_str::<Value>(printed.trim())
        .ok()
        .and_then(|error| error.get("error")?.as_str().map(str::to_string))
        .unwrap_or_else(|| printed.trim().to_string());
    words.contains(&format!("dashboard \"{id}\" not found"))
}

impl Backend for Cli {
    fn validate(&self, files: &[Vec<u8>]) -> Answer<Vec<Document>> {
        let folder = self.folder()?;
        let dir = self.dashboards(&folder, files)?;
        let subcommand = [
            OsStr::new("apply"),
            OsStr::new("-f"),
            dir.as_os_str(),
            OsStr::new("--dry-run"),
        ];
        let ran = self.run(&folder, None, &subcommand)?;
        let what = || format!("what dash0 apply -f {} --dry-run read", dir.display());
        let read: Vec<DryFile> = serde_json::from_slice(&ran.stdout).map_err(|e| {
            self.unreadable(what(), format!("it printed no files' changes in JSON: {e}"))
        })?;
        let mut documents = Vec::new();
        for file in read {
            let n = file
                .path
                .strip_suffix(".yaml")
                .and_then(|n| n.parse::<usize>().ok())
                .filter(|n| *n < files.len() && file.path == format!("{n}.yaml"))
                .ok_or_else(|| {
                    self.unreadable(
                        what(),
                        format!(
                            "it named the file {:?}, which timber did not write",
                            file.path
                        ),
                    )
                })?;
            for change in file.changes {
                if change.op != "apply" {
                    return Err(self.unreadable(
                        what(),
                        format!(
                            "it named a change {:?}, and a dry run of files applies",
                            change.op
                        ),
                    ));
                }
                documents.push(Document {
                    file: n,
                    kind: change.kind,
                    name: change.name,
                    id: change.origin_or_id,
                });
            }
        }
        Ok(documents)
    }

    fn marker(&self, at: &At<'_>, id: &str) -> Answer<Option<Held>> {
        let folder = self.folder()?;
        let subcommand = ["dashboards", "get", id, "-o", "json"].map(OsStr::new);
        let (args, ran) = self.ran(&folder, Some(at), &subcommand)?;
        if ran.code != Some(0) {
            if not_found(&ran.stderr, id) {
                return Ok(None);
            }
            return Err(self.dash0.failed(&args, &ran));
        }
        Held::parse(&ran.stdout).map(Some).map_err(|said| {
            self.unreadable(
                format!("the marker {id} in the dataset {}", at.dataset),
                format!("dash0 dashboards get {id} -o json: {said}"),
            )
        })
    }

    fn put_marker(&self, at: &At<'_>, marker: &Marker, record: &Record) -> Answer<()> {
        let folder = self.folder()?;
        let file = folder.path().join("marker.yaml");
        self.write(&file, &marker.document(record))?;
        let subcommand = [OsStr::new("apply"), OsStr::new("-f"), file.as_os_str()];
        self.run(&folder, Some(at), &subcommand).map(drop)
    }

    fn apply(&self, at: &At<'_>, files: &[Vec<u8>]) -> Answer<()> {
        let folder = self.folder()?;
        let dir = self.dashboards(&folder, files)?;
        let subcommand = [OsStr::new("apply"), OsStr::new("-f"), dir.as_os_str()];
        self.run(&folder, Some(at), &subcommand).map(drop)
    }

    fn delete(&self, at: &At<'_>, id: &str) -> Answer<()> {
        let folder = self.folder()?;
        let subcommand = ["dashboards", "delete", id, "--force"].map(OsStr::new);
        self.run(&folder, Some(at), &subcommand).map(drop)
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;

    use serde_json::json;
    use tempfile::TempDir;
    use timber_seams::{
        ChannelId, ChannelName, ComponentName, Environment, EnvironmentName, RepositoryName, Seam,
        Version,
    };

    use super::super::marker::Applying;
    use super::*;

    const API: &str = "https://api.us-east-1.aws.dash0.com";
    const DATASET: &str = "deneb";

    /// The variable naming the stand-in's folder to a child of this test
    /// binary, which runs every operation with the host's own `DASH0_*`
    /// variables set in its environment, as timber's would be.
    const CHILD: &str = "TIMBER_TEST_DASH0_STAND_IN";

    /// What the token's file holds, which no run is given.
    const SECRET: &str = "auth_0123456789secret";

    fn source() -> Source {
        Source::new(Seam::Target, "dash0")
    }

    /// A stand-in `dash0`, written by the test, which records each run's
    /// arguments, its environment, the configuration folder it was handed
    /// and what that folder held, and the files its `-f` names, each run
    /// in a folder of its own numbered from 0; and answers the n-th run
    /// with what the test sets for it, printing nothing and exiting 0
    /// where it sets nothing.
    struct StandIn {
        dir: PathBuf,
    }

    /// What one run of the stand-in was handed.
    #[derive(Debug)]
    struct Run {
        args: Vec<String>,
        env: Vec<String>,
        config_dir: String,
        config_held: String,
        files: Option<String>,
        contents: Option<String>,
    }

    impl StandIn {
        /// The stand-in in `dir`, with a token's file beside it.
        fn at(dir: &Path) -> StandIn {
            let at = dir.display();
            let script = format!(
                "#!/bin/sh\n\
                 d='{at}'\n\
                 n=$(ls \"$d/runs\" | wc -l | tr -d ' ')\n\
                 r=\"$d/runs/$n\"\n\
                 mkdir \"$r\"\n\
                 for arg in \"$@\"; do printf '%s\\n' \"$arg\"; done > \"$r/args\"\n\
                 env > \"$r/env\"\n\
                 prev=\n\
                 for arg in \"$@\"; do\n\
                 case \"$prev\" in\n\
                 --config-dir) printf '%s' \"$arg\" > \"$r/config-dir\"; ls -A \"$arg\" > \"$r/config-held\" ;;\n\
                 -f) if [ -d \"$arg\" ]; then ls \"$arg\" > \"$r/files\"; cat \"$arg\"/* > \"$r/contents\"; \
                     else cat \"$arg\" > \"$r/contents\"; fi ;;\n\
                 esac\n\
                 prev=$arg\n\
                 done\n\
                 if [ -f \"$d/answers/$n.out\" ]; then cat \"$d/answers/$n.out\"; fi\n\
                 if [ -f \"$d/answers/$n.err\" ]; then cat \"$d/answers/$n.err\" >&2; fi\n\
                 if [ -f \"$d/answers/$n.code\" ]; then exit \"$(cat \"$d/answers/$n.code\")\"; fi\n\
                 exit 0\n"
            );
            for folder in ["runs", "answers"] {
                fs::create_dir_all(dir.join(folder)).unwrap();
            }
            let program = dir.join("dash0");
            fs::write(&program, script).unwrap();
            fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).unwrap();
            fs::write(dir.join("token"), SECRET).unwrap();
            StandIn {
                dir: dir.to_path_buf(),
            }
        }

        fn cli(&self) -> Cli {
            Cli::new(self.dir.join("dash0"), &source())
        }

        fn token(&self) -> PathBuf {
            self.dir.join("token")
        }

        /// Run `n` prints `out`, says `err` on its stderr and exits with
        /// `code`.
        fn answers(&self, n: usize, out: &str, err: &str, code: i32) {
            let answers = self.dir.join("answers");
            fs::write(answers.join(format!("{n}.out")), out).unwrap();
            fs::write(answers.join(format!("{n}.err")), err).unwrap();
            fs::write(answers.join(format!("{n}.code")), code.to_string()).unwrap();
        }

        /// Every run, in order.
        fn runs(&self) -> Vec<Run> {
            let read = |run: &Path, name: &str| fs::read_to_string(run.join(name)).ok();
            let lines = |text: Option<String>| -> Vec<String> {
                text.unwrap_or_default()
                    .lines()
                    .map(str::to_string)
                    .collect()
            };
            let count = fs::read_dir(self.dir.join("runs")).unwrap().count();
            (0..count)
                .map(|n| {
                    let run = self.dir.join("runs").join(n.to_string());
                    Run {
                        args: lines(read(&run, "args")),
                        env: lines(read(&run, "env")),
                        config_dir: read(&run, "config-dir").unwrap(),
                        config_held: read(&run, "config-held").unwrap(),
                        files: read(&run, "files"),
                        contents: read(&run, "contents"),
                    }
                })
                .collect()
        }
    }

    /// A stand-in in a folder of its own.
    fn stand_in() -> (TempDir, StandIn) {
        let dir = TempDir::new().unwrap();
        let stand_in = StandIn::at(dir.path());
        (dir, stand_in)
    }

    fn at(token: &Path) -> At<'_> {
        At {
            api: API,
            dataset: DATASET,
            token,
        }
    }

    fn marker() -> Marker {
        let repository = RepositoryName {
            owner: "madswan-dev".into(),
            name: "dashboards".into(),
        };
        let environment = Environment {
            name: EnvironmentName::new("deneb"),
            channel: ChannelId::new(
                Some(ComponentName::new("dashboards")),
                ChannelName::new("stable"),
            ),
            waits: false,
            settings: json!({}),
        };
        Marker::of(&repository, &environment)
    }

    fn record() -> Record {
        let applied = Applying {
            index: timber_seams::Digest::of(b"1.3.0"),
            version: Version::new("1.3.0"),
        };
        marker().record(Some(&applied), ["overview".to_string()], None)
    }

    /// The arguments every run is given before its own, in `run`'s
    /// folder, and the token's path where it reaches Dash0.
    fn root(run: &Run, token: Option<&Path>) -> Vec<String> {
        let mut args = vec![
            "--agent-mode".to_string(),
            "--no-skill-hint".into(),
            "--config-dir".into(),
            run.config_dir.clone(),
        ];
        if let Some(token) = token {
            args.push("--auth-token-file".into());
            args.push(token.display().to_string());
        }
        args
    }

    /// The folder `run` was handed its configuration folder in, which the
    /// operation made.
    fn folder_of(run: &Run) -> PathBuf {
        let config = Path::new(&run.config_dir);
        assert_eq!(config.file_name().unwrap(), "config", "{run:?}");
        config.parent().unwrap().to_path_buf()
    }

    /// Each of `runs` was handed an empty configuration folder of its own,
    /// not `~/.dash0`, removed once its operation ended, and no variable of
    /// the host's own `dash0`.
    fn of_their_own(runs: &[Run], home: Option<&Path>) {
        for run in runs {
            assert_eq!(
                run.config_held, "",
                "a run's folder held something: {run:?}"
            );
            let folder = folder_of(run);
            assert!(
                folder.starts_with(std::env::temp_dir()),
                "{} is not under the temporary directory",
                folder.display()
            );
            assert!(!folder.exists(), "{} was left behind", folder.display());
            if let Some(home) = home {
                assert_ne!(Path::new(&run.config_dir), home.join(".dash0"));
            }
            let dash0: Vec<&String> = run.env.iter().filter(|l| l.starts_with("DASH0_")).collect();
            assert!(dash0.is_empty(), "the run saw {dash0:?}");
        }
    }

    /// What Dash0 answers `dashboards get` of the marker holding
    /// `record`, as JSON.
    fn answered(record: &Record) -> String {
        let mut document: Value = serde_json::from_slice(&marker().document(record)).unwrap();
        document["metadata"]["updatedAt"] = json!("2026-11-14T10:00:00Z");
        document.to_string()
    }

    // Requirement: Apply reads the version's identifiers from the CLI's
    // dry run

    #[test]
    fn a_dry_run_is_handed_the_files_and_no_token_address_or_dataset() {
        let (_dir, stand_in) = stand_in();
        let printed = json!([
            { "path": "0.yaml", "changes": [
                { "op": "apply", "kind": "Dashboard", "name": "Overview", "originOrId": "overview" },
                { "op": "apply", "kind": "PersesDashboard", "name": "Hosts", "originOrId": "" },
            ]},
            { "path": "1.yaml", "changes": [
                { "op": "apply", "kind": "Check rule", "name": "Latency", "originOrId": "latency" },
            ]},
        ]);
        stand_in.answers(0, &printed.to_string(), "", 0);
        let files = [b"kind: Dashboard\n".to_vec(), b"kind: CheckRule\n".to_vec()];
        let documents = stand_in.cli().validate(&files).unwrap();
        assert_eq!(
            documents,
            [
                Document {
                    file: 0,
                    kind: "Dashboard".into(),
                    name: "Overview".into(),
                    id: "overview".into(),
                },
                Document {
                    file: 0,
                    kind: "PersesDashboard".into(),
                    name: "Hosts".into(),
                    id: String::new(),
                },
                Document {
                    file: 1,
                    kind: "Check rule".into(),
                    name: "Latency".into(),
                    id: "latency".into(),
                },
            ]
        );
        let runs = stand_in.runs();
        let [run] = runs.as_slice() else {
            panic!("{runs:?}")
        };
        let dashboards = folder_of(run).join("dashboards").display().to_string();
        let mut args = root(run, None);
        args.extend(["apply".into(), "-f".into(), dashboards, "--dry-run".into()]);
        assert_eq!(run.args, args);
        assert_eq!(run.files.as_deref(), Some("0.yaml\n1.yaml\n"));
        assert_eq!(
            run.contents.as_deref(),
            Some("kind: Dashboard\nkind: CheckRule\n")
        );
        of_their_own(&runs, None);
    }

    #[test]
    fn a_dry_run_naming_a_file_timber_did_not_write_is_unreadable() {
        let (_dir, stand_in) = stand_in();
        let printed = json!([{ "path": "../other.yaml", "changes": [] }]);
        stand_in.answers(0, &printed.to_string(), "", 0);
        match stand_in.cli().validate(&[b"kind: Dashboard\n".to_vec()]) {
            Err(Not::Unreadable(u)) => {
                let said = u.said.unwrap();
                assert!(said.contains("\"../other.yaml\""), "{said}");
            }
            other => panic!("answered {other:?}"),
        }
    }

    // Requirement: Every run of the CLI takes its settings as flags

    #[test]
    fn the_marker_is_read_with_each_setting_as_a_flag_around_the_subcommand() {
        let (_dir, stand_in) = stand_in();
        stand_in.answers(0, &answered(&record()), "", 0);
        let token = stand_in.token();
        let id = marker().id();
        let held = stand_in.cli().marker(&at(&token), &id).unwrap().unwrap();
        assert_eq!(held.updated_at.as_deref(), Some("2026-11-14T10:00:00Z"));
        assert_eq!(marker().read(&held), Ok(record()));
        let runs = stand_in.runs();
        let mut args = root(&runs[0], Some(&token));
        args.extend(
            [
                "dashboards",
                "get",
                &id,
                "-o",
                "json",
                "--api-url",
                API,
                "--dataset",
                DATASET,
            ]
            .map(str::to_string),
        );
        assert_eq!(runs[0].args, args);
        of_their_own(&runs, None);
    }

    #[test]
    fn the_marker_is_written_and_the_dashboards_applied_and_deleted_each_with_its_settings() {
        let (_dir, stand_in) = stand_in();
        let (cli, token) = (stand_in.cli(), stand_in.token());
        cli.put_marker(&at(&token), &marker(), &record()).unwrap();
        cli.apply(
            &at(&token),
            &[b"a: 1\n".to_vec(), b"b: 2\n".to_vec(), b"c: 3\n".to_vec()],
        )
        .unwrap();
        cli.delete(&at(&token), "sluice").unwrap();
        let runs = stand_in.runs();
        let after = ["--api-url", API, "--dataset", DATASET].map(str::to_string);
        let expect = |run: &Run, subcommand: Vec<String>| {
            let mut args = root(run, Some(&token));
            args.extend(subcommand);
            args.extend(after.clone());
            assert_eq!(run.args, args);
        };
        let marker_file = folder_of(&runs[0])
            .join("marker.yaml")
            .display()
            .to_string();
        expect(&runs[0], vec!["apply".into(), "-f".into(), marker_file]);
        assert_eq!(
            runs[0].contents.as_deref().map(str::as_bytes),
            Some(marker().document(&record()).as_slice())
        );
        let dashboards = folder_of(&runs[1]).join("dashboards").display().to_string();
        expect(&runs[1], vec!["apply".into(), "-f".into(), dashboards]);
        assert_eq!(runs[1].files.as_deref(), Some("0.yaml\n1.yaml\n2.yaml\n"));
        expect(
            &runs[2],
            ["dashboards", "delete", "sluice", "--force"]
                .map(str::to_string)
                .to_vec(),
        );
        of_their_own(&runs, None);
    }

    // Requirement: The credential is the host's, handed by its path

    #[test]
    fn the_token_goes_by_its_path_and_its_contents_are_in_no_argument_or_variable() {
        let (_dir, stand_in) = stand_in();
        stand_in.answers(0, &answered(&record()), "", 0);
        let (cli, token) = (stand_in.cli(), stand_in.token());
        // timber never opens the token's file, so it may be one timber
        // cannot read.
        fs::set_permissions(&token, fs::Permissions::from_mode(0o000)).unwrap();
        let at = at(&token);
        cli.marker(&at, &marker().id()).unwrap();
        cli.put_marker(&at, &marker(), &record()).unwrap();
        cli.apply(&at, &[b"a: 1\n".to_vec()]).unwrap();
        cli.delete(&at, "sluice").unwrap();
        fs::set_permissions(&token, fs::Permissions::from_mode(0o600)).unwrap();
        for run in stand_in.runs() {
            let given = run.args.iter().position(|a| a == "--auth-token-file");
            let path = given.map(|at| run.args[at + 1].as_str());
            assert_eq!(path, Some(token.to_str().unwrap()), "{run:?}");
            for line in run.args.iter().chain(&run.env) {
                assert!(!line.contains(SECRET), "{line}");
            }
        }
    }

    // Requirement: current is unread where the marker does not say

    #[test]
    fn a_marker_dash0_holds_none_of_is_no_marker() {
        let (_dir, stand_in) = stand_in();
        let id = marker().id();
        let words = format!(
            "{{\n  \"error\": \"dashboard \\\"{id}\\\" not found (status: 404, trace_id: 4bf92f)\"\n}}\n"
        );
        stand_in.answers(0, "", &words, 1);
        let token = stand_in.token();
        assert_eq!(stand_in.cli().marker(&at(&token), &id), Ok(None));
        // Another dashboard's absence is no word on the marker's.
        let other = "{\"error\": \"dashboard \\\"overview\\\" not found (status: 404)\"}";
        stand_in.answers(1, "", other, 1);
        assert!(stand_in.cli().marker(&at(&token), &id).is_err());
    }

    // Requirement: Dash0's words are the answer

    #[test]
    fn another_failure_is_unreadable_naming_the_command_and_carrying_its_words_verbatim() {
        let (_dir, stand_in) = stand_in();
        let words = "{\n  \"error\": \"access denied; check your permissions (status: 403)\"\n}";
        stand_in.answers(0, "", &format!("{words}\n"), 1);
        stand_in.answers(1, "", &format!("{words}\n"), 1);
        let token = stand_in.token();
        let cli = stand_in.cli();
        for answer in [
            cli.marker(&at(&token), &marker().id()).map(drop),
            cli.put_marker(&at(&token), &marker(), &record()),
        ] {
            match answer {
                Err(Not::Unreadable(u)) => {
                    assert_eq!(u.source, source());
                    assert_eq!(u.said.as_deref(), Some(words));
                    let program = stand_in.dir.join("dash0").display().to_string();
                    assert!(
                        u.what.starts_with(&format!("{program} --agent-mode")),
                        "{}",
                        u.what
                    );
                }
                other => panic!("answered {other:?}, not unreadable"),
            }
        }
    }

    // Requirement: No run of the CLI reads the host's own settings

    #[test]
    fn the_hosts_own_dash0_settings_reach_no_run() {
        let (dir, stand_in) = stand_in();
        stand_in.answers(0, "[]", "", 0);
        stand_in.answers(1, &answered(&record()), "", 0);
        let home = dir.path().join("home");
        fs::create_dir_all(home.join(".dash0")).unwrap();
        fs::write(home.join(".dash0/config.yaml"), "profile: host\n").unwrap();
        let child = Command::new(std::env::current_exe().unwrap())
            .args([
                "profiles::target::dash0::cli::tests::a_child_runs_every_operation",
                "--exact",
                "--ignored",
                "--test-threads=1",
            ])
            .env(CHILD, dir.path())
            .env("HOME", &home)
            .env("DASH0_AUTH_TOKEN", SECRET)
            .env("DASH0_PROFILE", "host")
            .env("DASH0_CONFIG_DIR", home.join(".dash0"))
            .output()
            .unwrap();
        let printed = String::from_utf8_lossy(&child.stdout);
        assert!(child.status.success(), "{printed}");
        assert!(
            printed.contains("1 passed"),
            "the child ran no test: {printed}"
        );
        let runs = stand_in.runs();
        assert_eq!(runs.len(), 5, "{runs:?}");
        of_their_own(&runs, Some(&home));
        for run in &runs {
            assert!(!run.args.iter().any(|a| a.contains(SECRET)), "{run:?}");
        }
    }

    /// Every operation, run against the stand-in in the folder `CHILD`
    /// names, by a child of [`the_hosts_own_dash0_settings_reach_no_run`]
    /// whose environment holds the host's own `DASH0_*` variables; it
    /// does nothing in a run of the suite itself.
    #[test]
    #[ignore = "run by the_hosts_own_dash0_settings_reach_no_run, in a child holding DASH0_ variables"]
    fn a_child_runs_every_operation() {
        let Some(dir) = std::env::var_os(CHILD) else {
            return;
        };
        for name in ["DASH0_AUTH_TOKEN", "DASH0_PROFILE", "DASH0_CONFIG_DIR"] {
            assert!(std::env::var_os(name).is_some(), "{name} is not set");
        }
        let stand_in = StandIn {
            dir: PathBuf::from(dir),
        };
        let (cli, token) = (stand_in.cli(), stand_in.token());
        let at = at(&token);
        cli.validate(&[b"a: 1\n".to_vec()]).unwrap();
        cli.marker(&at, &marker().id()).unwrap();
        cli.put_marker(&at, &marker(), &record()).unwrap();
        cli.apply(&at, &[b"a: 1\n".to_vec()]).unwrap();
        cli.delete(&at, "sluice").unwrap();
    }
}
