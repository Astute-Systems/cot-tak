//! Create a mission, upload a file and attach it, then list the mission's contents.
//!
//! ```text
//! cargo run -p cottak --example mission_upload --features marti -- \
//!     tak.example.com ca.pem client.pem client.key RECON-1 ./plan.zip
//! ```

use cottak::marti::{MartiClient, MartiConfig, MissionContent, NewMission, UploadOptions};
use cottak::net::TlsConfig;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let [host, ca, cert, key, mission, file] = a.as_slice() else {
        return Err(
            "usage: mission_upload <host> <ca.pem> <client.pem> <client.key> <mission> <file>"
                .into(),
        );
    };
    let tls = TlsConfig::from_files(ca, Some((Path::new(cert), Path::new(key))))?;
    let marti = MartiClient::new(MartiConfig::new(host.as_str(), tls)).await?;
    println!("server version {}", marti.version().await?);

    let uid = "cottak-example";
    match marti
        .create_mission(mission, uid, &NewMission::default())
        .await
    {
        Ok(m) => println!("created mission {}", m.name),
        Err(cottak::marti::Error::Conflict) => println!("mission {mission} already exists"),
        Err(e) => return Err(e.into()),
    }

    let data = std::fs::read(file)?;
    let name = Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("upload.bin");
    let meta = marti
        .upload_file(name, &data, &UploadOptions::default())
        .await?;
    let hash = meta["Hash"]
        .as_str()
        .or_else(|| meta["hash"].as_str())
        .ok_or("server reply had no hash")?;

    let updated = marti
        .add_content(
            mission,
            &MissionContent {
                hashes: vec![hash.to_owned()],
                uids: vec![],
            },
            uid,
        )
        .await?;
    for item in &updated.contents {
        println!("content: {:?}", item.data.filename);
    }
    Ok(())
}
