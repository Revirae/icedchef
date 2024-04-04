use serde::{Deserialize, Serialize};

use crate::models::{Inventory};
use crate::errors::{LoadError, SaveError};


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Persistence {
    pub inventory: Inventory
}

impl Persistence {
    fn path() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push("chef.json");
        path
    }
    pub async fn load() -> Result<Persistence, LoadError> {
        use async_std::prelude::*;
        let mut contents = String::new();
        let mut file = async_std::fs::File::open(
            Self::path()
        )
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&mut contents)
            .map_err(|_| LoadError::Format)
    }
    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }
        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        async_std::task::sleep(std::time::Duration::from_secs(2))
            .await;

        Ok(())
    }
}

