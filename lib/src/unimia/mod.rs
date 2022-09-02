#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// This struct is the general configuration used to access UniMia.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnimiaUserConfig {
    pub username: String,
    pub password: String,
}
