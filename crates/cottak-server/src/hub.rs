//! Shared server state: connected clients, the latest-state store and routing.

use crate::registry::{Limits, Registry};
use cottak_core::detail::{Contact, Group, Link, Marti};
use cottak_core::messages::control;
use cottak_core::Event;
use cottak_net::Protocol;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, warn};

/// What the server knows about a connected client.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ClientInfo {
    /// Identifier of this connection, unique for the life of the server.
    pub id: u64,
    /// Remote socket address.
    pub peer: SocketAddr,
    /// `true` if the connection arrived on a TLS listener.
    pub tls: bool,
    /// Wire protocol currently in use.
    pub protocol: Protocol,
    /// UID of the client, learned from its position reports.
    pub uid: Option<String>,
    /// Callsign of the client, learned from its position reports.
    pub callsign: Option<String>,
    /// Team colour from the client's `__group`, if it sent one.
    pub team: Option<String>,
    /// Team role from the client's `__group`, if it sent one.
    pub role: Option<String>,
    /// Time of the last event received from the client.
    pub last_event: Option<std::time::SystemTime>,
}

/// Something that happened on the server; see [`Server::subscribe`](crate::Server::subscribe).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ServerEvent {
    /// A client connected.
    Connected(ClientInfo),
    /// A client disconnected.
    Disconnected(ClientInfo),
    /// A client upgraded its connection to a new protocol.
    ProtocolChanged {
        /// Connection that changed.
        id: u64,
        /// The protocol now in use.
        protocol: Protocol,
    },
    /// An event was received from a client (or injected by the application)
    /// and routed.
    Event {
        /// Connection it came from, or `None` if the application sent it.
        from: Option<u64>,
        /// The event.
        event: Arc<Event>,
    },
}

pub(crate) struct Entry {
    pub(crate) info: ClientInfo,
    tx: mpsc::Sender<Arc<Event>>,
}

#[derive(Default)]
pub(crate) struct State {
    next_id: u64,
    pub(crate) clients: HashMap<u64, Entry>,
    pub(crate) store: HashMap<String, Arc<Event>>,
    pub(crate) registry: Registry,
}

/// A newly registered connection.
pub(crate) struct Registration {
    pub(crate) id: u64,
    pub(crate) rx: mpsc::Receiver<Arc<Event>>,
    /// Latest known state of every entity, to send before live traffic.
    pub(crate) replay: Vec<Arc<Event>>,
}

pub(crate) struct Hub {
    state: Mutex<State>,
    observers: broadcast::Sender<ServerEvent>,
    client_queue: usize,
    max_clients: usize,
    max_stored: usize,
    replay: bool,
    limits: Limits,
}

impl Hub {
    pub(crate) fn new(
        client_queue: usize,
        max_clients: usize,
        max_stored: usize,
        replay: bool,
        observer_queue: usize,
        limits: Limits,
    ) -> Self {
        Self {
            state: Mutex::new(State::default()),
            observers: broadcast::channel(observer_queue.max(1)).0,
            client_queue: client_queue.max(1),
            max_clients,
            max_stored,
            replay,
            limits,
        }
    }

