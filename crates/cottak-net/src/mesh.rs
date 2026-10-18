//! Mesh (serverless) transport: Situational Awareness over UDP multicast.
//!
//! Each datagram carries exactly one CoT event, either as XML or as a TAK
//! Protocol v1 mesh frame (`0xbf 0x01 0xbf <protobuf>`). Receivers detect the
//! format automatically.
//!
//! Only static protocol selection is implemented. The dynamic per-contact
//! version tracking described in the TAK Protocol specification is not; if
//! you need to interoperate with XML-only peers, keep [`MeshProtocol::Xml`]
//! (the default).

use crate::error::{Error, Result};
use cottak_core::Event;
use cottak_proto::{frame, TakFrame};
use socket2::{Domain, Protocol as SockProtocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;
use tracing::trace;

/// Default Situational Awareness multicast group used by ATAK/WinTAK.
pub const SA_GROUP: Ipv4Addr = Ipv4Addr::new(239, 2, 3, 1);
/// Default Situational Awareness port.
pub const SA_PORT: u16 = 6969;
/// Default GeoChat multicast group.
pub const CHAT_GROUP: Ipv4Addr = Ipv4Addr::new(224, 10, 10, 1);
/// Default GeoChat port.
pub const CHAT_PORT: u16 = 17012;

/// Largest UDP payload over IPv4.
const MAX_DATAGRAM: usize = 65_507;

/// Wire format for outgoing datagrams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshProtocol {
    /// Legacy XML; understood by every TAK client.
    #[default]
    Xml,
    /// TAK Protocol v1 (protobuf).
    V1,
}

/// Settings for a [`MeshSocket`].
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Multicast group to join and send to. A non-multicast address is used
    /// as a plain unicast destination.
    pub group: Ipv4Addr,
    /// UDP port.
    pub port: u16,
    /// Local interface address for multicast; `UNSPECIFIED` lets the OS choose.
    pub interface: Ipv4Addr,
    /// Multicast time-to-live (hops).
    pub ttl: u32,
    /// Receive our own multicast datagrams.
    pub loopback: bool,
    /// Format of outgoing datagrams.
    pub protocol: MeshProtocol,
}

impl Default for MeshConfig {
    /// The ATAK situational-awareness group: `239.2.3.1:6969`.
    fn default() -> Self {
        Self {
            group: SA_GROUP,
            port: SA_PORT,
            interface: Ipv4Addr::UNSPECIFIED,
            ttl: 1,
            loopback: false,
            protocol: MeshProtocol::Xml,
        }
    }
}

impl MeshConfig {
    /// The GeoChat group `224.10.10.1:17012`.
    pub fn chat() -> Self {
        Self {
            group: CHAT_GROUP,
            port: CHAT_PORT,
            ..Self::default()
        }
    }
}

/// A UDP socket joined to a mesh multicast group.
#[derive(Debug)]
pub struct MeshSocket {
    socket: UdpSocket,
    dest: SocketAddr,
    protocol: MeshProtocol,
}

