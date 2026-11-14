//! The Dash0 target (C.12's Dash0 row), bound as `dash0`: one dataset of
//! Dash0 converged on the dashboards its environment's version holds, each
//! by its fixed identifier, from the host that holds the dataset's write
//! token (design D1 to D8).
//!
//! An environment names it in its `target` table, which names the API's
//! address, the dataset, the host's credential and the component's
//! outputs whose files are dashboard files, and nothing else (45, C.10;
//! design D2):
//!
//! ```toml
//! [environments.dashboards.deneb]
//! channel = "stable"
//! target.dash0 = { api = "https://api.us-east-1.aws.dash0.com", dataset = "deneb", credential = "dash0-dashboards", dashboards = ["dashboards"] }
//! ```
//!
//! The token is the host's: the bindings file's `[credentials]` names the
//! path of the file holding it, and the target hands that path to Dash0's
//! CLI as `--auth-token-file`, never opening the file (118, 64, rule
//! core.21; design D3). Every run of the CLI is given each setting as a
//! flag, in a configuration folder of its own, with no `DASH0_*` variable
//! (see [`cli`]; design D4).
//!
//! Apply runs these steps, and stops at the first that fails, answering it
//! (90; design D7):
//!
//! 1. The table and the credential; the verified version's index, read
//!    through the profile's own store by digest, refused where it names
//!    another publishing repository than the checkout's, or none; and the
//!    file of each output `dashboards` names, read by digest through
//!    [`file_artifact`] (42, rule core.4).
//! 2. One dry run of the CLI over the files, with no token, which reads
//!    every document's kind and identifier; the version is refused where a
//!    document is no `Dashboard` or `PersesDashboard`, a dashboard has no
//!    identifier, two give one identifier, one gives the marker's, or an
//!    output's file holds no dashboard (design D5). Nothing has reached
//!    Dash0.
//! 3. The marker read (see [`marker`]): what it names is every dashboard
//!    timber may have applied to the dataset for the environment.
//! 4. The marker written naming those and every identifier of the version,
//!    and the version being applied, before any dashboard is.
//! 5. Every file applied with one `dash0 apply`.
//! 6. Each dashboard the marker named that the version does not hold
//!    deleted, and no other: a dashboard made by hand is never named, so
//!    never removed (C.12, rule core.75).
//! 7. The marker written naming the version and its identifiers alone.
//!
//! current and report read the marker and answer the version it names as
//! applied in full, and report when Dash0 says it was last written; they
//! answer unread where there is no marker, where it is deleted, where it
//! names an apply that did not finish, or where it is not timber's record
//! of the environment, and never guess (89, 91, 119; design D8). desired
//! answers the head the core hands it (88).

mod cli;
mod marker;
#[cfg(test)]
mod scripted;
mod table;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::Path;

use timber_seams::{
    Answer, Digest, Environment, Head, Not, RepositoryName, Running, Seam, Source, Store, Target,
    Verified, Why,
};

use super::file_artifact::{self, Words};
use crate::bindings::Credentials;
use cli::{At, Backend, Cli, Document};
use marker::{Applying, Held, Marker, Record};
use table::{Named, Table};

/// The profile name an environment's `target` table names this target by.
pub const PROFILE: &str = "dash0";

/// A dashboard file, as the family module names one in an answer.
const WORDS: Words = Words {
    file: "dashboard file",
    carries: "dashboard",
};

/// The kinds of dashboard Dash0's CLI applies, as its dry run names them.
const KINDS: [&str; 2] = ["Dashboard", "PersesDashboard"];

/// The Dash0 target, reading through a store binding of its own, opened
/// and never made, and running Dash0's CLI.
pub struct Dash0 {
    store: Box<dyn Store>,
    repository: Answer<RepositoryName>,
    credentials: Credentials,
    backend: Box<dyn Backend>,
    source: Source,
}

impl Dash0 {
    /// The target reading through `store`, its markers named for
    /// `repository`, the checkout's, the token of each environment the
    /// file of `credentials` names, and running `dash0` as found on
    /// `PATH`.
    pub fn new(
        store: Box<dyn Store>,
        repository: Answer<RepositoryName>,
        credentials: &Credentials,
    ) -> Dash0 {
        Dash0::with_backend(
            store,
            repository,
            credentials,
            Box::new(Cli::new("dash0", &Dash0::source())),
        )
    }

    fn with_backend(
        store: Box<dyn Store>,
        repository: Answer<RepositoryName>,
        credentials: &Credentials,
        backend: Box<dyn Backend>,
    ) -> Dash0 {
        Dash0 {
            store,
            repository,
            credentials: credentials.clone(),
            backend,
            source: Dash0::source(),
        }
    }

    /// The target seam and this profile, as every answer names them.
    pub fn source() -> Source {
        Source::new(Seam::Target, PROFILE)
    }

    /// Refused, by this target, for `said`.
    fn refused(&self, said: String) -> Not {
        Not::refused(&self.source, Why::Said, Some(said))
    }