    pub(crate) fn limits(&self) -> Limits {
        self.limits
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, State> {
        // A poisoned lock only means another session panicked; the data is
        // still consistent because every critical section is short and
        // leaves the maps valid at each step.
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn subscribe(&self) -> broadcast::Receiver<ServerEvent> {
        self.observers.subscribe()
    }

    /// Adds a connection, or returns `None` if the server is full.
    ///
    /// The replay snapshot is taken under the same lock as the registration,
    /// so no event is missed or delivered twice.
    pub(crate) fn register(&self, peer: SocketAddr, tls: bool) -> Option<Registration> {
        let mut st = self.lock();
        if st.clients.len() >= self.max_clients {
            return None;
        }
        st.next_id += 1;
        let id = st.next_id;
        let (tx, rx) = mpsc::channel(self.client_queue);
        let info = ClientInfo {
            id,
            peer,
            tls,
            protocol: Protocol::Xml,
            uid: None,
            callsign: None,
            team: None,
            role: None,
            last_event: None,
        };
        let replay = if self.replay {
            let mut all: Vec<_> = st
                .store
                .values()
                .filter(|e| !e.is_stale())
                .cloned()
                .collect();
            all.sort_by(|a, b| a.time.cmp(&b.time).then_with(|| a.uid.cmp(&b.uid)));
            all
        } else {
            Vec::new()
        };
        st.clients.insert(
            id,
            Entry {
                info: info.clone(),
                tx,
            },
        );
        drop(st);
        let _ = self.observers.send(ServerEvent::Connected(info));
        Some(Registration { id, rx, replay })
    }

    pub(crate) fn unregister(&self, id: u64) {
        let removed = self.lock().clients.remove(&id);
        if let Some(entry) = removed {
            let _ = self.observers.send(ServerEvent::Disconnected(entry.info));
        }
    }

    pub(crate) fn set_protocol(&self, id: u64, protocol: Protocol) {
        if let Some(entry) = self.lock().clients.get_mut(&id) {
            entry.info.protocol = protocol;
        }
        let _ = self
            .observers
            .send(ServerEvent::ProtocolChanged { id, protocol });
    }

    pub(crate) fn clients(&self) -> Vec<ClientInfo> {
        let mut v: Vec<_> = self
            .lock()
            .clients
            .values()
            .map(|e| e.info.clone())
            .collect();
        v.sort_by_key(|c| c.id);
        v
    }

    pub(crate) fn stored(&self) -> Vec<Arc<Event>> {
        let mut v: Vec<_> = self
            .lock()
            .store
            .values()
            .filter(|e| !e.is_stale())
            .cloned()
            .collect();
        v.sort_by(|a, b| a.uid.cmp(&b.uid));
        v
    }

    /// Stores and routes `event`. `from` is the sending connection, if any.
    pub(crate) fn ingest(&self, from: Option<u64>, event: Event) {
        let event = Arc::new(event);
        let now = now_ms();
        let mut st = self.lock();

        if let Some(id) = from {
            learn_identity(&mut st, id, &event);
        }
        self.update_store(&mut st, &event);
        st.registry.observe_event(&event);

        // Events addressed to missions are recorded and go to the subscribers.
        let sender_uid = from
            .and_then(|id| st.clients.get(&id))
            .and_then(|e| e.info.uid.clone())
            .unwrap_or_else(|| "server".to_owned());
        let mut mission_uids: HashSet<String> = HashSet::new();
        if let Some(marti) = event.detail.get::<Marti>() {
            for name in marti.dests.iter().filter_map(|d| d.mission.as_deref()) {
                if let Some(subscribers) =
                    st.registry
                        .deliver(name, &event, &sender_uid, now, self.limits)
                {
                    mission_uids.extend(subscribers);
                }
            }
        }

        for entry in recipients(&st, from, &event, &mission_uids) {
            match entry.tx.try_send(Arc::clone(&event)) {
                Ok(()) => {}
                Err(mpsc::error::TrySendError::Full(_)) => {
                    warn!(client = entry.info.id, uid = %event.uid, "client queue full; dropping event");
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    debug!(client = entry.info.id, "client is gone; dropping event");
                }
            }
        }
        drop(st);
        let _ = self.observers.send(ServerEvent::Event { from, event });
    }

    fn update_store(&self, st: &mut State, event: &Arc<Event>) {
        let ty = event.cot_type.as_str();
        if ty == control::DELETE_TYPE {
            for link in event.detail.get_all::<Link>() {
                if let Some(uid) = link.uid {
                    st.store.remove(&uid);
                }
            }
            return;
        }
        // Only situational-awareness atoms are worth replaying.
        if !ty.starts_with("a-") || event.is_stale() {
            return;
        }
        if !st.store.contains_key(&event.uid) && st.store.len() >= self.max_stored {
            st.store.retain(|_, e| !e.is_stale());
            if st.store.len() >= self.max_stored {
                debug!(uid = %event.uid, "store full; not keeping event for replay");
                return;
            }
        }
        st.store.insert(event.uid.clone(), Arc::clone(event));
    }
}

/// A client's own position report tells us who it is. There is no
/// authentication here: the last claim to a callsign wins.
fn learn_identity(st: &mut State, id: u64, event: &Event) {
    let Some(entry) = st.clients.get_mut(&id) else {
        return;
    };
    entry.info.last_event = Some(std::time::SystemTime::now());
    if !event.cot_type.as_str().starts_with("a-") {
        return;
    }
    let Some(callsign) = event.detail.get::<Contact>().and_then(|c| c.callsign) else {
        return;
    };
    // The first position report names the client. Later reports about other
    // entities (a client may forward events) must not change who it is.
    if entry.info.uid.as_ref().is_some_and(|u| *u != event.uid) {
        return;
    }
    entry.info.uid = Some(event.uid.clone());
    entry.info.callsign = Some(callsign);
    if let Some(group) = event.detail.get::<Group>() {
        entry.info.team = group.name;
        entry.info.role = group.role;
    }
}

/// Chooses who receives `event`: everyone but the sender, or only the
/// callsigns named in `<marti><dest callsign=.../></marti>` plus the
/// subscribers of the missions named in `<dest mission=.../>`.
fn recipients<'a>(
    st: &'a State,
    from: Option<u64>,
    event: &Event,
    mission_uids: &HashSet<String>,
) -> Vec<&'a Entry> {
    let dests = event
        .detail
        .get::<Marti>()
        .map(|m| m.dests)
        .unwrap_or_default();
    if dests.is_empty() {
        return st
            .clients
            .values()
            .filter(|e| Some(e.info.id) != from)
            .collect();
    }
    let callsigns: HashSet<&str> = dests.iter().filter_map(|d| d.callsign.as_deref()).collect();
    st.clients
        .values()
        .filter(|e| Some(e.info.id) != from)
        .filter(|e| {
            e.info
                .callsign
                .as_deref()
                .is_some_and(|c| callsigns.contains(c))
                || e.info
                    .uid
                    .as_deref()
                    .is_some_and(|u| mission_uids.contains(u))
        })
        .collect()
}

/// Milliseconds since the Unix epoch.
pub(crate) fn now_ms() -> i64 {
    i64::try_from(cottak_core::timestamp::now().unix_timestamp_nanos() / 1_000_000).unwrap_or(0)
}