impl MeshSocket {
    /// Binds to the configured port and joins the group.
    ///
    /// Must be called from within a Tokio runtime.
    pub fn bind(config: &MeshConfig) -> Result<Self> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(SockProtocol::UDP))?;
        socket.set_reuse_address(true)?;
        #[cfg(all(unix, not(any(target_os = "solaris", target_os = "illumos"))))]
        socket.set_reuse_port(true)?;
        socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port).into())?;

        if config.group.is_multicast() {
            socket.join_multicast_v4(&config.group, &config.interface)?;
            socket.set_multicast_ttl_v4(config.ttl)?;
            socket.set_multicast_loop_v4(config.loopback)?;
            if !config.interface.is_unspecified() {
                socket.set_multicast_if_v4(&config.interface)?;
            }
        }
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket: UdpSocket::from_std(socket.into())?,
            dest: SocketAddrV4::new(config.group, config.port).into(),
            protocol: config.protocol,
        })
    }

    /// The local address the socket is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket.local_addr()?)
    }

    /// Sends one event to the group.
    pub async fn send(&self, event: &Event) -> Result<()> {
        let bytes = match self.protocol {
            MeshProtocol::Xml => event.to_xml_string()?.into_bytes(),
            MeshProtocol::V1 => TakFrame::from_event(event.clone()).encode_mesh()?,
        };
        if bytes.len() > MAX_DATAGRAM {
            return Err(Error::MessageTooLarge(MAX_DATAGRAM));
        }
        self.socket.send_to(&bytes, self.dest).await?;
        Ok(())
    }

    /// Waits for the next valid event and the address it came from.
    ///
    /// Datagrams that are not valid CoT (or that carry only control
    /// information) are skipped.
    pub async fn recv(&self) -> Result<(Event, SocketAddr)> {
        let mut buf = vec![0u8; MAX_DATAGRAM];
        loop {
            let (n, from) = self.socket.recv_from(&mut buf).await?;
            match decode_datagram(&buf[..n]) {
                Ok(Some(ev)) => return Ok((ev, from)),
                Ok(None) => trace!(%from, "mesh datagram without an event"),
                Err(e) => trace!(%from, error = %e, "ignoring invalid mesh datagram"),
            }
        }
    }
}

/// Decodes one datagram in either format.
pub(crate) fn decode_datagram(data: &[u8]) -> Result<Option<Event>> {
    if data.first() == Some(&frame::MAGIC) {
        return Ok(TakFrame::decode_mesh(data)?.event);
    }
    let text = std::str::from_utf8(data)
        .map_err(|_| Error::Core(cottak_core::Error::Xml("datagram is not UTF-8".into())))?;
    Ok(Some(Event::from_xml_str(text)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::Contact;
    use cottak_core::Point;
    use std::time::Duration;
    use tokio::time::timeout;

    fn ev(uid: &str) -> Event {
        Event::new(uid, "a-f-G".parse().unwrap(), Point::new(1.0, 2.0).unwrap())
            .with(Contact::new("TEST"))
    }

    fn pair(protocol: MeshProtocol) -> (MeshSocket, MeshSocket) {
        // Unicast to loopback so the test does not depend on multicast routing.
        let rx = MeshSocket::bind(&MeshConfig {
            group: Ipv4Addr::LOCALHOST,
            port: 0,
            ..MeshConfig::default()
        })
        .unwrap();
        let port = rx.local_addr().unwrap().port();
        let tx = MeshSocket::bind(&MeshConfig {
            group: Ipv4Addr::LOCALHOST,
            port: 0,
            protocol,
            ..MeshConfig::default()
        })
        .unwrap();
        let tx = MeshSocket {
            dest: SocketAddrV4::new(Ipv4Addr::LOCALHOST, port).into(),
            ..tx
        };
        (tx, rx)
    }

    #[tokio::test]
    async fn xml_and_protobuf_datagrams_round_trip() {
        for protocol in [MeshProtocol::Xml, MeshProtocol::V1] {
            let (tx, rx) = pair(protocol);
            tx.send(&ev("U1")).await.unwrap();
            let (got, _) = timeout(Duration::from_secs(5), rx.recv())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(got.uid, "U1");
            assert_eq!(
                got.detail.get::<Contact>().unwrap().callsign.as_deref(),
                Some("TEST")
            );
        }
    }

    #[tokio::test]
    async fn garbage_datagrams_are_skipped() {
        let (tx, rx) = pair(MeshProtocol::Xml);
        tx.socket.send_to(b"not cot", tx.dest).await.unwrap();
        tx.socket
            .send_to(&[0xBF, 0x09, 0xBF, 1, 2], tx.dest)
            .await
            .unwrap();
        tx.send(&ev("AFTER")).await.unwrap();
        let (got, _) = timeout(Duration::from_secs(5), rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got.uid, "AFTER");
    }

    #[test]
    fn default_is_the_atak_sa_group() {
        let c = MeshConfig::default();
        assert_eq!((c.group, c.port), (Ipv4Addr::new(239, 2, 3, 1), 6969));
        assert!(c.group.is_multicast());
    }
}
