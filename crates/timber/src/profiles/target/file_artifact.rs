//! A version's files, read here for every target profile (the Dash0
//! target's design D1). An output a component declares is packed by the
//! file projection as one artifact of one layer, its file's bytes; a
//! target profile that applies such files reads the verified version's
//! index with [`index`], finds the artifact of an output in it with
//! [`file_of`] or [`artifact_of`], and reads its bytes with [`bytes`],
//! each through the profile's own store binding by digest (42, rules
//! core.4, core.5).
//!
//! Each answer names the artifact in the asking profile's own [`Words`]:
//! a closure file to the Nix profiles, a dashboard file to the Dash0
//! target. The profiles share this module so that none calls another
//! (rule core.2), and it names no tool and no service.

use std::fmt;

use timber_seams::{
    Answer, Artifact, Digest, Held, IndexMeta, Manifest, Not, OutputName, Source, Store, Verified,
    Why,
};

/// The ecosystem the file projection packs in, as an index names a file's
/// artifact; read from the index, so no profile asks that projection
/// anything (rule core.2).
pub const FILE: &str = "file";

/// What an asking profile calls one of its file artifacts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Words {
    /// One such file: `closure file`, `dashboard file`.
    pub file: &'static str,
    /// What the file carries, as a version lacking it is said to carry
    /// none of it: `closure`, `dashboard`.
    pub carries: &'static str,
}

/// A file artifact as an answer names it, in its profile's words:
/// `the closure file swancloud-srv@0.4.0 (sha256:…)`.
pub struct Named<'a>(pub &'a Artifact, pub Words);

impl fmt::Display for Named<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Named(artifact, words) = self;
        write!(
            f,
            "the {} {}@{} ({})",
            words.file,
            artifact.meta.name(),
            artifact.meta.version(),
            artifact.digest
        )
    }
}

/// The index `version` names, read through `store` by its digest,
/// naming the version verified: unreadable where the store holds none,
/// and refused where the digest holds no index or an index of another
/// version (42, rule core.4).
pub fn index(store: &dyn Store, version: &Verified, source: &Source) -> Answer<IndexMeta> {
    let digest = version.index();
    let meta = match store.read(digest) {
        Ok(Held::Manifest(Manifest::Index(meta))) => meta,
        Ok(_) => {
            return Err(refused(
                source,
                format!(
                    "{digest}, the index of version {} as verified, holds no index",
                    version.version()
                ),
            ))
        }
        Err(Not::Absent(_)) => {
            return Err(Not::unreadable(
                source,
                format!("the index of version {}", version.version()),
                Some(format!("the store holds no {digest}")),
            ))
        }
        Err(not) => return Err(not),
    };
    if meta.version() != version.version() {
        return Err(refused(
            source,
            format!(
                "the index {digest} names version {}, and the version verified is {}",
                meta.version(),
                version.version()
            ),
        ));
    }
    Ok(meta)
}

/// The file projection's artifact of `output` among those `index`
/// names, where it names one.
pub fn file_of<'a>(index: &'a IndexMeta, output: &OutputName) -> Option<&'a Artifact> {
    index
        .artifacts()
        .iter()
        .find(|a| a.meta.ecosystem().as_str() == FILE && a.meta.name().as_str() == output.as_str())
}

/// The file projection's artifact of `output` among those `index`
/// names, refused naming the output and the version, and what `words`
/// says the file carries, where it names none.
pub fn artifact_of<'a>(
    index: &'a IndexMeta,
    output: &OutputName,
    words: Words,
    source: &Source,
) -> Answer<&'a Artifact> {
    file_of(index, output).ok_or_else(|| {
        refused(
            source,
            format!(
                "version {} of {} names no {FILE} artifact of the output {output}, \
                 so it carries no {} of it",
                index.version(),
                index.component(),
                words.carries
            ),
        )
    })
}

