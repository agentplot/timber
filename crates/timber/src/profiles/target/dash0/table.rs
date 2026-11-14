//! The environment's `dash0` table, read at every operation of the
//! target: `api`, the address of Dash0's API, `https://` alone, since the
//! token goes there; `dataset`, the dataset's identifier; `credential`,
//! the name of the host's credential its `[credentials]` gives the path
//! of; and `dashboards`, the component's outputs whose files are
//! dashboard files. All four are required, and no other key is read, so
//! no token, path or profile sits in the table (45, C.10, C.12; design
//! D2). A key found wanting is named with the four the table reads, and
//! its value is in no words, since a value of another form may be a
//! credential.

use std::collections::BTreeMap;
use std::fmt;

use serde_json::Value;
use timber_seams::{Answer, Environment, Not, OutputName, Source};

use super::PROFILE;

/// The key naming the address of Dash0's API.
pub(super) const API: &str = "api";

/// The key naming the dataset.
pub(super) const DATASET: &str = "dataset";

/// The key naming the host's credential.
pub(super) const CREDENTIAL: &str = "credential";

/// The key naming the outputs whose files are dashboard files.
pub(super) const DASHBOARDS: &str = "dashboards";

/// The scheme an address of Dash0's API begins with.
const HTTPS: &str = "https://";

/// What an environment's table names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Table {
    pub api: String,
    pub dataset: String,
    pub credential: String,
    pub dashboards: Vec<OutputName>,
}

/// An environment as an answer names it: by its name and its component.
pub(super) struct Named<'a>(pub &'a Environment);

impl fmt::Display for Named<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let environment = self.0;
        write!(
            f,
            "environment {} of {}",
            environment.name,
            environment.channel.component_name()
        )
    }
}

/// The words every refusal of the table ends with.
fn reads() -> String {
    format!("the {PROFILE} target reads {API}, {DATASET}, {CREDENTIAL} and {DASHBOARDS}")
}

impl Table {
    /// `environment`'s settings read as the table, or unreadable naming
    /// the environment, the first key found wanting and the four the table
    /// reads: one the target does not read, or a required one missing or
    /// of another form.
    pub fn read(environment: &Environment, source: &Source) -> Answer<Table> {
        let named = Named(environment);
        let wanting = |key: &str, said: String| {
            Not::unreadable(
                source,
                format!("setting {key} of {named}"),
                Some(format!("{said}; {}", reads())),
            )
        };
        let Some(settings) = environment.settings.as_object() else {
            return Err(Not::unreadable(
                source,
                format!("the {PROFILE} table of {named}"),
                Some(format!("the settings are no table, and {}", reads())),
            ));
        };
        let known = [API, DATASET, CREDENTIAL, DASHBOARDS];
        if let Some(key) = settings.keys().find(|k| !known.contains(&k.as_str())) {
            return Err(wanting(
                key,
                format!(
                    "the table names {key}, which the {PROFILE} target does not read: the token \
                     is the host's, named by {CREDENTIAL} in the host's [credentials]"
                ),
            ));
        }
        let text =
            |key: &str, what: &str, problem: fn(&str) -> Option<String>| match settings.get(key) {
                None => Err(wanting(
                    key,
                    format!("the table names no {key}, which is required: {what}"),
                )),
                Some(Value::String(text)) => match problem(text) {
                    None => Ok(text.clone()),
                    Some(problem) => Err(wanting(key, format!("{key} is no {what}: {problem}"))),
                },
                Some(_) => Err(wanting(
                    key,
                    format!("{key} is no {what}: it is not a string"),
                )),
            };
        let api = text(API, "address https://<host> of Dash0's API", api_problem)?;
        let dataset = text(DATASET, "dataset's identifier", dataset_problem)?;
        let credential = text(
            CREDENTIAL,
            "name of a credential the host's [credentials] gives",
            credential_problem,
        )?;
        let what = "list of the component's outputs whose files are dashboard files";
        let dashboards = match settings.get(DASHBOARDS) {
            None => {
                return Err(wanting(
                    DASHBOARDS,
                    format!("the table names no {DASHBOARDS}, which is required: a {what}"),
                ))
            }
            Some(value) => outputs(value).map_err(|problem| {
                wanting(DASHBOARDS, format!("{DASHBOARDS} is no {what}: {problem}"))
            })?,
        };
        Ok(Table {
            api,
            dataset,
            credential,
            dashboards,
        })
    }
}

