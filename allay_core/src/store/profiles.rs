use std::path::Path;

use keyring::Entry;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::util;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreProfilesMicrosoftAuth")]
#[ts(export, export_to = "../src/interfaces/IStoreProfilesMicrosoftAuth.ts")]
pub struct MicrosoftAuth {
    pub mc_account_token: String,
    pub expires_at: String
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreProfilesUser")]
#[ts(export, export_to = "../src/interfaces/IStoreProfilesUser.ts")]
pub struct User {
    pub username: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreProfilesPlayer")]
#[ts(export, export_to = "../src/interfaces/IStoreProfilesPlayer.ts")]
pub struct Player {
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreProfiles")]
#[ts(export, export_to = "../src/interfaces/IStoreProfiles.ts")]
pub struct Profiles {
    pub microsoft_auth: MicrosoftAuth,
    pub user: User,
    pub player: Player
}

impl Profiles {
    
    pub async fn init(file_path: &Path) -> crate::Result<Self> {

        let profile = if let Ok(profile_json) = util::io::read_json_file::<Profiles>(&file_path).await {
            profile_json
        } else {
            Profiles {
                microsoft_auth: MicrosoftAuth {
                    mc_account_token: "".to_owned(),
                    expires_at: "".to_owned(),
                },
                user: User {
                    username: "".to_owned(),
                    id: "".to_owned(),
                },
                player: Player {
                    name: "".to_owned(),
                    uuid: "".to_owned(),
                }
            }
        };

        Ok(profile)
    }

    pub fn get_microsoft_access_token(&self) -> crate::Result<String> {
        let entry = Entry::new(util::config::KEYTAR_SERVICE, "accesstoken")?;
        Ok(entry.get_password()?)
    }

    pub fn set_microsoft_access_token(&self, access_token: &str) -> crate::Result<()> {
        let entry = Entry::new(util::config::KEYTAR_SERVICE, "accesstoken")?;
        if access_token.len() <= 0 {
            entry.delete_password()?;
        } else {
            entry.set_password(access_token)?;
        }
        Ok(())
    }

    pub fn get_microsoft_refresh_token(&self) -> crate::Result<String> {
        let entry = Entry::new(util::config::KEYTAR_SERVICE, "refreshtoken")?;
        Ok(entry.get_password()?)
    }

    pub fn set_microsoft_refresh_token(&self, refresh_token: &str) -> crate::Result<()> {
        let entry = Entry::new(util::config::KEYTAR_SERVICE, "refreshtoken")?;
        if refresh_token.len() <= 0 {
            entry.delete_password()?;
        } else {
            entry.set_password(refresh_token)?;
        }
        Ok(())
    }

    pub async fn sync(&self, file_path: &Path) -> crate::Result<()> {
        util::io::write_struct_file(file_path, &self).await?;
        Ok(())
    }
}