/// The bytes of the file `artifact`, read through `store` by the one
/// layer its manifest names and checked against that layer's digest
/// (42, rule core.5): unreadable where the store holds the manifest or
/// the layer not, and refused where they are not one file's, each
/// naming the artifact in `words`.
pub fn bytes(
    store: &dyn Store,
    artifact: &Artifact,
    words: Words,
    source: &Source,
) -> Answer<Vec<u8>> {
    let (digest, file) = (&artifact.digest, Named(artifact, words));
    let unheld = |what: &Digest| {
        Not::unreadable(
            source,
            file.to_string(),
            Some(format!("the store holds no {what}")),
        )
    };
    let layers = match store.read(digest) {
        Ok(Held::Manifest(Manifest::Artifact { meta, layers })) if meta == artifact.meta => layers,
        Ok(Held::Manifest(Manifest::Artifact { meta, .. })) => {
            return Err(refused(
                source,
                format!(
                    "the store's manifest at {digest} names {} {} {}, and {file}",
                    meta.ecosystem(),
                    meta.name(),
                    meta.version()
                ),
            ))
        }
        Ok(_) => {
            return Err(refused(
                source,
                format!("{digest} holds no artifact's manifest, and {file}"),
            ))
        }
        Err(Not::Absent(_)) => return Err(unheld(digest)),
        Err(not) => return Err(not),
    };
    let [layer] = layers.as_slice() else {
        return Err(refused(
            source,
            format!(
                "{file} names {} layers, and a {} is one",
                layers.len(),
                words.file
            ),
        ));
    };
    let bytes = match store.read(layer) {
        Ok(Held::Bytes(bytes)) => bytes,
        Ok(Held::Manifest(_)) => {
            return Err(refused(
                source,
                format!("the layer {layer} of {file} holds a manifest and not a file"),
            ))
        }
        Err(Not::Absent(_)) => return Err(unheld(layer)),
        Err(not) => return Err(not),
    };
    let answered = Digest::of(&bytes);
    if &answered != layer {
        return Err(Not::refused(
            source,
            Why::OtherBytes {
                expected: layer.clone(),
                answered,
            },
            None,
        ));
    }
    Ok(bytes)
}

