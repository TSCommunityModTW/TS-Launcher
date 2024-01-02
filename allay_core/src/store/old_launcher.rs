use std::{path::PathBuf, collections::HashMap};

use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::{util, app_path, io};

use super::ValueGetSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct MicrosoftAuth {
    mc_account_token: String,
    access_token: String,
    refresh_token: String,
    expires_at: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    name: String,
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileJson {
    microsoft_auth: MicrosoftAuth,
    user: User,
    player: Player,
    remember_status: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    json: ProfileJson,
    path: PathBuf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Java {
    server_id: String,
    java17_path: String,
    java16_path: String,
    java8_path: String,
    ram_max_size: i32,
    ram_min_size: i32,
    java_parameter: String,
    is_built_in_java_vm: bool,
    ram_checked: bool,
    java_path_checked: bool,
    java_parameter_checked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    open_game_keep_launcher_state: bool,
    game_start_open_monitor_log: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsJson {
    language: String,
    java: HashMap<String, Java>,
    display_position: i32,
    launcher_keep_open: bool,
    selected_server_start: String,
    general: General
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    json: SettingsJson,
    path: PathBuf
}

#[derive(Debug)]
pub struct LauncherStore {
    profile: Profile,
    settings: Settings,
}

const PROFILE_JSON: &str = "profile.json";
const SETTINGS_JSON: &str = "settings.json";

impl LauncherStore {

    pub async fn init() -> crate::Result<Self> {

        let profile_path = app_path::get_app_dir_path().join(PROFILE_JSON);
        let settings_path = app_path::get_app_dir_path().join(SETTINGS_JSON);

        let profile = if let Ok(profile_json) = io::read_json_file::<ProfileJson>(&profile_path).await {
            profile_json
        } else {
            ProfileJson {
                microsoft_auth: MicrosoftAuth {
                    mc_account_token: "".to_owned(),
                    access_token: "".to_owned(),
                    refresh_token: "".to_owned(),
                    expires_at: "".to_owned(),
                },
                user: User {
                    username: "".to_owned(),
                    id: "".to_owned(),
                },
                player: Player {
                    name: "".to_owned(),
                    uuid: "".to_owned(),
                },
                remember_status: true,
            }
        };

        util::io::write_struct_file(&profile_path, &profile).await?;

        let settings = if let Ok(settings_json) = io::read_json_file::<SettingsJson>(&settings_path).await {
            settings_json
        } else {

            let mut java: HashMap<String, Java> = HashMap::new();
            java.insert(String::from("global"), Java {
                server_id: String::from("global"),
                java17_path: "".to_owned(),
                java16_path: "".to_owned(),
                java8_path: "".to_owned(),
                ram_max_size: 4096,
                ram_min_size: 4096,
                java_parameter: "".to_owned(),
                is_built_in_java_vm: true,
                ram_checked: false,
                java_path_checked: false,
                java_parameter_checked: false,
            });

            SettingsJson {
                language: "".to_owned(),
                java,
                display_position: 0,
                launcher_keep_open: true,
                selected_server_start: "".to_owned(),
                general: General {
                    open_game_keep_launcher_state: true,
                    game_start_open_monitor_log: false,
                },
            }
        };

        util::io::write_struct_file(&settings_path, &settings).await?;

        Ok(LauncherStore {
            profile: Profile {
                json: profile,
                path: profile_path,
            },
            settings: Settings {
                json: settings,
                path: settings_path,
            },
        })
    }

    fn is_empty_insert_new_java(&mut self, id: &str) {
        if let None = self.settings.json.java.get(id) {
            self.settings.json.java.insert(id.to_string(), Java {
                server_id: id.to_string(),
                java17_path: "".to_owned(),
                java16_path: "".to_owned(),
                java8_path: "".to_owned(),
                ram_max_size: 4096,
                ram_min_size: 4096,
                java_parameter: "".to_owned(),
                is_built_in_java_vm: true,
                ram_checked: false,
                java_path_checked: false,
                java_parameter_checked: false,
            });
        }
    }

    pub async fn save(&self) -> crate::Result<()> {
        util::io::write_struct_file(&self.profile.path, &self.profile.json).await?;
        util::io::write_struct_file(&self.settings.path, &self.settings.json).await?;
        Ok(())
    }

    pub fn open_game_keep_launcher_state(&mut self) -> ValueGetSet<bool> {
        ValueGetSet(&mut self.settings.json.general.open_game_keep_launcher_state)
    }

    pub fn game_start_open_monitor_log(&mut self) -> ValueGetSet<bool> {
        ValueGetSet(&mut self.settings.json.general.game_start_open_monitor_log)
    }

    pub fn ram_checked(&mut self, id: &str) -> ValueGetSet<bool> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().ram_checked)
    }

    pub fn java_path_checked(&mut self, id: &str) -> ValueGetSet<bool> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java_path_checked)
    }

    pub fn java_parameter_checked(&mut self, id: &str) -> ValueGetSet<bool> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java_parameter_checked)
    }

    pub fn user_username(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.user.username)
    }

    pub fn user_id(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.user.id)
    }

    pub fn player_name(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.player.name)
    }

    pub fn player_uuid(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.player.uuid)
    }

    pub fn remember_status(&mut self) -> ValueGetSet<bool> {
        ValueGetSet(&mut self.profile.json.remember_status)
    }

    pub fn language(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.settings.json.language)
    }

    pub fn java17_path(&mut self, id: &str) -> ValueGetSet<String> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java17_path)
    }

    pub fn java16_path(&mut self, id: &str) -> ValueGetSet<String> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java16_path)
    }

    pub fn java8_path(&mut self, id: &str) -> ValueGetSet<String> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java8_path)
    }

    pub fn ram_max_size(&mut self, id: &str) -> ValueGetSet<i32> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().ram_max_size)
    }

    pub fn ram_mix_size(&mut self, id: &str) -> ValueGetSet<i32> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().ram_min_size)
    }

    pub fn java_parameter(&mut self, id: &str) -> ValueGetSet<String> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().java_parameter)
    }

    pub fn is_built_in_java_vm(&mut self, id: &str) -> ValueGetSet<bool> {
        self.is_empty_insert_new_java(id);
        ValueGetSet(&mut self.settings.json.java.get_mut(id).unwrap().is_built_in_java_vm)
    }

    pub fn display_position(&mut self) -> ValueGetSet<i32> {
        ValueGetSet(&mut self.settings.json.display_position)
    }

    pub fn launcher_keep_open(&mut self) -> ValueGetSet<bool> {
        ValueGetSet(&mut self.settings.json.launcher_keep_open)
    }

    pub fn selected_server_start(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.settings.json.selected_server_start)
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

    pub fn microsoft_expires_at(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.microsoft_auth.expires_at)
    }

    pub fn microsoft_mc_account_token(&mut self) -> ValueGetSet<String> {
        ValueGetSet(&mut self.profile.json.microsoft_auth.mc_account_token)
    }
}