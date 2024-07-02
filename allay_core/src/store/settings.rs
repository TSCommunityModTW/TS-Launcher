use std::{collections::HashMap, path::Path};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json}; 
use ts_rs::TS;


use crate::util;

#[derive(Debug)]
pub enum JavaPathVersion {
    Java8,
    Java16,
    Java17
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreSettingsJava")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IStoreSettingsJava.ts")]
pub struct Java {
    pub id: String,
    pub java17_path: String,
    pub java16_path: String,
    pub java8_path: String,
    pub java_path: String,
    pub ram_max_size: i32,
    pub ram_min_size: i32,
    pub java_parameter: String,
    pub is_built_in_java_vm: bool,
    pub ram_checked: bool,
    pub java_path_checked: bool,
    pub java_parameter_checked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreSettingSelectedServer")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IStoreSettingSelectedServer.ts")]
pub struct SelectedServerStart {
    pub main_id: String,
    pub child_server_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreSettingsGeneral")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IStoreSettingsGeneral.ts")]
pub struct General {
    pub open_game_keep_launcher_state: bool,
    // pub game_start_open_monitor_log: bool,
}



#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(rename = "IStoreSettings")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IStoreSettings.ts")]
pub struct Settings {
    pub language: String,
    pub java: HashMap<String, Java>,
    pub display_position: i32,
    pub launcher_keep_open: bool,
    pub selected_server_start: SelectedServerStart,
    pub general: General
}


impl Settings {
    
    pub async fn init(file_path: &Path) -> crate::Result<Self> {

        let settings = if let Ok(settings_json) = util::io::read_json_file::<Settings>(&file_path).await {
            tracing::debug!("Loaded setting's JSON.");
            settings_json
        } else if let Ok(value) = util::io::read_json_file::<Value>(&file_path).await {
            // If reading as Settings fails, try reading as Value for migration
            tracing::debug!("Loaded setting's JSON as Value for migration.");
            let migrated_value = migrate_settings(value)?;
            serde_json::from_value(migrated_value)?
        }
        else {
            let mut java: HashMap<String, Java> = HashMap::new();
            java.insert("global".to_owned(), Java {
                id: "global".to_owned(),
                java17_path: "".to_owned(),
                java16_path: "".to_owned(),
                java8_path: "".to_owned(),
                java_path: "".to_owned(),
                ram_max_size: 4096,
                ram_min_size: 4096,
                java_parameter: "".to_owned(),
                is_built_in_java_vm: true,
                ram_checked: false,
                java_path_checked: false,
                java_parameter_checked: false,
            });

            let settings = Settings {
                language: "".to_owned(),
                java,
                display_position: 0,
                launcher_keep_open: true,
                selected_server_start: SelectedServerStart{
                    main_id: "".to_owned(),
                    child_server_id: "".to_owned()
                },
                general: General {
                    open_game_keep_launcher_state: true,
                    // game_start_open_monitor_log: false,
                },
            };
            tracing::debug!("Created setting's JSON.");
            settings
        };

        Ok(settings)

    }

    pub async fn update_java(&mut self, id: &str, version: JavaPathVersion, path: &str) -> crate::Result<()> {
        tracing::debug!("Update_java");
        // let store = Store::get().await?;
        // let mut settings = store.settings.write().await;

        if let Some(java) = self.java.get_mut(id) {
            match version {
                JavaPathVersion::Java8 => java.java8_path = path.to_owned(),
                JavaPathVersion::Java16 => java.java16_path = path.to_owned(),
                JavaPathVersion::Java17 => java.java17_path = path.to_owned(),
            }
        }

        Ok(())
    }

    pub async fn sync(&self, file_path: &Path) -> crate::Result<()> {
        tracing::debug!("Sync");
        util::io::write_struct_file(file_path, &self).await?;
        Ok(())
    }

    
}


//Corvert 0.1.1 json to 0.1.2
fn migrate_settings(mut value: Value) -> crate::Result<Value> {
    if let Some(obj) = value.as_object_mut() {
        if let Some(selected_server_start) = obj.get("selected_server_start") {
            if selected_server_start.is_string() {
                let main_id: String = selected_server_start.as_str().unwrap().to_owned();
                let new_selected_server_start = json!({
                    "main_id": main_id,
                    "child_server_id": ""
                });
                obj.insert("selected_server_start".to_owned(), new_selected_server_start);
                tracing::debug!("Migrated selected_server_start to new format.");
            }
        }
        
    }
    Ok(value)
}