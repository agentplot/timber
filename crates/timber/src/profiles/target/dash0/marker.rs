//! The marker (119; design D6): one dashboard of timber's own in the
//! dataset, beside the dashboards it records, so what timber applied there
//! is read with no timber process running (20) and lasts with them. A
//! dashboard's metadata keeps no field of timber's, and the write token
//! writes dashboards alone, so the record is the marker's description:
//!
//! - Its identifier is `timber-` and the first 32 hexadecimal characters
//!   of the SHA-256 of `<owner>/<name>/<component>/<environment>`, the
//!   owner and name the checkout's repository, so each environment of each
//!   repository keeps one marker in a dataset.
//! - It is a `Dashboard` in Dash0's folder `/timber`, its display name
//!   `timber: <owner>/<name> <component> <environment>`.
//! - Its description is one JSON object of the form `timber/dash0/v1`:
//!   the repository, the component and the environment; `index` and
//!   `version`, the version last applied in full, where there is one;
//!   `dashboards`, every identifier timber may have applied there and not
//!   removed; and `applying`, the version of an apply not finished, where
//!   there is one.
//!
//! The marker is read back from `dashboards get <id> -o json`: its
//! description, `metadata.updatedAt`, and the `dash0.com/deleted-at`
//! annotation Dash0 gives a dashboard it holds deleted.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use timber_seams::{Digest, Environment, RepositoryName, Version};

/// The form of the record the marker's description holds.
pub(super) const FORM: &str = "timber/dash0/v1";

/// The folder of Dash0's UI the marker is kept in.
pub(super) const FOLDER: &str = "/timber";

/// The annotation naming a dashboard's folder.
const FOLDER_PATH: &str = "dash0.com/folder-path";

/// The annotation Dash0 gives a dashboard it holds deleted.
const DELETED_AT: &str = "dash0.com/deleted-at";

/// How many hexadecimal characters of the SHA-256 the identifier holds.
const HASH_LEN: usize = 32;

/// Whose marker it is: the checkout's repository, the component and the
/// environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Marker {
    repository: String,
    component: String,
    environment: String,
}

/// A version as the record names it: its index and its version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Applying {
    pub index: Digest,
    pub version: Version,
}

/// The record the marker's description holds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Record {
    pub marker: String,
    pub repository: String,
    pub component: String,
    pub environment: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<Digest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    pub dashboards: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applying: Option<Applying>,
}

