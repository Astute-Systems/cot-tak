# Mesh networking

Without any server, TAK devices on the same network share situational awareness by sending one
CoT event per **UDP multicast** datagram. `cottak::net::mesh` (feature `net`) speaks that
protocol.

| Group | Address | Used for |
|---|---|---|
| Situational awareness | `239.2.3.1:6969` (`SA_GROUP`, `SA_PORT`) | Positions, markers, shapes |
| GeoChat | `224.10.10.1:17012` (`CHAT_GROUP`, `CHAT_PORT`) | Chat messages |

## Sending and receiving

```rust,no_run
use cottak::core::{Event, Point};
use cottak::net::mesh::{MeshConfig, MeshSocket};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
// Binding is synchronous: it joins the multicast group.
let socket = MeshSocket::bind(&MeshConfig::default())?;

socket
    .send(&Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?))
    .await?;

loop {
    let (event, from) = socket.recv().await?;
    println!("{} from {from}: {}", event.uid, event.cot_type);
}
# }
```

`recv` skips datagrams that are not valid CoT (or that carry only control information), so a noisy
network does not stop your loop. Receivers detect the format of each datagram automatically.

## Configuration

`MeshConfig::default()` is the SA group; `MeshConfig::chat()` is the GeoChat group. Fields:

| Field | Default | Meaning |
|---|---|---|
| `group` | `239.2.3.1` | Multicast group to join and send to (a unicast address is used as a plain destination) |
| `port` | `6969` | UDP port |
| `interface` | `0.0.0.0` | Local interface for multicast; unspecified lets the OS choose |
| `ttl` | `1` | Multicast hops |
| `loopback` | `false` | Also receive your own datagrams |
| `protocol` | `MeshProtocol::Xml` | Format of outgoing datagrams |

```rust
use cottak::net::mesh::{MeshConfig, MeshProtocol, CHAT_PORT};

let chat = MeshConfig {
    protocol: MeshProtocol::V1,
    loopback: true,
    ..MeshConfig::chat()
};
assert_eq!(chat.port, CHAT_PORT);
```

## XML or protocol v1?

Outgoing datagrams are **XML by default**, which every TAK client understands. Choose
`MeshProtocol::V1` only when every peer supports [TAK Protocol v1](protocol.md).

!!! info "Not implemented: dynamic version tracking"
    The TAK Protocol specification describes tracking each contact's supported protocol version
    from its `TakControl` messages and choosing per peer. `cottak` implements only *static*
    selection: you pick the format in `MeshConfig`. Receiving always auto-detects.

## Notes

- Multicast needs a network that forwards it (many Wi-Fi access points and VPNs do not). Test with
  `loopback: true` on one machine first.
- Datagrams are limited to the UDP maximum for IPv4 (65,507 bytes); larger events are an error.
- There is no authentication or encryption on the mesh; that is a property of the TAK mesh
  protocol itself.
