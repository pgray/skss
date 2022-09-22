use super::KeyHost;

pub async fn get_keys(host: &KeyHost, user: &str) -> Result<String, Box<dyn std::error::Error>> {
    match host {
        KeyHost::Github => Ok(reqwest::get(format!("https://github.com/{}.keys", user))
            .await?
            .text()
            .await?),
        KeyHost::Gitlab => Ok(reqwest::get(format!("https://gitlab.com/{}.keys", user))
            .await?
            .text()
            .await?),
        KeyHost::Sourcehut => Ok(reqwest::get(format!("https://meta.sr.ht/~{}.keys", user))
            .await?
            .text()
            .await?),
    }
}
use std::fs::{create_dir, File};
use std::io::prelude::*;
pub async fn write_tmp(
    user: String,
    keys: String,
    tmpdir: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new(&tmpdir).is_dir() {
        create_dir(&tmpdir)?;
    }
    let mut file = File::create(format!("{}/{}.keys", tmpdir, user))?;
    file.write_all(keys.as_bytes())?;
    Ok(())
}

use std::path::Path;
pub async fn update(user: &String, tmpdir: String) -> Result<bool, Box<dyn std::error::Error>> {
    let user_auth_path = format!("/home/{}/.ssh/authorized_keys", user);
    let user_tmp_path = format!("{}/{}.keys", tmpdir, user);
    if Path::new(&user_auth_path).is_file() {
        let uaps = std::fs::metadata(user_auth_path)?;
        let utps = std::fs::metadata(user_tmp_path)?;
        if uaps.len() == utps.len() {
            return Ok(false);
        }
    }
    Ok(true)
}
