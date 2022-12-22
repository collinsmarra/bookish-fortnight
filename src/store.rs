use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

use crate::api::tasks::Task;

#[derive(Store, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct TokenStore{
    pub token: String,
    pub username: String,
    pub tasks: Vec<Task>,
}