    /// The path of the file holding the table's credential, as the host's
    /// `[credentials]` names it; unreadable naming the credential and the
    /// bindings file where it names none, so nothing runs (118; design
    /// D3). The file is never opened.
    fn token<'a>(&'a self, environment: &Environment, table: &Table) -> Answer<&'a Path> {
        let name = &table.credential;
        self.credentials.path(name).ok_or_else(|| {
            Not::unreadable(
                &self.source,
                format!("credential {name} of {}", Named(environment)),
                Some(format!(
                    "the bindings file {} names no {name} in its [credentials], the path of the \
                     file holding the token the {PROFILE} target hands Dash0's CLI",
                    self.credentials.file.display()
                )),
            )
        })
    }

    /// The marker of `environment`, named for the checkout's repository,
    /// or why the repository is unread.
    fn marker(&self, environment: &Environment) -> Answer<(RepositoryName, Marker)> {
        let repository = self.repository.clone()?;
        let marker = Marker::of(&repository, environment);
        Ok((repository, marker))
    }

    /// What `environment` runs, as its marker says: the index the marker
    /// names as applied in full, and the marker as Dash0 holds it; unread
    /// naming why where the marker does not say (89, 119; design D8).
    fn running(&self, environment: &Environment) -> Answer<(Digest, Held)> {
        let table = Table::read(environment, &self.source)?;
        let token = self.token(environment, &table)?;
        let (_, marker) = self.marker(environment)?;
        let at = At {
            api: &table.api,
            dataset: &table.dataset,
            token,
        };
        let id = marker.id();
        let dataset = &table.dataset;
        let unread = |said: String| {
            Not::unreadable(
                &self.source,
                format!("what {} runs", Named(environment)),
                Some(said),
            )
        };
        let Some(held) = self.backend.marker(&at, &id)? else {
            return Err(unread(format!(
                "the dataset {dataset} at {} holds no marker {id} of timber's: it holds nothing \
                 timber applied for the environment, or what it holds was applied otherwise",
                table.api
            )));
        };
        if let Some(deleted) = &held.deleted_at {
            return Err(unread(format!(
                "the marker {id} in the dataset {dataset} was deleted at {deleted}, as Dash0 \
                 says, and the next apply writes it back"
            )));
        }
        let record = marker.read(&held).map_err(|holds| {
            unread(format!("the marker {id} in the dataset {dataset}: {holds}"))
        })?;
        let applied = record.applied();
        if let Some(applying) = &record.applying {
            let over = match &applied {
                Some(applied) => format!("version {} at {}", applied.version, applied.index),
                None => "none".into(),
            };
            return Err(unread(format!(
                "the marker {id} in the dataset {dataset} names an apply of version {} at {} \
                 over {over} that did not finish",
                applying.version, applying.index
            )));
        }
        let Some(applied) = applied else {
            return Err(unread(format!(
                "the marker {id} in the dataset {dataset} names no version applied in full"
            )));
        };
        Ok((applied.index, held))
    }

    /// The index of `version` read by digest, refused where it names
    /// another publishing repository than the checkout's, `repository`, or
    /// none, since its marker would be one current never reads; and the
    /// file of each output the table names, read by digest (42, rule
    /// core.4).
    fn files(
        &self,
        environment: &Environment,
        table: &Table,
        repository: &RepositoryName,
        version: &Verified,
    ) -> Answer<Vec<Vec<u8>>> {
        let index = file_artifact::index(self.store.as_ref(), version, &self.source)?;
        let (digest, number) = (version.index(), version.version());
        match index.repository() {
            Some(named) if named == repository => {}
            Some(named) => {
                return Err(self.refused(format!(
                    "the index {digest} of version {number} names the publishing repository \
                     {named}, and this checkout is of {repository}, under which the marker of {} \
                     is kept",
                    Named(environment)
                )))
            }
            None => {
                return Err(self.refused(format!(
                    "the index {digest} of version {number} names no publishing repository, and \
                     the marker of {} is kept under this checkout's, {repository}",
                    Named(environment)
                )))
            }
        }
        let mut files = Vec::new();
        for output in &table.dashboards {
            let artifact = file_artifact::artifact_of(&index, output, WORDS, &self.source)?;
            files.push(file_artifact::bytes(
                self.store.as_ref(),
                artifact,
                WORDS,
                &self.source,
            )?);
        }
        Ok(files)
    }

    /// The identifiers of the version, read from `documents`, what the dry
    /// run read of the table's outputs' files, in their order; refused
    /// naming the output and the document where a document is no
    /// dashboard, has no identifier, gives the marker's or one another
    /// gives, or where an output's file holds no dashboard (design D5).
    fn identifiers(
        &self,
        environment: &Environment,
        table: &Table,
        marker: &str,
        version: &Verified,
        documents: &[Document],
    ) -> Answer<Vec<String>> {
        let mut first: BTreeMap<&str, usize> = BTreeMap::new();
        let mut holds = vec![false; table.dashboards.len()];
        let mut identifiers = Vec::new();
        for document in documents {
            let Some(output) = table.dashboards.get(document.file) else {
                return Err(Not::unreadable(
                    &self.source,
                    "what the dry run of Dash0's CLI read",
                    Some(format!(
                        "it named file {} of {} handed it",
                        document.file,
                        table.dashboards.len()
                    )),
                ));
            };
            let (kind, name, id) = (&document.kind, &document.name, &document.id);
            if !KINDS.contains(&kind.as_str()) {
                return Err(self.refused(format!(
                    "the output {output} holds a document of the kind {kind}, {name:?}, and the \
                     {PROFILE} target applies dashboards alone, each a Dashboard or a \
                     PersesDashboard"
                )));
            }
            if id.is_empty() {
                return Err(self.refused(format!(
                    "the output {output} holds the {kind} {name:?} with no identifier, which \
                     Dash0 would create anew at every apply: a Dashboard names its identifier \
                     in metadata.dash0Extensions.id, and a PersesDashboard in its label \
                     dash0.com/id"
                )));
            }
            if id == marker {
                return Err(self.refused(format!(
                    "the output {output} holds the {kind} {name:?}, whose identifier {id} is the \
                     marker timber keeps for {}",
                    Named(environment)
                )));
            }
            if let Some(place) = first.insert(id.as_str(), document.file) {
                let other = &table.dashboards[place];
                return Err(self.refused(if place == document.file {
                    format!("the output {output} gives the identifier {id} to two documents")
                } else {
                    format!("the outputs {other} and {output} each give the identifier {id}")
                }));
            }
            holds[document.file] = true;
            identifiers.push(id.clone());
        }
        if let Some(empty) = holds.iter().position(|held| !held) {
            return Err(self.refused(format!(
                "the {} of the output {} in version {} holds no dashboard",
                WORDS.file,
                table.dashboards[empty],
                version.version()
            )));
        }
        Ok(identifiers)
    }

    /// The marker's record before this apply, where Dash0 holds one, deleted
    /// or not; unreadable where it is not timber's record of the
    /// environment, since what timber applied there is then not known.
    fn before(&self, at: &At<'_>, marker: &Marker) -> Answer<Option<Record>> {
        let id = marker.id();
        let Some(held) = self.backend.marker(at, &id)? else {
            return Ok(None);
        };
        marker.read(&held).map(Some).map_err(|holds| {
            Not::unreadable(
                &self.source,
                format!("the marker {id} in the dataset {}", at.dataset),
                Some(format!(
                    "{holds}, so what timber applied there is not known, and nothing is applied"
                )),
            )
        })
    }
}

