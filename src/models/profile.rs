use serde::{Deserialize, Serialize};
pub mod api;
pub mod crypto;
pub mod encoder;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub iccid: Option<String>,
    pub k: Option<String>,
    pub opc: Option<String>,
    pub kid: Option<String>,
    pub kic: Option<String>,
    pub imsi: Option<String>,
    pub pin: Option<String>,
    pub puk: Option<String>,
    pub adm: Option<String>,
    pub smsp: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedProfile {
    iccid: String,
    profile: String,
}

impl EncryptedProfile {
    pub fn iccid(&self) -> &String {
        &self.iccid
    }

    pub fn profile(&self) -> &String {
        &self.profile
    }
}