/// What keeps `api` from being an address of Dash0's API: no `https://`,
/// no host after it, or whitespace.
fn api_problem(api: &str) -> Option<String> {
    let Some(host) = api.strip_prefix(HTTPS) else {
        return Some(format!(
            "it does not begin {HTTPS}, and the token is sent to it"
        ));
    };
    if host.is_empty() || host.starts_with('/') {
        return Some(format!("it names no host after {HTTPS}"));
    }
    whitespace(api)
}

/// What keeps `dataset` from being a dataset's identifier: nothing, a
/// `/`, a leading `-`, or whitespace.
fn dataset_problem(dataset: &str) -> Option<String> {
    if dataset.is_empty() {
        return Some("it is empty".into());
    }
    if dataset.contains('/') {
        return Some("it holds `/`".into());
    }
    if dataset.starts_with('-') {
        return Some("it begins with `-`".into());
    }
    whitespace(dataset)
}

/// What keeps `credential` from being a credential's name: nothing, or
/// whitespace.
fn credential_problem(credential: &str) -> Option<String> {
    if credential.is_empty() {
        return Some("it is empty".into());
    }
    whitespace(credential)
}

fn whitespace(text: &str) -> Option<String> {
    text.chars()
        .any(char::is_whitespace)
        .then(|| "it holds whitespace".into())
}