impl Target for Dash0 {
    /// 88: the head the core handed, as a target that is pushed to answers
    /// it, once the table is read.
    fn desired(&self, environment: &Environment, head: Head<'_>) -> Answer<Digest> {
        Table::read(environment, &self.source)?;
        head()
    }

    /// 89: the index the marker names as applied in full.
    fn current(&self, environment: &Environment) -> Answer<Digest> {
        self.running(environment).map(|(index, _)| index)
    }

    /// 90: the version's dashboards applied by their identifiers, every
    /// one named in the marker before any is applied, and each the marker
    /// named that the version does not hold removed after (design D7).
    fn apply(&self, environment: &Environment, version: &Verified) -> Answer<()> {
        let table = Table::read(environment, &self.source)?;
        let token = self.token(environment, &table)?;
        let (repository, marker) = self.marker(environment)?;
        let files = self.files(environment, &table, &repository, version)?;
        let documents = self.backend.validate(&files)?;
        let identifiers =
            self.identifiers(environment, &table, &marker.id(), version, &documents)?;
        let at = At {
            api: &table.api,
            dataset: &table.dataset,
            token,
        };
        let before = self.before(&at, &marker)?;
        let applied = before.as_ref().and_then(Record::applied);
        let named = before.map(|record| record.dashboards).unwrap_or_default();
        let this = Applying {
            index: version.index().clone(),
            version: version.version().clone(),
        };
        let applying = marker.record(
            applied.as_ref(),
            named.iter().chain(&identifiers).cloned(),
            Some(this.clone()),
        );
        self.backend.put_marker(&at, &marker, &applying)?;
        self.backend.apply(&at, &files)?;
        for gone in named.iter().filter(|id| !identifiers.contains(*id)) {
            self.backend.delete(&at, gone)?;
        }
        let converged = marker.record(Some(&this), identifiers, None);
        self.backend.put_marker(&at, &marker, &converged)
    }

    /// 91: the index the marker names as applied in full, and when Dash0
    /// says the marker was last written.
    fn report(&self, environment: &Environment) -> Answer<Running> {
        let (digest, held) = self.running(environment)?;
        let converged_at = held.updated_at.ok_or_else(|| {
            Not::unreadable(
                &self.source,
                format!("when {} converged", Named(environment)),
                Some(
                    "Dash0 answered its marker with no updatedAt, so when it was last written \
                     is not known"
                        .into(),
                ),
            )
        })?;
        Ok(Running {
            digest,
            converged_at,
        })
    }
}
