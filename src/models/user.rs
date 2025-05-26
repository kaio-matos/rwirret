use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub anonymous: bool,
    pub username: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let mut permissions = HashSet::new();

        permissions.insert("Category::View".to_owned());

        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            permissions,
        }
    }
}

impl User {
    pub async fn get_user(id: i64, pool: &SqlitePool) -> Option<Self> {
        let sqluser = database::SqlUser::get(id, pool).await?;

        // lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
        let sql_user_perms = database::SqlPermissionTokens::get(id, pool).await?;

        Some(sqluser.into_user(Some(sql_user_perms)))
    }
}
