use std::env::var;

use anyhow::{anyhow, Result};

pub struct Settings {
    pub jwt: String,
    pub world_id: String,
    pub imap_domain: String,
    pub email_username: String,
    pub email_password: String,
    pub aws_bucket_name: String,
    pub aws_region: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        Ok(Self {
            jwt: var("JWT").map_err(|_| anyhow!("Error getting JWT from environment"))?,
            world_id: var("WORLD_ID")
                .map_err(|_| anyhow!("Error getting world id from environment"))?,
            imap_domain: var("IMAP_DOMAIN")
                .map_err(|_| anyhow!("Error getting imap domain from environment"))?,
            email_username: var("EMAIL_USERNAME")
                .map_err(|_| anyhow!("Error getting email username from environment"))?,
            email_password: var("EMAIL_PASSWORD")
                .map_err(|_| anyhow!("Error getting email password from environment"))?,
            aws_bucket_name: var("AWS_BUCKET_NAME")
                .map_err(|_| anyhow!("Error getting AWS bucket name from environment"))?,
            aws_region: var("AWS_REGION")
                .map_err(|_| anyhow!("Error getting AWS region from environment"))?,
        })
    }
}
