use keyring::{Entry, Error, store_info, use_native_store};
use secrecy::{ExposeSecret, SecretString};

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

pub fn fetch_atcoder() -> Result<AtCoderCredentials, Error> {
    let entry = Entry::new(SERVICE_NAME, ATCODER_ACC_NAME)?;

    Ok(AtCoderCredentials::new(SecretString::from(
        entry.get_password()?,
    )))
}

pub fn save_atcoder(cred: AtCoderCredentials) -> Result<(), Error> {
    let entry = Entry::new(SERVICE_NAME, ATCODER_ACC_NAME)?;

    entry.set_password(cred.revel_session.expose_secret())?;
    Ok(())
}

pub fn delete_atcoder() -> Result<(), Error> {
    let entry = Entry::new(SERVICE_NAME, ATCODER_ACC_NAME)?;

    entry.delete_credential()?;
    Ok(())
}