fn refused(source: &Source, said: String) -> Not {
    Not::refused(source, Why::Said, Some(said))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use timber_fakes::FakeStore;
    use timber_seams::{
        ArtifactMeta, ComponentName, Ecosystem, EcosystemName, Envelope, Name, Put, Seam, Version,
    };

    use super::*;

    const WORDS: Words = Words {
        file: "dashboard file",
        carries: "dashboard",
    };

    fn source() -> Source {
        Source::new(Seam::Target, "dash0")
    }

    /// A store that answers a read of a digest it was told to with bytes
    /// of the test's own in place of what it holds, as a store whose
    /// bytes were changed under it would.
    #[derive(Clone, Default)]
    struct Changed {
        store: FakeStore,
        changed: Rc<RefCell<BTreeMap<Digest, Vec<u8>>>>,
    }

    impl Store for Changed {
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
        fn read(&self, digest: &Digest) -> Answer<Held> {
            match self.changed.borrow().get(digest) {
                Some(bytes) => Ok(Held::Bytes(bytes.clone())),
                None => self.store.read(digest),
            }
        }
        fn versions(&self, ecosystem: &Ecosystem, name: &EcosystemName) -> Answer<Vec<Version>> {
            self.store.versions(ecosystem, name)
        }
    }

    fn meta(name: &str, version: &str) -> ArtifactMeta {
        ArtifactMeta::new(
            Ecosystem::new(FILE),
            EcosystemName::new(name),
            Version::new(version),
        )
        .unwrap()
    }

    /// The file artifact of `dashboards` at 1.3.0, its manifest naming
    /// `layers`, put into `store`.
    fn artifact(store: &dyn Store, layers: Vec<Digest>) -> Artifact {
        let meta = meta("dashboards", "1.3.0");
        let manifest = Manifest::Artifact {
            meta: meta.clone(),
            layers,
        };
        let digest = store.put_artifact(&manifest).unwrap().digest;
        Artifact { digest, meta }
    }

    /// An index of `dashboards` at `version` naming `artifacts`, put into
    /// `store`, and its digest.
    fn put_index(store: &dyn Store, version: &str, artifacts: Vec<Artifact>) -> Digest {
        let index = IndexMeta::new(
            ComponentName::new("dashboards"),
            Version::new(version),
            artifacts,
        )
        .unwrap();
        store.put_artifact(&Manifest::Index(index)).unwrap().digest
    }

    fn refused_said<T: fmt::Debug>(answer: Answer<T>) -> String {
        match answer {
            Err(Not::Refused(r)) => {
                assert_eq!(r.source, source());
                assert_eq!(r.why, Why::Said);
                r.said.expect("the profile's words")
            }
            other => panic!("answered {other:?}, not refused"),
        }
    }

    #[test]
    fn a_files_bytes_are_read_by_its_one_layer() {
        let store = FakeStore::default();
        let layer = store.put_bytes(b"kind: Dashboard\n").unwrap().digest;
        let file = artifact(&store, vec![layer]);
        assert_eq!(
            bytes(&store, &file, WORDS, &source()),
            Ok(b"kind: Dashboard\n".to_vec())
        );
    }

    #[test]
    fn an_index_of_another_version_is_refused_naming_both() {
        let store = FakeStore::default();
        let digest = put_index(&store, "1.3.0", Vec::new());
        let verified = Verified::new(digest.clone(), Version::new("1.4.0"));
        assert_eq!(
            refused_said(index(&store, &verified, &source())),
            format!("the index {digest} names version 1.3.0, and the version verified is 1.4.0")
        );
        let same = Verified::new(digest, Version::new("1.3.0"));
        assert_eq!(
            index(&store, &same, &source()).map(|m| m.version().clone()),
            Ok(Version::new("1.3.0"))
        );
    }

    #[test]
    fn a_manifest_of_two_layers_is_refused_naming_the_artifact_in_its_words() {
        let store = FakeStore::default();
        let one = store.put_bytes(b"one").unwrap().digest;
        let two = store.put_bytes(b"two").unwrap().digest;
        let file = artifact(&store, vec![one, two]);
        assert_eq!(
            refused_said(bytes(&store, &file, WORDS, &source())),
            format!(
                "the dashboard file dashboards@1.3.0 ({}) names 2 layers, and a dashboard \
                 file is one",
                file.digest
            )
        );
    }

    #[test]
    fn a_layer_whose_bytes_are_not_its_digest_is_refused_naming_both_digests() {
        let store = Changed::default();
        let layer = store.put_bytes(b"kind: Dashboard\n").unwrap().digest;
        let file = artifact(&store, vec![layer.clone()]);
        store
            .changed
            .borrow_mut()
            .insert(layer.clone(), b"kind: CheckRule\n".to_vec());
        match bytes(&store, &file, WORDS, &source()) {
            Err(Not::Refused(r)) => {
                assert_eq!(r.source, source());
                assert_eq!(
                    r.why,
                    Why::OtherBytes {
                        expected: layer,
                        answered: Digest::of(b"kind: CheckRule\n"),
                    }
                );
            }
            other => panic!("changed bytes answered {other:?}, not refused"),
        }
    }

    #[test]
    fn a_digest_the_store_lacks_is_unreadable_naming_the_artifact_in_its_words() {
        let store = FakeStore::default();
        let layer = Digest::of(b"never put");
        let file = artifact(&store, vec![layer.clone()]);
        match bytes(&store, &file, WORDS, &source()) {
            Err(Not::Unreadable(u)) => {
                assert_eq!(u.source, source());
                assert_eq!(
                    u.what,
                    format!("the dashboard file dashboards@1.3.0 ({})", file.digest)
                );
                assert_eq!(u.said, Some(format!("the store holds no {layer}")));
            }
            other => panic!("a missing layer answered {other:?}, not unreadable"),
        }
        let gone = Verified::new(Digest::of(b"no index"), Version::new("1.3.0"));
        match index(&store, &gone, &source()) {
            Err(Not::Unreadable(u)) => {
                assert_eq!(u.what, "the index of version 1.3.0");
                assert_eq!(
                    u.said,
                    Some(format!("the store holds no {}", Digest::of(b"no index")))
                );
            }
            other => panic!("a missing index answered {other:?}, not unreadable"),
        }
    }

    #[test]
    fn an_output_the_index_lacks_is_refused_naming_what_its_file_carries() {
        let store = FakeStore::default();
        let digest = put_index(&store, "1.3.0", Vec::new());
        let meta = index(
            &store,
            &Verified::new(digest, Version::new("1.3.0")),
            &source(),
        )
        .unwrap();
        assert_eq!(
            refused_said(artifact_of(
                &meta,
                &OutputName::new("dashboards"),
                WORDS,
                &source()
            )),
            "version 1.3.0 of dashboards names no file artifact of the output dashboards, so it \
             carries no dashboard of it"
        );
    }
}