/// The marker as Dash0 answers it: its description, when Dash0 says it
/// was last written, and when it was deleted, where Dash0 holds it
/// deleted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Held {
    pub description: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl Marker {
    /// The marker of `environment` in the checkout of `repository`.
    pub fn of(repository: &RepositoryName, environment: &Environment) -> Marker {
        Marker {
            repository: format!("{}/{}", repository.owner, repository.name),
            component: environment.channel.component_name().as_str().to_string(),
            environment: environment.name.as_str().to_string(),
        }
    }

    /// Its identifier: `timber-` and the first 32 hexadecimal characters
    /// of the SHA-256 of `<owner>/<name>/<component>/<environment>`.
    pub fn id(&self) -> String {
        let named = format!(
            "{}/{}/{}",
            self.repository, self.component, self.environment
        );
        let hash: String = Sha256::digest(named.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        format!("timber-{}", &hash[..HASH_LEN])
    }

    /// Its display name, as Dash0's UI shows it.
    pub fn display_name(&self) -> String {
        format!(
            "timber: {} {} {}",
            self.repository, self.component, self.environment
        )
    }

    /// The record naming `applied`, the version last applied in full
    /// where there is one, every identifier of `dashboards`, sorted and
    /// each once, and `applying`, an apply not finished.
    pub fn record(
        &self,
        applied: Option<&Applying>,
        dashboards: impl IntoIterator<Item = String>,
        applying: Option<Applying>,
    ) -> Record {
        let mut dashboards: Vec<String> = dashboards.into_iter().collect();
        dashboards.sort();
        dashboards.dedup();
        Record {
            marker: FORM.into(),
            repository: self.repository.clone(),
            component: self.component.clone(),
            environment: self.environment.clone(),
            index: applied.map(|a| a.index.clone()),
            version: applied.map(|a| a.version.clone()),
            dashboards,
            applying,
        }
    }

    /// The `Dashboard` document timber writes for `record`, as JSON, which
    /// Dash0's CLI reads as YAML: in the folder `/timber`, by the
    /// marker's identifier, the record its description.
    pub fn document(&self, record: &Record) -> Vec<u8> {
        let id = self.id();
        let description =
            serde_json::to_string(record).expect("a record is always written as JSON");
        let document = json!({
            "kind": "Dashboard",
            "metadata": {
                "name": id,
                "annotations": { FOLDER_PATH: FOLDER },
                "dash0Extensions": { "id": id },
            },
            "spec": {
                "display": {
                    "name": self.display_name(),
                    "description": description,
                },
                "duration": "1h",
                "panels": {},
                "layouts": [],
            },
        });
        let mut bytes =
            serde_json::to_vec_pretty(&document).expect("a document is always written as JSON");
        bytes.push(b'\n');
        bytes
    }

    /// The record `held` holds, where its description is a record of the
    /// form `timber/dash0/v1` naming this marker's repository, component
    /// and environment; otherwise what it holds, never guessed (119).
    pub fn read(&self, held: &Held) -> Result<Record, String> {
        let Some(description) = held.description.as_deref() else {
            return Err(format!(
                "it has no description, so it holds no record of the form {FORM}"
            ));
        };
        let record: Record = serde_json::from_str(description).map_err(|e| {
            format!("its description, {description:?}, is no record of the form {FORM}: {e}")
        })?;
        if record.marker != FORM {
            return Err(format!(
                "its record is of the form {}, and timber reads {FORM}",
                record.marker
            ));
        }
        let ours = (&self.repository, &self.component, &self.environment);
        if (&record.repository, &record.component, &record.environment) != ours {
            return Err(format!(
                "its record names the environment {} of {} in {}, and this marker is of the \
                 environment {} of {} in {}",
                record.environment,
                record.component,
                record.repository,
                self.environment,
                self.component,
                self.repository
            ));
        }
        Ok(record)
    }
}

impl Record {
    /// The version the record names as applied in full, where it names
    /// one.
    pub fn applied(&self) -> Option<Applying> {
        match (&self.index, &self.version) {
            (Some(index), Some(version)) => Some(Applying {
                index: index.clone(),
                version: version.clone(),
            }),
            _ => None,
        }
    }
}

impl Held {
    /// What `dashboards get <id> -o json` printed, read as the marker:
    /// its description, its `metadata.updatedAt`, and its
    /// `dash0.com/deleted-at` annotation; what keeps it from being a
    /// dashboard otherwise.
    pub fn parse(printed: &[u8]) -> Result<Held, String> {
        let dashboard: Value = serde_json::from_slice(printed)
            .map_err(|e| format!("it printed no dashboard in JSON: {e}"))?;
        let metadata = dashboard
            .get("metadata")
            .and_then(Value::as_object)
            .ok_or_else(|| "it printed JSON naming no dashboard's metadata".to_string())?;
        let text = |value: Option<&Value>| value.and_then(Value::as_str).map(str::to_string);
        Ok(Held {
            description: text(dashboard.pointer("/spec/display/description")),
            updated_at: text(metadata.get("updatedAt")),
            deleted_at: text(
                metadata
                    .get("annotations")
                    .and_then(|annotations| annotations.get(DELETED_AT)),
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use timber_seams::{ChannelId, ChannelName, ComponentName, EnvironmentName};

    use super::*;

    fn repository() -> RepositoryName {
        RepositoryName {
            owner: "madswan-dev".into(),
            name: "dashboards".into(),
        }
    }

    fn environment(name: &str) -> Environment {
        Environment {
            name: EnvironmentName::new(name),
            channel: ChannelId::new(
                Some(ComponentName::new("dashboards")),
                ChannelName::new("stable"),
            ),
            waits: false,
            settings: json!({}),
        }
    }

    fn deneb() -> Marker {
        Marker::of(&repository(), &environment("deneb"))
    }

    fn version(text: &str) -> Applying {
        Applying {
            index: Digest::of(text.as_bytes()),
            version: Version::new(text),
        }
    }

    /// What Dash0's CLI prints for the document timber wrote for
    /// `record`, as `dashboards get -o json` answers one: the document
    /// with Dash0's own fields beside timber's, and the deletion where
    /// one is given.
    fn answered(record: &Record, deleted_at: Option<&str>) -> Vec<u8> {
        let mut document: Value = serde_json::from_slice(&deneb().document(record)).unwrap();
        let metadata = document["metadata"].as_object_mut().unwrap();
        metadata.insert("createdAt".into(), json!("2026-11-14T09:00:00Z"));
        metadata.insert("updatedAt".into(), json!("2026-11-14T10:00:00Z"));
        metadata.insert("version".into(), json!(3));
        metadata.insert(
            "dash0Extensions".into(),
            json!({ "id": deneb().id(), "dataset": "deneb" }),
        );
        if let Some(at) = deleted_at {
            metadata["annotations"][DELETED_AT] = json!(at);
        }
        serde_json::to_vec(&document).unwrap()
    }

    // Requirement: The marker is a dashboard of timber's own in the dataset

    #[test]
    fn the_identifier_is_the_hash_of_the_repository_the_component_and_the_environment() {
        let hash: String = Sha256::digest(b"madswan-dev/dashboards/dashboards/deneb")
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let id = deneb().id();
        assert_eq!(id, format!("timber-{}", &hash[..32]));
        assert_eq!(id.len(), "timber-".len() + 32);
        assert_ne!(Marker::of(&repository(), &environment("vega")).id(), id);
    }

    #[test]
    fn the_document_is_a_dashboard_in_the_timber_folder_named_for_whose_it_is() {
        let record = deneb().record(None, ["overview".to_string()], None);
        let document: Value = serde_json::from_slice(&deneb().document(&record)).unwrap();
        assert_eq!(document["kind"], "Dashboard");
        assert_eq!(document["metadata"]["dash0Extensions"]["id"], deneb().id());
        assert_eq!(
            document["metadata"]["annotations"]["dash0.com/folder-path"],
            "/timber"
        );
        assert_eq!(
            document["spec"]["display"]["name"],
            "timber: madswan-dev/dashboards dashboards deneb"
        );
        let description = document["spec"]["display"]["description"].as_str().unwrap();
        let written: Value = serde_json::from_str(description).unwrap();
        assert_eq!(
            written,
            json!({
                "marker": "timber/dash0/v1",
                "repository": "madswan-dev/dashboards",
                "component": "dashboards",
                "environment": "deneb",
                "dashboards": ["overview"],
            })
        );
    }

    // Requirement: The marker's description is its record

    #[test]
    fn a_record_written_is_read_back_from_dash0s_answer() {
        let applied = version("1.3.0");
        let record = deneb().record(
            Some(&applied),
            ["sluice".to_string(), "overview".into(), "sluice".into()],
            None,
        );
        assert_eq!(record.dashboards, ["overview", "sluice"]);
        let held = Held::parse(&answered(&record, None)).unwrap();
        assert_eq!(held.updated_at.as_deref(), Some("2026-11-14T10:00:00Z"));
        assert_eq!(held.deleted_at, None);
        let read = deneb().read(&held).unwrap();
        assert_eq!(read, record);
        assert_eq!(read.applied(), Some(applied.clone()));
        assert_eq!(read.index, Some(applied.index));
        assert_eq!(read.version, Some(Version::new("1.3.0")));
        assert_eq!(read.applying, None);
    }

    #[test]
    fn a_record_in_progress_names_both_versions() {
        let record = deneb().record(
            Some(&version("1.3.0")),
            ["overview".to_string(), "new".into()],
            Some(version("1.4.0")),
        );
        let held = Held::parse(&answered(&record, None)).unwrap();
        let read = deneb().read(&held).unwrap();
        assert_eq!(read.applying, Some(version("1.4.0")));
        assert_eq!(read.applied(), Some(version("1.3.0")));
        let description: Value =
            serde_json::from_str(held.description.as_deref().unwrap()).unwrap();
        assert_eq!(
            description["applying"],
            json!({ "index": Digest::of(b"1.4.0"), "version": "1.4.0" })
        );
        let first = deneb().record(None, ["overview".to_string()], Some(version("1.4.0")));
        let read = deneb()
            .read(&Held::parse(&answered(&first, None)).unwrap())
            .unwrap();
        assert_eq!(read.applied(), None);
        assert_eq!(read.applying, Some(version("1.4.0")));
    }

    #[test]
    fn a_deleted_answer_is_read_with_when_it_was_deleted() {
        let record = deneb().record(Some(&version("1.3.0")), ["overview".to_string()], None);
        let held = Held::parse(&answered(&record, Some("2026-11-14T11:00:00Z"))).unwrap();
        assert_eq!(held.deleted_at.as_deref(), Some("2026-11-14T11:00:00Z"));
        assert_eq!(deneb().read(&held), Ok(record));
    }

    #[test]
    fn an_answer_of_another_form_or_naming_another_environment_is_never_read_as_a_record() {
        let vega = Marker::of(&repository(), &environment("vega"));
        let theirs = vega.record(Some(&version("1.3.0")), ["overview".to_string()], None);
        let held = Held {
            description: Some(serde_json::to_string(&theirs).unwrap()),
            updated_at: None,
            deleted_at: None,
        };
        assert_eq!(
            deneb().read(&held),
            Err(
                "its record names the environment vega of dashboards in madswan-dev/dashboards, \
                 and this marker is of the environment deneb of dashboards in \
                 madswan-dev/dashboards"
                    .into()
            )
        );
        let mut later = deneb().record(None, Vec::new(), None);
        later.marker = "timber/dash0/v2".into();
        let held = Held {
            description: Some(serde_json::to_string(&later).unwrap()),
            ..held
        };
        assert_eq!(
            deneb().read(&held),
            Err(
                "its record is of the form timber/dash0/v2, and timber reads timber/dash0/v1"
                    .into()
            )
        );
        let made_by_hand = Held {
            description: Some("Our overview".into()),
            ..held.clone()
        };
        let said = deneb().read(&made_by_hand).unwrap_err();
        assert!(
            said.starts_with("its description, \"Our overview\", is no record of the form"),
            "{said}"
        );
        let undescribed = Held {
            description: None,
            ..held
        };
        assert!(deneb().read(&undescribed).is_err());
        for printed in [&b"not json"[..], b"[]", b"{\"kind\":\"Dashboard\"}"] {
            assert!(Held::parse(printed).is_err(), "{printed:?}");
        }
    }
}
