//! A Dash0 that answers as a test sets, in the CLI's place, so the
//! profile's refusals, its order of steps and the target group run in
//! `check-fast` (design D9). It holds the dataset's dashboards and its
//! markers in memory. A file handed it is a test's stand-in for a
//! dashboard file: a JSON list of documents, each `{ kind, name, id }` as
//! the CLI's dry run would read it, which its dry run answers and its
//! apply upserts by identifier. Any operation fails with words a test
//! gives, every time or at its n-th asking, and at a dataset a test makes
//! unreachable; every operation asked is recorded in order.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use serde::Deserialize;
use timber_seams::{Answer, Not, Source};

use super::cli::{At, Backend, Document};
use super::marker::{Held, Marker, Record};

/// When Dash0 says each marker it writes was last written.
pub(super) const CLOCK: &str = "2026-11-14T10:00:00Z";

/// An operation asked, with what it was asked of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Asked {
    /// A dry run of this many files.
    Validate(usize),
    /// A read of the marker of this identifier.
    Marker(String),
    /// A write of the marker holding this record.
    PutMarker(Record),
    /// An apply of the files holding these identifiers.
    Apply(Vec<String>),
    /// A deletion of the dashboard of this identifier.
    Delete(String),
}

impl Asked {
    /// The operation's name, as a test lists what ran.
    pub fn name(&self) -> &'static str {
        match self {
            Asked::Validate(_) => "validate",
            Asked::Marker(_) => "marker",
            Asked::PutMarker(_) => "put_marker",
            Asked::Apply(_) => "apply",
            Asked::Delete(_) => "delete",
        }
    }
}

/// One document of a test's stand-in file.
#[derive(Clone, Debug, Deserialize)]
struct StandIn {
    kind: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    id: String,
}

#[derive(Default)]
struct State {
    dashboards: BTreeSet<String>,
    markers: BTreeMap<String, Held>,
    fails: BTreeMap<&'static str, (usize, String)>,
    unreachable: BTreeMap<String, String>,
    asked: Vec<Asked>,
}

/// The scripted Dash0; a clone shares its state.
#[derive(Clone)]
pub(super) struct Scripted {
    state: Rc<RefCell<State>>,
    source: Source,
}

impl Scripted {
    pub fn new(source: &Source) -> Scripted {
        Scripted {
            state: Rc::default(),
            source: source.clone(),
        }
    }

    /// The dataset holds the dashboard `id`, made by hand or by an earlier
    /// apply.
    pub fn holds(&self, id: &str) {
        self.state.borrow_mut().dashboards.insert(id.into());
    }

    /// The identifiers of every dashboard the dataset holds, the markers
    /// apart.
    pub fn dashboards(&self) -> Vec<String> {
        self.state.borrow().dashboards.iter().cloned().collect()
    }

    /// The marker `id` as Dash0 holds it, where it holds one.
    pub fn held(&self, id: &str) -> Option<Held> {
        self.state.borrow().markers.get(id).cloned()
    }

    /// The marker `id` set to `held`, as a hand or an earlier timber left
    /// it.
    pub fn holds_marker(&self, id: &str, held: Held) {
        self.state.borrow_mut().markers.insert(id.into(), held);
    }

    /// The marker `id` deleted in Dash0's UI at `at`: Dash0 still answers
    /// it, annotated deleted.
    pub fn deletes_marker(&self, id: &str, at: &str) {
        let mut state = self.state.borrow_mut();
        let held = state.markers.get_mut(id).expect("the test wrote no marker");
        held.deleted_at = Some(at.into());
    }

    /// The operation `operation` fails at its `nth` asking, 1 the first, or
    /// at every asking where `nth` is 0, the CLI saying `words`.
    pub fn fails(&self, operation: &'static str, nth: usize, words: &str) {
        let mut state = self.state.borrow_mut();
        state.fails.insert(operation, (nth, words.into()));
    }

    /// Every operation at `dataset` fails, the CLI saying `words`.
    pub fn unreachable(&self, dataset: &str, words: &str) {
        let mut state = self.state.borrow_mut();
        state.unreachable.insert(dataset.into(), words.into());
    }

    /// Every operation asked, in order.
    pub fn asked(&self) -> Vec<Asked> {
        self.state.borrow().asked.clone()
    }

    /// The names of every operation asked, in order.
    pub fn ran(&self) -> Vec<&'static str> {
        self.state.borrow().asked.iter().map(Asked::name).collect()
    }

    /// `asked` recorded, then the words it fails with, where it fails: at
    /// an unreachable dataset, or as the test set it to.
    fn ask(&self, asked: Asked, at: Option<&At<'_>>) -> Answer<()> {
        let name = asked.name();
        let mut state = self.state.borrow_mut();
        state.asked.push(asked);
        let what = format!("dash0 {name}");
        if let Some(words) = at.and_then(|at| state.unreachable.get(at.dataset)) {
            return Err(Not::unreadable(&self.source, what, Some(words.clone())));
        }
        let count = state.asked.iter().filter(|a| a.name() == name).count();
        match state.fails.get(name) {
            Some((nth, words)) if *nth == 0 || *nth == count => {
                Err(Not::unreadable(&self.source, what, Some(words.clone())))
            }
            _ => Ok(()),
        }
    }
}

/// The documents of a test's stand-in file.
fn documents(file: &[u8]) -> Vec<StandIn> {
    serde_json::from_slice(file).unwrap_or_else(|e| {
        panic!(
            "the test handed no list of documents, {e}: {}",
            String::from_utf8_lossy(file)
        )
    })
}

impl Backend for Scripted {
    fn validate(&self, files: &[Vec<u8>]) -> Answer<Vec<Document>> {
        self.ask(Asked::Validate(files.len()), None)?;
        let mut read = Vec::new();
        for (file, bytes) in files.iter().enumerate() {
            for document in documents(bytes) {
                read.push(Document {
                    file,
                    kind: document.kind,
                    name: document.name,
                    id: document.id,
                });
            }
        }
        Ok(read)
    }

    fn marker(&self, at: &At<'_>, id: &str) -> Answer<Option<Held>> {
        self.ask(Asked::Marker(id.into()), Some(at))?;
        Ok(self.held(id))
    }

    fn put_marker(&self, at: &At<'_>, marker: &Marker, record: &Record) -> Answer<()> {
        self.ask(Asked::PutMarker(record.clone()), Some(at))?;
        let held = Held {
            description: Some(serde_json::to_string(record).unwrap()),
            updated_at: Some(CLOCK.into()),
            deleted_at: None,
        };
        self.holds_marker(&marker.id(), held);
        Ok(())
    }

    fn apply(&self, at: &At<'_>, files: &[Vec<u8>]) -> Answer<()> {
        let ids: Vec<String> = files
            .iter()
            .flat_map(|file| documents(file))
            .map(|document| document.id)
            .collect();
        self.ask(Asked::Apply(ids.clone()), Some(at))?;
        self.state.borrow_mut().dashboards.extend(ids);
        Ok(())
    }

    fn delete(&self, at: &At<'_>, id: &str) -> Answer<()> {
        self.ask(Asked::Delete(id.into()), Some(at))?;
        self.state.borrow_mut().dashboards.remove(id);
        Ok(())
    }
}
