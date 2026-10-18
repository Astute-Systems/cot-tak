//! In-memory missions and files behind the Marti API.
//!
//! This module is plain data and rules; HTTP and JSON live in [`crate::marti`].
//! It is always accessed through the hub's lock.

use cottak_core::Event;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Why an operation was refused. The HTTP layer maps these to status codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Failure {
    NotFound,
    Conflict,
    Forbidden,
    Invalid(&'static str),
    Full,
}

pub(crate) type Outcome<T> = Result<T, Failure>;

/// Role names TAK Server uses.
pub(crate) const ROLE_OWNER: &str = "MISSION_OWNER";
pub(crate) const ROLE_SUBSCRIBER: &str = "MISSION_SUBSCRIBER";
pub(crate) const ROLE_READ_ONLY: &str = "MISSION_READONLY_SUBSCRIBER";

#[derive(Debug, Clone, Copy)]
pub(crate) struct Limits {
    pub(crate) missions: usize,
    pub(crate) files: usize,
    pub(crate) bytes: usize,
    pub(crate) uids_per_mission: usize,
}

pub(crate) struct UidEntry {
    pub(crate) uid: String,
    pub(crate) ts: i64,
    pub(crate) creator: String,
}

pub(crate) struct ContentEntry {
    pub(crate) hash: String,
    pub(crate) ts: i64,
    pub(crate) creator: String,
}

pub(crate) struct Change {
    pub(crate) kind: &'static str,
    pub(crate) hash: Option<String>,
    pub(crate) uid: Option<String>,
    pub(crate) ts: i64,
    pub(crate) creator: String,
}

pub(crate) struct Mission {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) chat_room: Option<String>,
    pub(crate) base_layer: Option<String>,
    pub(crate) bbox: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) classification: Option<String>,
    pub(crate) tool: String,
    pub(crate) creator_uid: String,
    pub(crate) created: i64,
    pub(crate) last_edited: i64,
    password: Option<String>,
    pub(crate) invite_only: bool,
    pub(crate) expiration: i64,
    pub(crate) guid: String,
    pub(crate) default_role: &'static str,
    pub(crate) uids: Vec<UidEntry>,
    pub(crate) contents: Vec<ContentEntry>,
    /// Subscribed client UID -> time subscribed.
    pub(crate) subscribers: BTreeMap<String, i64>,
    pub(crate) changes: Vec<Change>,
    /// Latest event known for each UID in the mission.
    pub(crate) latest: HashMap<String, Arc<Event>>,
}

impl Mission {
    pub(crate) fn password_protected(&self) -> bool {
        self.password.is_some()
    }

    /// `true` if no password is set or `given` matches it.
    pub(crate) fn password_ok(&self, given: Option<&str>) -> bool {
        match (&self.password, given) {
            (None, _) => true,
            (Some(want), Some(got)) => constant_time_eq(want.as_bytes(), got.as_bytes()),
            (Some(_), None) => false,
        }
    }

    fn touch(&mut self, now: i64) {
        self.last_edited = now;
    }
}

pub(crate) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let mut diff = a.len() ^ b.len();
    for i in 0..a.len().max(b.len()) {
        diff |= usize::from(a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0));
    }
    diff == 0
}

pub(crate) struct FileRecord {
    pub(crate) id: i64,
    pub(crate) hash: String,
    pub(crate) name: String,
    pub(crate) mime: String,
    pub(crate) uid: String,
    pub(crate) submitter: Option<String>,
    pub(crate) creator_uid: Option<String>,
    pub(crate) ts: i64,
    pub(crate) keywords: Vec<String>,
    pub(crate) tool: String,
    pub(crate) data: Arc<Vec<u8>>,
}

pub(crate) struct NewMission {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) chat_room: Option<String>,
    pub(crate) base_layer: Option<String>,
    pub(crate) bbox: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) classification: Option<String>,
    pub(crate) tool: Option<String>,
    pub(crate) creator_uid: String,
    pub(crate) password: Option<String>,
    pub(crate) default_role: Option<String>,
    pub(crate) invite_only: bool,
    pub(crate) expiration: i64,
}

/// Content to add to a mission.
#[derive(Clone, Copy)]
pub(crate) struct Addition<'a> {
    pub(crate) hashes: &'a [String],
    pub(crate) uids: &'a [String],
    pub(crate) creator: &'a str,
}