/// `value` read as a non-empty list of output names, each a non-empty
/// string named once; what keeps it from being one, by the items'
/// places, otherwise.
fn outputs(value: &Value) -> Result<Vec<OutputName>, String> {
    let Value::Array(items) = value else {
        return Err("it is not a list".into());
    };
    if items.is_empty() {
        return Err("it names no output".into());
    }
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    let mut names = Vec::new();
    for (place, item) in (1..).zip(items) {
        let name = match item {
            Value::String(name) if !name.is_empty() => name.as_str(),
            _ => return Err(format!("its item {place} is no output's name")),
        };
        if let Some(first) = seen.insert(name, place) {
            return Err(format!("its items {first} and {place} name one output"));
        }
        names.push(OutputName::new(name));
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use timber_seams::{ChannelId, ChannelName, ComponentName, EnvironmentName, Seam};

    use super::*;

    const ADDRESS: &str = "https://api.us-east-1.aws.dash0.com";

    fn source() -> Source {
        Source::new(Seam::Target, PROFILE)
    }

    fn deneb(settings: Value) -> Environment {
        Environment {
            name: EnvironmentName::new("deneb"),
            channel: ChannelId::new(
                Some(ComponentName::new("dashboards")),
                ChannelName::new("stable"),
            ),
            waits: false,
            settings,
        }
    }

    fn table() -> Value {
        json!({
            "api": ADDRESS,
            "dataset": "deneb",
            "credential": "dash0-dashboards",
            "dashboards": ["dashboards"],
        })
    }

    /// The table with `key` set to `value`.
    fn with(key: &str, value: Value) -> Value {
        let mut settings = table();
        settings[key] = value;
        settings
    }

    /// The table without `key`.
    fn without(key: &str) -> Value {
        let mut settings = table();
        settings.as_object_mut().unwrap().remove(key);
        settings
    }

    /// What `settings` is unreadable naming, and its words, once they
    /// name the four keys the table reads.
    fn wanting(settings: Value) -> (String, String) {
        match Table::read(&deneb(settings.clone()), &source()) {
            Err(Not::Unreadable(u)) => {
                assert_eq!(u.source, source());
                let said = u.said.expect("the target's words");
                assert!(
                    said.ends_with(
                        "the dash0 target reads api, dataset, credential and dashboards"
                    ),
                    "{said}"
                );
                (u.what, said)
            }
            other => panic!("{settings} answered {other:?}"),
        }
    }

    fn setting(key: &str) -> String {
        format!("setting {key} of environment deneb of dashboards")
    }

    // Requirement: The table names the API, the dataset, the credential
    // and the dashboards

    #[test]
    fn the_four_keys_are_read() {
        let settings = with("dashboards", json!(["overview", "sluice"]));
        assert_eq!(
            Table::read(&deneb(settings), &source()),
            Ok(Table {
                api: ADDRESS.into(),
                dataset: "deneb".into(),
                credential: "dash0-dashboards".into(),
                dashboards: vec![OutputName::new("overview"), OutputName::new("sluice")],
            })
        );
    }

    #[test]
    fn each_key_missing_is_unreadable_naming_it_as_required() {
        for key in ["api", "dataset", "credential", "dashboards"] {
            let (what, said) = wanting(without(key));
            assert_eq!(what, setting(key));
            assert!(
                said.starts_with(&format!("the table names no {key}, which is required")),
                "{said}"
            );
        }
    }

    #[test]
    fn an_address_in_the_clear_or_of_no_host_is_unreadable_naming_api() {
        for (api, problem) in [
            (
                "http://api.us-east-1.aws.dash0.com",
                "it does not begin https://, and the token is sent to it",
            ),
            ("api.us-east-1.aws.dash0.com", "it does not begin https://"),
            ("https://", "it names no host after https://"),
            ("https:///api", "it names no host after https://"),
            ("https://api.dash0.com /x", "it holds whitespace"),
        ] {
            let (what, said) = wanting(with("api", json!(api)));
            assert_eq!(what, setting("api"));
            assert!(said.contains(problem), "{api}: {said}");
            if api != HTTPS {
                assert!(!said.contains(api), "{said}");
            }
        }
        let (_, said) = wanting(with("api", json!(443)));
        assert!(said.contains("it is not a string"), "{said}");
    }

    #[test]
    fn a_dataset_that_is_no_identifier_is_unreadable_naming_dataset() {
        for (dataset, problem) in [
            ("", "it is empty"),
            ("deneb/prod", "it holds `/`"),
            ("-deneb", "it begins with `-`"),
            ("de neb", "it holds whitespace"),
        ] {
            let (what, said) = wanting(with("dataset", json!(dataset)));
            assert_eq!(what, setting("dataset"));
            assert!(said.contains(problem), "{dataset:?}: {said}");
        }
        let (what, _) = wanting(with("dataset", json!(["deneb"])));
        assert_eq!(what, setting("dataset"));
    }

    #[test]
    fn a_credential_that_is_no_name_is_unreadable_naming_credential() {
        for (credential, problem) in [
            (json!(""), "it is empty"),
            (json!("dash0 dashboards"), "it holds whitespace"),
            (
                json!({ "path": "/run/secrets/dash0" }),
                "it is not a string",
            ),
        ] {
            let (what, said) = wanting(with("credential", credential.clone()));
            assert_eq!(what, setting("credential"));
            assert!(said.contains(problem), "{credential}: {said}");
            assert!(!said.contains("/run/secrets"), "{said}");
        }
    }

    #[test]
    fn an_empty_or_repeated_dashboards_is_unreadable_naming_dashboards() {
        for (dashboards, problem) in [
            (json!([]), "it names no output"),
            (
                json!(["overview", "sluice", "overview"]),
                "its items 1 and 3 name one output",
            ),
            (json!(["overview", ""]), "its item 2 is no output's name"),
            (json!(["overview", 3]), "its item 2 is no output's name"),
            (json!("overview"), "it is not a list"),
        ] {
            let (what, said) = wanting(with("dashboards", dashboards.clone()));
            assert_eq!(what, setting("dashboards"));
            assert!(said.contains(problem), "{dashboards}: {said}");
        }
    }

    // Requirement: The table reads no other key

    #[test]
    fn a_token_in_the_table_is_unreadable_naming_it_and_the_four_and_never_its_value() {
        let (what, said) = wanting(with("token", json!("auth_abc")));
        assert_eq!(what, setting("token"));
        assert!(
            said.starts_with("the table names token, which the dash0 target does not read"),
            "{said}"
        );
        assert!(!said.contains("auth_abc"), "{said}");
        assert!(!what.contains("auth_abc"), "{what}");
    }

    #[test]
    fn settings_that_are_no_table_are_unreadable_naming_the_environment() {
        for settings in [json!("auth_abc"), json!(["deneb"]), json!(null)] {
            let (what, said) = wanting(settings.clone());
            assert_eq!(what, "the dash0 table of environment deneb of dashboards");
            assert!(said.starts_with("the settings are no table"), "{said}");
            assert!(!said.contains("auth_abc"), "{said}");
        }
    }
}
