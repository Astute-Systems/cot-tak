//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains a TAK package creator.
//!

use crate::cot::FileShare;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use uuid::Uuid;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

/// Create a data package
/// # Arguments
/// * `fileshare` - A mutable reference to a FileShare
/// * `directory` - A string slice that holds the directory path
/// # Returns
/// * A Result containing a string
/// # Errors
/// * An io::Error
///
pub fn create_data_package(fileshare: &mut FileShare, directory: &str) -> io::Result<String> {
    let data_package_path = format!("{}/{}.zip", directory, fileshare.hash());
    let file = File::create(&data_package_path)?;
    let mut zip = ZipWriter::new(file);

    let manifest_text = compose_manifest(fileshare)?;

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("MANIFEST/manifest.xml", options)
        .expect("Failed to create manifest file");
    zip.write_all(manifest_text.as_bytes())?;

    // Loop through vector of paths
    for attachment_path in &fileshare.attachment_paths {
        println!("Adding attachment: {}", attachment_path);
        let arcname = get_attachment_arcname(fileshare, attachment_path);
        zip.start_file(arcname, options)?;
        let mut f = File::open(attachment_path)?;
        io::copy(&mut f, &mut zip)?;
    }

    zip.finish()?;

    // Update the size
    fileshare.size_in_bytes = Path::new(&data_package_path).metadata()?.len() as i32;

    // Create the sha256 hash
    fileshare.sha256 = format!("{:x}", Sha256::digest(&std::fs::read(&data_package_path)?));

    Ok(data_package_path)
}

/// Compose the manifest
/// # Arguments
/// * `fileshare` - A reference to a FileShare
/// # Returns
/// * A Result containing a string
/// # Errors
/// * An io::Error
///
/// ```xml
/// <MissionPackageManifest version="2">
/// <Configuration>
///     <Parameter name="uid" value="some-uuid"/>
///     <Parameter name="name" value="some-package-name"/>
///     <Parameter name="onReceiveDelElementTreee" value="true"/>
/// </Configuration>
/// <Contents>
///     <Content ignore="false" zipEntry="some-uuid/some-hash.ext">
///         <Parameter name="uid" value="some-uid"/>
///         <Parameter name="contentType" value="application/octet-stream"/>
///     </Content>
///     <Content ignore="false" zipEntry="some-uuid/another-hash.ext">
///         <Parameter name="uid" value="some-uid"/>
///         <Parameter name="contentType" value="application/octet-stream"/>
///     </Content>
/// </Contents>
/// </MissionPackageManifest>
/// ```
///
fn compose_manifest(fileshare: &FileShare) -> io::Result<String> {
    let pre = &format!(
        r#"<MissionPackageManifest version="2">
    <Configuration>
        <Parameter name="uid" value="{uid}"/>
        <Parameter name="name" value="{name}"/>
        <Parameter name="onReceiveDelElementTreee" value="{value}"/>
    </Configuration>
    <Contents>"#,
        uid = fileshare.uid,
        name = fileshare.name,
        value = true
    );

    let post = r#"</Contents>
</MissionPackageManifest>"#;

    let mut content = String::new();
    for attachment_path in &fileshare.attachment_paths {
        content += &format!(
            r#"<Content ignore="{ignore}" zipEntry="{zipEntry}">
        <Parameter name="uid" value="{value}"/>
        <Parameter name="contentType" value="{content_type}"/>
    </Content>"#,
            ignore = false,
            zipEntry = get_attachment_arcname(fileshare, attachment_path),
            value = Uuid::new_v4(),
            content_type = mime_guess::from_path(attachment_path).first_or_octet_stream()
        );
    }

    Ok(format!("{}{}{}", pre, content, post))
}

/// Get the attachment arcname
/// # Arguments
/// * `fileshare` - A reference to a FileShare
/// * `attachment_path` - A string slice that holds the attachment path
/// # Returns
/// * A string
///
fn get_attachment_arcname(fileshare: &FileShare, attachment_path: &str) -> String {
    let filename = Path::new(attachment_path)
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let extension = Path::new(attachment_path)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    format!("{}/{}.{}", fileshare.uid, filename, extension)
}
