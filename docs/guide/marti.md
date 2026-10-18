# Marti API

TAK Server exposes a REST API under `/Marti/api` (HTTPS, default port 8443). `cottak-marti`
(`cottak::marti`, feature `marti`) is a typed client for the parts most applications need:
**missions** (data-sync feeds), **file sync**, and the **contacts, clients and groups** the server
knows about. Endpoints and response shapes were taken from the TAK Server source.

## Connecting

`MartiClient` uses the same [`TlsConfig`](client.md#tls) as the streaming client, usually with a
client certificate (mutual TLS):

```rust,no_run
use cottak::marti::{MartiClient, MartiConfig};
use cottak::net::TlsConfig;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let tls = TlsConfig::from_files(
    "server-ca.pem",
    Some(("client.pem".as_ref(), "client.key".as_ref())),
)?;
let marti = MartiClient::new(MartiConfig::new("tak.example.com", tls)).await?;

println!("server {}", marti.version().await?);
# Ok(()) }
```

For ports that use password login, add `.with_basic_auth(user, password)` to the config.
Redirects are never followed (credentials could otherwise leak to another host), and every
transfer has a size limit (`max_download` / `max_upload`).

## Missions

A mission is a named feed of CoT UIDs and files that subscribers keep in sync.

```rust,no_run
use cottak::marti::{ContentRef, MissionContent, MissionQuery, NewMission, UploadOptions};
# use cottak::marti::{MartiClient, MartiConfig};
# use cottak::net::TlsConfig;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
# let marti = MartiClient::new(MartiConfig::new("tak.example.com", TlsConfig::new())).await?;
// Create. `creator_uid` is the UID of the client doing it.
let mission = marti
    .create_mission("RECON-1", "ALPHA-1", &NewMission {
        description: Some("Reconnaissance".into()),
        ..NewMission::default()
    })
    .await?;
println!("created {} ({:?})", mission.name, mission.guid);

// Upload a file, then attach it (and some CoT UIDs) to the mission.
let file = marti.upload_file("plan.zip", b"...", &UploadOptions::default()).await?;
let hash = file["Hash"].as_str().unwrap().to_owned();
marti
    .add_content("RECON-1", &MissionContent { hashes: vec![hash.clone()], uids: vec!["UID-1".into()] }, "ALPHA-1")
    .await?;

// Follow along.
marti.subscribe("RECON-1", "BRAVO-2", None).await?;
let current = marti.mission("RECON-1", None).await?;
println!("{} files, {} uids", current.contents.len(), current.uids.len());
let latest = marti.mission_cot("RECON-1").await?;     // latest Event for each UID
println!("{} events", latest.len());

// Detach and clean up.
marti.remove_content("RECON-1", &ContentRef::Hash(hash), "ALPHA-1").await?;
marti.delete_mission("RECON-1", "ALPHA-1", false).await?;
# let _ = marti.missions(&MissionQuery::default()).await?;
# Ok(()) }
```

| Method | Endpoint |
|---|---|
| `missions`, `mission` | `GET /missions`, `GET /missions/{name}` |
| `create_mission`, `delete_mission` | `PUT`/`DELETE /missions/{name}` |
| `add_content`, `remove_content` | `PUT`/`DELETE /missions/{name}/contents` |
| `changes` | `GET /missions/{name}/changes` |
| `subscribe`, `unsubscribe`, `subscribers` | `PUT`/`DELETE /missions/{name}/subscription`, `GET .../subscriptions` |
| `mission_cot` | `GET /missions/{name}/cot` |

Names are percent-encoded for you, and names with control characters or over-long values are
rejected locally. Errors are typed: `Error::NotFound`, `Conflict` (name taken), `Forbidden`,
`Unauthorized`, `TooLarge` and `Status { status, body }` for anything else.

## Files

`upload_file` (`POST /Marti/sync/upload`) stores a file and returns the server's metadata JSON,
including its `Hash`. `upload_data_package` (`POST /Marti/sync/missionupload`) uploads a data
package and returns its download URL. `download(hash)` fetches content by hash.

```rust,no_run
# use cottak::marti::{MartiClient, MartiConfig, UploadOptions};
# use cottak::net::TlsConfig;
# async fn run() -> Result<(), Box<dyn std::error::Error>> {
# let marti = MartiClient::new(MartiConfig::new("tak.example.com", TlsConfig::new())).await?;
let package = marti
    .upload_data_package("briefing.zip", "ALPHA-1", b"PK...", &UploadOptions::default())
    .await?;
let bytes = marti.download(package.hash.as_deref().unwrap()).await?;
# let _ = bytes;
# Ok(()) }
```

## People and groups

```rust,no_run
use cottak::marti::EndpointQuery;
# use cottak::marti::{MartiClient, MartiConfig};
# use cottak::net::TlsConfig;
# async fn run() -> Result<(), Box<dyn std::error::Error>> {
# let marti = MartiClient::new(MartiConfig::new("tak.example.com", TlsConfig::new())).await?;
for c in marti.contacts().await? {
    println!("{:?} ({:?}) team {:?}", c.callsign, c.uid, c.team);
}
let clients = marti.client_endpoints(&EndpointQuery::default()).await?;
let groups = marti.groups().await?;
# let _ = (clients, groups);
# Ok(()) }
```

## Tolerant models

The JSON models (`Mission`, `Resource`, `Contact`, ...) treat every field as optional, accept
`null` as absent, accept timestamps as epoch milliseconds *or* ISO text, and keep unknown fields in
an `extra` map. A newer or older server therefore does not break decoding, and you can read fields
the models do not name.

## Testing against the embedded server

The [embedded server](server.md#the-marti-mock) mocks this API, so you can test mission code
without a TAK Server.

!!! note "Verification status"
    Endpoint paths and parameters were verified against TAK Server's source. Response shapes for
    uploads, contacts and subscriptions are inferred from that source and have not been checked
    against a live server.
