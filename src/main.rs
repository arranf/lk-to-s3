#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::collections::HashSet;
use std::{thread, time};

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use config::{Config, Environment, File};
use imap::{connect, types::Uid};
use native_tls::TlsConnector;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use ureq;

mod export_request;
use crate::export_request::ExportRequest;

fn main() -> Result<()> {
    // Get settings from config file, prefer env variables
    let mut settings = Config::default();
    settings
        .merge(File::with_name("config"))
        .unwrap()
        .merge(Environment::new())
        .unwrap();

    request_export(&settings)?;
    println!("Export requested");
    println!("Waiting for email");
    thread::sleep(time::Duration::from_secs(90));
    println!("Getting link from email");
    let export_link = get_export_link(&settings)?;
    println!("Export Link: {}", export_link);
    save_to_s3(&settings, export_link.to_owned())?;
    Ok(())
}

fn request_export(settings: &Config) -> Result<()> {
    // Request export in JSON
    ureq::post("https://app.legendkeeper.com/api")
        .set(
            "Authorization",
            &format!(
                "Bearer {0}",
                settings.get_str("jwt").expect("Expected JWT to be set")
            ),
        )
        .set("Content-Type", "application/json")
        .send_string(&serde_json::to_string(&ExportRequest::new(
            settings.get("world-id")?,
        ))?);
    Ok(())
}

fn get_export_link(settings: &Config) -> Result<String> {
    let domain: &str = &settings.get_str("imap-domain")?;
    let tls = TlsConnector::builder().build()?;

    // We pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = connect((domain, 993), domain, &tls)?;

    let mut imap_session = client
        .login(
            &settings.get_str("email-username")?,
            &settings.get_str("email-password")?,
        )
        .map_err(|e| e.0)?;

    // Get most recent LK export emails
    imap_session.select("INBOX")?;
    let haystack_uids: HashSet<Uid> =
        imap_session.uid_search("NEW SUBJECT \"LegendKeeper export\"")?;

    let mut most_recent_email_uid: Option<Uid> = None;
    let mut export_url: Option<String> = None;

    // Get most recent LK export URL
    for uid in haystack_uids.iter() {
        let message = imap_session.uid_fetch(uid.to_string(), "RFC822")?;
        let message = message
            .iter()
            .next()
            .ok_or_else(|| anyhow!("No message for UID"));
        let body = message.expect("Expect message to exist").body();
        let body = std::str::from_utf8(body.expect("Expect message to have body"))
            .expect("message was not valid utf-8")
            .to_string();
        if body.contains("export") && body.contains("files.legendkeeper.com") {
            let result: String = body
                .split(" ")
                .filter(|a| a.starts_with("https://files"))
                .take(1)
                .take(1)
                .collect();
            let end_bytes = result.find(".zip").unwrap_or(result.len());
            let result = result[0..end_bytes + "zip".len() + 1].to_owned();
            if !result.is_empty() {
                most_recent_email_uid = Some(uid.to_owned());
                export_url = Some(result);
                break;
            }
        }
    }

    // Mark the email as deleted
    if most_recent_email_uid.is_some() {
        let uid = most_recent_email_uid.unwrap();
        imap_session.uid_store(uid.to_string(), "+FLAGS (\\Deleted)")?;
        imap_session.expunge()?;
    }
    imap_session.logout()?;

    export_url.ok_or_else(|| anyhow!("Unable to get export URL"))
}

fn save_to_s3(settings: &Config, url: String) -> Result<()> {
    let region = settings.get_str("aws-region")?.parse()?;
    let credentials = Credentials::default_blocking()?;
    let bucket = Bucket::new(&settings.get_str("aws-bucket-name")?, region, credentials)?;

    let response = ureq::get(&url).call();
    if response.ok() {
        let mut reader = response.into_reader();
        let now = Utc::now().format("%Y-%m-%d-T%H-%M-%S").to_string();
        bucket.put_object_stream_blocking(&mut reader, &format!("/backup/{}.zip", now))?;
        return Ok(());
    }
    Err(anyhow!("Error downloading data from LegendKeeper"))
}
