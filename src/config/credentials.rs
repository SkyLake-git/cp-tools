use keyring::Entry;
use secrecy::{ExposeSecret, SecretString};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const SERVICE_NAME: &str = "cp-tools";
const ATCODER_ACC_NAME: &str = "atcoder-revel-session";

pub struct AtCoderCredentials {
    pub revel_session: SecretString,
}

impl AtCoderCredentials {
    pub fn new(revel_session: SecretString) -> Self {
        Self { revel_session }
    }
}

fn format_dotenv_key(key: &str) -> String {
    key.to_uppercase().replace("-", "_")
}

fn delete_secret_from_dotenv(key: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(".env")?;

    let res: String = content
        .lines()
        .filter(|line| return !line.starts_with(format!("{}=", format_dotenv_key(key)).as_str()))
        .collect();

    Ok(fs::write(".env", res.to_owned())?)
}

fn delete_secret(key: &str) -> anyhow::Result<()> {
    delete_secret_from_dotenv(key).ok();
    let entry = Entry::new(SERVICE_NAME, ATCODER_ACC_NAME)?;
    entry.delete_credential().ok();
    Ok(())
}

fn fetch_secret_keyring(key: &str) -> anyhow::Result<String> {
    Ok(Entry::new(SERVICE_NAME, key)?.get_password()?)
}

fn fetch_secret(key: &str) -> anyhow::Result<String> {
    if let Ok(pass) = fetch_secret_keyring(key) {
        return Ok(pass);
    }

    Ok(std::env::var(format_dotenv_key(key))?)
}

fn save_secret(key: &str, secret: String, to_dotenv: bool) -> anyhow::Result<()> {
    if to_dotenv {
        let mut file = OpenOptions::new().create(true).append(true).open(".env")?;

        writeln!(file, "{}", format!("{}={secret}", format_dotenv_key(key)))?;
    } else {
        let entry = Entry::new(SERVICE_NAME, ATCODER_ACC_NAME)?;

        entry.set_password(&secret)?;
    }
    Ok(())
}

pub fn fetch_atcoder() -> anyhow::Result<AtCoderCredentials> {
    Ok(AtCoderCredentials::new(SecretString::from(fetch_secret(
        ATCODER_ACC_NAME,
    )?)))
}

pub fn save_atcoder(cred: AtCoderCredentials, to_dotenv: bool) -> anyhow::Result<()> {
    save_secret(
        ATCODER_ACC_NAME,
        cred.revel_session.expose_secret().to_owned(),
        to_dotenv,
    )
}

pub fn delete_atcoder() -> anyhow::Result<()> {
    delete_secret(ATCODER_ACC_NAME)
}