pub(crate) enum ContentRef<'a> {
    Hash(&'a str),
    Uid(&'a str),
}

#[derive(Default)]
pub(crate) struct Registry {
    pub(crate) missions: BTreeMap<String, Mission>,
    pub(crate) files: HashMap<String, FileRecord>,
    file_bytes: usize,
    next_file_id: i64,
}

impl Registry {
    pub(crate) fn mission(&self, name: &str) -> Outcome<&Mission> {
        self.missions.get(name).ok_or(Failure::NotFound)
    }

    fn mission_mut(&mut self, name: &str) -> Outcome<&mut Mission> {
        self.missions.get_mut(name).ok_or(Failure::NotFound)
    }

    pub(crate) fn create(&mut self, p: NewMission, now: i64, limits: Limits) -> Outcome<&Mission> {
        if self.missions.contains_key(&p.name) {
            return Err(Failure::Conflict);
        }
        if self.missions.len() >= limits.missions {
            return Err(Failure::Full);
        }
        let default_role = match p.default_role.as_deref() {
            None | Some(ROLE_SUBSCRIBER) => ROLE_SUBSCRIBER,
            Some(ROLE_READ_ONLY) => ROLE_READ_ONLY,
            Some(ROLE_OWNER) => ROLE_OWNER,
            Some(_) => return Err(Failure::Invalid("unknown defaultRole")),
        };
        let mission = Mission {
            name: p.name.clone(),
            description: p.description,
            chat_room: p.chat_room,
            base_layer: p.base_layer,
            bbox: p.bbox,
            path: p.path,
            classification: p.classification,
            tool: p.tool.unwrap_or_else(|| "public".to_owned()),
            creator_uid: p.creator_uid.clone(),
            created: now,
            last_edited: now,
            password: p.password.filter(|s| !s.is_empty()),
            invite_only: p.invite_only,
            expiration: p.expiration,
            guid: Event::random_uid(),
            default_role,
            uids: Vec::new(),
            contents: Vec::new(),
            subscribers: BTreeMap::new(),
            changes: vec![Change {
                kind: "CREATE_MISSION",
                hash: None,
                uid: None,
                ts: now,
                creator: p.creator_uid,
            }],
            latest: HashMap::new(),
        };
        Ok(self.missions.entry(p.name).or_insert(mission))
    }

    /// Deletes a mission; with `deep`, also the files only it referenced.
    pub(crate) fn delete(&mut self, name: &str, deep: bool) -> Outcome<()> {
        let mission = self.missions.remove(name).ok_or(Failure::NotFound)?;
        if deep {
            for c in mission.contents {
                let shared = self
                    .missions
                    .values()
                    .any(|m| m.contents.iter().any(|x| x.hash == c.hash));
                if !shared {
                    self.remove_file(&c.hash);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn add_content(
        &mut self,
        name: &str,
        add: &Addition<'_>,
        now: i64,
        limits: Limits,
        lookup: impl Fn(&str) -> Option<Arc<Event>>,
    ) -> Outcome<&Mission> {
        let Addition {
            hashes,
            uids,
            creator,
        } = *add;
        if hashes.iter().any(|h| !self.files.contains_key(h.as_str())) {
            return Err(Failure::NotFound);
        }
        let mission = self.missions.get_mut(name).ok_or(Failure::NotFound)?;
        for hash in hashes {
            if mission.contents.iter().any(|c| &c.hash == hash) {
                continue;
            }
            mission.contents.push(ContentEntry {
                hash: hash.clone(),
                ts: now,
                creator: creator.to_owned(),
            });
            mission.changes.push(Change {
                kind: "ADD_CONTENT",
                hash: Some(hash.clone()),
                uid: None,
                ts: now,
                creator: creator.to_owned(),
            });
        }
        for uid in uids {
            if mission.uids.iter().any(|u| &u.uid == uid) {
                continue;
            }
            if mission.uids.len() >= limits.uids_per_mission {
                return Err(Failure::Full);
            }
            add_uid(mission, uid, creator, now);
            if let Some(ev) = lookup(uid) {
                mission.latest.insert(uid.clone(), ev);
            }
        }
        mission.touch(now);
        Ok(&*mission)
    }

    pub(crate) fn remove_content(
        &mut self,
        name: &str,
        item: &ContentRef<'_>,
        creator: &str,
        now: i64,
    ) -> Outcome<&Mission> {
        let mission = self.mission_mut(name)?;
        let change = match item {
            ContentRef::Hash(h) => {
                let before = mission.contents.len();
                mission.contents.retain(|c| c.hash != *h);
                (mission.contents.len() != before).then(|| Change {
                    kind: "REMOVE_CONTENT",
                    hash: Some((*h).to_owned()),
                    uid: None,
                    ts: now,
                    creator: creator.to_owned(),
                })
            }
            ContentRef::Uid(u) => {
                let before = mission.uids.len();
                mission.uids.retain(|e| e.uid != *u);
                mission.latest.remove(*u);
                (mission.uids.len() != before).then(|| Change {
                    kind: "REMOVE_CONTENT",
                    hash: None,
                    uid: Some((*u).to_owned()),
                    ts: now,
                    creator: creator.to_owned(),
                })
            }
        };
        if let Some(c) = change {
            mission.changes.push(c);
            mission.touch(now);
        }
        Ok(&*mission)
    }

    pub(crate) fn subscribe(
        &mut self,
        name: &str,
        uid: &str,
        password: Option<&str>,
        now: i64,
    ) -> Outcome<&Mission> {
        let mission = self.mission_mut(name)?;
        if !mission.password_ok(password) {
            return Err(Failure::Forbidden);
        }
        mission.subscribers.insert(uid.to_owned(), now);
        Ok(&*mission)
    }

    pub(crate) fn unsubscribe(&mut self, name: &str, uid: &str) -> Outcome<()> {
        self.mission_mut(name)?.subscribers.remove(uid);
        Ok(())
    }

    /// Keeps the mission copies of an entity current when it moves.
    pub(crate) fn observe_event(&mut self, event: &Arc<Event>) {
        for m in self.missions.values_mut() {
            if m.latest.contains_key(&event.uid) || m.uids.iter().any(|u| u.uid == event.uid) {
                m.latest.insert(event.uid.clone(), Arc::clone(event));
            }
        }
    }

    /// Records an event addressed to a mission and returns the subscribers to
    /// deliver it to, or `None` if no such mission exists.
    pub(crate) fn deliver(
        &mut self,
        name: &str,
        event: &Arc<Event>,
        sender_uid: &str,
        now: i64,
        limits: Limits,
    ) -> Option<Vec<String>> {
        let m = self.missions.get_mut(name)?;
        if !m.uids.iter().any(|u| u.uid == event.uid) && m.uids.len() < limits.uids_per_mission {
            add_uid(m, &event.uid, sender_uid, now);
            m.touch(now);
        }
        if m.uids.iter().any(|u| u.uid == event.uid) {
            m.latest.insert(event.uid.clone(), Arc::clone(event));
        }
        Some(m.subscribers.keys().cloned().collect())
    }

    // ----- files -----------------------------------------------------------------------------

    /// Stores a file. Identical content (same hash) is stored once.
    pub(crate) fn add_file(
        &mut self,
        mut record: FileRecord,
        limits: Limits,
    ) -> Outcome<&FileRecord> {
        if self.files.contains_key(&record.hash) {
            return self.files.get(&record.hash).ok_or(Failure::NotFound);
        }
        if self.files.len() >= limits.files
            || self.file_bytes.saturating_add(record.data.len()) > limits.bytes
        {
            return Err(Failure::Full);
        }
        self.next_file_id += 1;
        record.id = self.next_file_id;
        self.file_bytes += record.data.len();
        let hash = record.hash.clone();
        Ok(self.files.entry(hash).or_insert(record))
    }

    fn remove_file(&mut self, hash: &str) {
        if let Some(f) = self.files.remove(hash) {
            self.file_bytes -= f.data.len();
        }
    }
}

fn add_uid(mission: &mut Mission, uid: &str, creator: &str, now: i64) {
    mission.uids.push(UidEntry {
        uid: uid.to_owned(),
        ts: now,
        creator: creator.to_owned(),
    });
    mission.changes.push(Change {
        kind: "ADD_CONTENT",
        hash: None,
        uid: Some(uid.to_owned()),
        ts: now,
        creator: creator.to_owned(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMITS: Limits = Limits {
        missions: 2,
        files: 2,
        bytes: 10,
        uids_per_mission: 2,
    };

    fn new(name: &str) -> NewMission {
        NewMission {
            name: name.into(),
            description: None,
            chat_room: None,
            base_layer: None,
            bbox: None,
            path: None,
            classification: None,
            tool: None,
            creator_uid: "C".into(),
            password: None,
            default_role: None,
            invite_only: false,
            expiration: -1,
        }
    }

    fn file(hash: &str, len: usize) -> FileRecord {
        FileRecord {
            id: 0,
            hash: hash.into(),
            name: "f".into(),
            mime: "text/plain".into(),
            uid: "u".into(),
            submitter: None,
            creator_uid: None,
            ts: 0,
            keywords: vec![],
            tool: "public".into(),
            data: Arc::new(vec![0; len]),
        }
    }

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    /// Adds content as creator `C` with no event lookup; returns the failure, if any.
    fn add(
        r: &mut Registry,
        mission: &str,
        hashes: &[&str],
        uids: &[&str],
        now: i64,
    ) -> Option<Failure> {
        let (hashes, uids) = (strings(hashes), strings(uids));
        let add = Addition {
            hashes: &hashes,
            uids: &uids,
            creator: "C",
        };
        r.add_content(mission, &add, now, LIMITS, |_| None).err()
    }

    #[test]
    fn missions_are_unique_and_bounded() {
        let mut r = Registry::default();
        r.create(new("a"), 1, LIMITS).unwrap();
        assert_eq!(r.create(new("a"), 1, LIMITS).err(), Some(Failure::Conflict));
        r.create(new("b"), 1, LIMITS).unwrap();
        assert_eq!(r.create(new("c"), 1, LIMITS).err(), Some(Failure::Full));
        let mut bad = new("d");
        bad.default_role = Some("ROOT".into());
        r.delete("a", false).unwrap();
        assert!(matches!(
            r.create(bad, 1, LIMITS).err(),
            Some(Failure::Invalid(_))
        ));
    }

    #[test]
    fn content_needs_existing_files_and_is_logged() {
        let mut r = Registry::default();
        r.create(new("m"), 1, LIMITS).unwrap();
        assert_eq!(add(&mut r, "m", &["nope"], &[], 2), Some(Failure::NotFound));
        r.add_file(file("h1", 3), LIMITS).unwrap();
        assert_eq!(add(&mut r, "m", &["h1", "h1"], &["U1"], 3), None);
        let m = r.mission("m").unwrap();
        assert_eq!(
            (m.contents.len(), m.uids.len()),
            (1, 1),
            "duplicates are ignored"
        );
        assert_eq!(
            m.changes.iter().map(|c| c.kind).collect::<Vec<_>>(),
            ["CREATE_MISSION", "ADD_CONTENT", "ADD_CONTENT"]
        );
        let m = r
            .remove_content("m", &ContentRef::Hash("h1"), "C", 4)
            .unwrap();
        assert!(m.contents.is_empty());
        assert_eq!(m.changes.last().unwrap().kind, "REMOVE_CONTENT");
        // Removing something absent is a no-op, not an error.
        let n = r.mission("m").unwrap().changes.len();
        let m = r
            .remove_content("m", &ContentRef::Uid("zzz"), "C", 5)
            .unwrap();
        assert_eq!(m.changes.len(), n);
        assert_eq!(add(&mut r, "m", &[], &["U2", "U3"], 6), Some(Failure::Full));
    }

    #[test]
    fn file_store_is_bounded_and_deduplicated() {
        let mut r = Registry::default();
        let id = r.add_file(file("h1", 6), LIMITS).unwrap().id;
        assert_eq!(
            r.add_file(file("h1", 6), LIMITS).unwrap().id,
            id,
            "same hash is stored once"
        );
        assert_eq!(
            r.add_file(file("h2", 6), LIMITS).err(),
            Some(Failure::Full),
            "byte budget"
        );
        r.add_file(file("h3", 4), LIMITS).unwrap();
        assert_eq!(
            r.add_file(file("h4", 0), LIMITS).err(),
            Some(Failure::Full),
            "file count"
        );
    }

    #[test]
    fn deep_delete_keeps_shared_files() {
        let mut r = Registry::default();
        r.add_file(file("shared", 1), LIMITS).unwrap();
        r.add_file(file("mine", 1), LIMITS).unwrap();
        r.create(new("a"), 1, LIMITS).unwrap();
        r.create(new("b"), 1, LIMITS).unwrap();
        assert_eq!(add(&mut r, "a", &["shared", "mine"], &[], 2), None);
        assert_eq!(add(&mut r, "b", &["shared"], &[], 2), None);
        r.delete("a", true).unwrap();
        assert!(r.files.contains_key("shared") && !r.files.contains_key("mine"));
    }

    #[test]
    fn passwords_gate_subscription() {
        let mut r = Registry::default();
        let mut p = new("secret");
        p.password = Some("pw".into());
        r.create(p, 1, LIMITS).unwrap();
        assert_eq!(
            r.subscribe("secret", "U", None, 2).err(),
            Some(Failure::Forbidden)
        );
        assert_eq!(
            r.subscribe("secret", "U", Some("nope"), 2).err(),
            Some(Failure::Forbidden)
        );
        r.subscribe("secret", "U", Some("pw"), 2).unwrap();
        assert_eq!(r.mission("secret").unwrap().subscribers.len(), 1);
        r.unsubscribe("secret", "U").unwrap();
        assert!(r.mission("secret").unwrap().subscribers.is_empty());
    }

    #[test]
    fn constant_time_compare_handles_lengths() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abcd"));
        assert!(!constant_time_eq(b"", b"a"));
        assert!(constant_time_eq(b"", b""));
    }
}
