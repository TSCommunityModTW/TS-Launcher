use std::{path::Path, collections::HashMap};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::util;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModpackFile {
    pub name: String,
    pub path: String,
    pub sha1: String,
    pub size: i32,
    pub download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modpack {
    pub name: String,
    pub version: String,
    pub project_id: i32,
    pub file_id: i32,
    pub files: Vec<ModpackFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModLoader {
    pub r#type: String,
    pub id: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleFile {
    pub name: String,
    pub r#type: String,
    pub action: String,
    pub project_id: i32,
    pub file_id: i32,
    pub file_name: String,
    pub file_path: String,
    pub sha1: String,
    pub size: i32,
    pub version: String,
    pub download_url: String,
    pub user_revert: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    pub size: usize,
    pub modules: Vec<ModuleFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceJson {
    pub id: String,
    pub minecraft_version: String,
    pub modpack: Modpack,
    pub mod_loader: ModLoader,
    pub module: Module
}

pub struct Instances(HashMap<String, Arc<RwLock<InstanceJson>>>);

impl Instances {

    pub fn new() -> Self {
        Instances(HashMap::new())
    }

    pub async fn init(&mut self, file_path: &Path) -> crate::Result<()> {

        let mut instance = if let Ok(instance_json) = util::io::read_json_file::<HashMap<String, InstanceJson>>(&file_path).await {
            instance_json
        } else {
            HashMap::new()
        };

        for (_, cache) in instance.drain() {
            let id = &cache.id;
            let cache = Arc::new(RwLock::new(cache.clone()));
            self.0.insert(id.to_owned(), cache);
        }

        Ok(())
    }

    pub fn insert_new_instance(&mut self, id: &str, instance: InstanceJson) -> crate::Result<Arc<RwLock<InstanceJson>>> {
        let instance = Arc::new(RwLock::new(instance));
        self.0.insert(id.to_owned(), instance.clone());
        Ok(instance)
    }

    pub fn get(&self, id: &str) -> Option<Arc<RwLock<InstanceJson>>> {
        self.0.get(id).cloned()
    }

    pub async fn sync(&self, file_path: &Path) -> crate::Result<()> {

        let mut write_instances: HashMap<String, InstanceJson> = HashMap::new();

        for key in self.0.keys() {
            if let Some(instances) = self.0.get(key) {
                let instances = instances.clone();
                let instances = instances.write().await;
                write_instances.insert(key.to_owned(), instances.clone());
            }
        }

        util::io::write_struct_file(file_path, &write_instances).await?;

        Ok(())
    }
}