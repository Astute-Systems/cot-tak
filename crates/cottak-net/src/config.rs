//! Client configuration.

use crate::tls::TlsConfig;
use std::time::Duration;

/// Which wire protocol the client should try to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProtocolPreference {
    /// Stay on legacy XML for the whole connection.
    XmlOnly,
    /// Upgrade to TAK Protocol v1 (protobuf) if the server offers it.
    #[default]
    PreferProtobuf,
}

/// Exponential backoff with jitter for reconnection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Backoff {
    /// Delay after the first failure.
    pub min: Duration,
    /// Upper bound on the delay.
    pub max: Duration,
}

impl Default for Backoff {
    fn default() -> Self {
        Self {
            min: Duration::from_secs(1),
            max: Duration::from_secs(60),
        }
    }
}

impl Backoff {
    /// Delay before retry number `attempt` (0-based): `min * 2^attempt`, capped
    /// at `max`, then jittered uniformly into the upper half of that value.
    pub fn delay(&self, attempt: u32) -> Duration {
        let exp = self
            .min
            .saturating_mul(1u32.checked_shl(attempt.min(30)).unwrap_or(u32::MAX));
        let capped = exp.min(self.max);
        let half = capped / 2;
        half + half.mul_f64(fastrand::f64())
    }
}

/// Settings for [`TakClient`](crate::TakClient).
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// `host:port` of the TAK Server (8087 for TCP, 8089 for TLS by convention).
    pub addr: String,
    /// TLS settings; `None` connects over plain TCP.
    pub tls: Option<TlsConfig>,
    /// UID of this client, used for keepalive pings.
    pub uid: String,
    /// Wire protocol preference.
    pub protocol: ProtocolPreference,
    /// Limit on establishing the TCP (and TLS) connection.
    pub connect_timeout: Duration,
    /// Interval between keepalive pings.
    pub ping_interval: Duration,
    /// The connection is dropped if nothing is received for this long.
    pub read_timeout: Duration,
    /// Limit on a single write.
    pub write_timeout: Duration,
    /// How long to wait for the server's answer to a protocol upgrade request.
    pub negotiate_timeout: Duration,
    /// Reconnection delays.
    pub backoff: Backoff,
    /// Capacity of the outgoing queue.
    pub send_queue: usize,
    /// Capacity of the incoming queue.
    pub recv_queue: usize,
    /// Largest accepted message in bytes.
    pub max_message_len: usize,
    /// Drop outgoing events that are already stale instead of sending them.
    pub drop_stale: bool,
}

impl ClientConfig {
    /// Defaults suitable for a TAK Server: 30 s ping, 90 s read timeout.
    pub fn new(addr: impl Into<String>, uid: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
            tls: None,
            uid: uid.into(),
            protocol: ProtocolPreference::default(),
            connect_timeout: Duration::from_secs(10),
            ping_interval: Duration::from_secs(30),
            read_timeout: Duration::from_secs(90),
            write_timeout: Duration::from_secs(10),
            negotiate_timeout: Duration::from_secs(30),
            backoff: Backoff::default(),
            send_queue: 1024,
            recv_queue: 1024,
            max_message_len: cottak_proto::frame::DEFAULT_MAX_FRAME_LEN,
            drop_stale: true,
        }
    }

    /// Enables TLS.
    #[must_use]
    pub fn with_tls(mut self, tls: TlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// Sets the protocol preference.
    #[must_use]
    pub fn with_protocol(mut self, protocol: ProtocolPreference) -> Self {
        self.protocol = protocol;
        self
    }

    /// Sets the keepalive ping interval and the matching read timeout (3x).
    #[must_use]
    pub fn with_keepalive(mut self, ping_interval: Duration) -> Self {
        self.ping_interval = ping_interval;
        self.read_timeout = ping_interval * 3;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_is_capped_and_jittered() {
        let b = Backoff {
            min: Duration::from_secs(1),
            max: Duration::from_secs(8),
        };
        for _ in 0..200 {
            let d0 = b.delay(0);
            assert!(
                d0 >= Duration::from_millis(500) && d0 <= Duration::from_secs(1),
                "{d0:?}"
            );
            let d2 = b.delay(2);
            assert!(
                d2 >= Duration::from_secs(2) && d2 <= Duration::from_secs(4),
                "{d2:?}"
            );
            let big = b.delay(500);
            assert!(
                big >= Duration::from_secs(4) && big <= Duration::from_secs(8),
                "{big:?}"
            );
        }
    }
}
