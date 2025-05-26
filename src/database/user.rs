use std::collections::HashSet;

use sqlx::SqlitePool;

use crate::models::User;

use super::DataModel;

impl DataModel for SqlUser {
    async fn create_table(pool: &SqlitePool) {
        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS users (
                    "id" INTEGER PRIMARY KEY,
                    "anonymous" BOOLEAN NOT NULL,
                    "username" VARCHAR(256) NOT NULL
                )
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS user_permissions (
                    "user_id" INTEGER NOT NULL,
                    "token" VARCHAR(256) NOT NULL
                )
        "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO users
                    (id, anonymous, username) SELECT 1, true, 'Guest'
                ON CONFLICT(id) DO UPDATE SET
                    anonymous = EXCLUDED.anonymous,
                    username = EXCLUDED.username
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO users
                    (id, anonymous, username) SELECT 2, false, 'Test'
                ON CONFLICT(id) DO UPDATE SET
                    anonymous = EXCLUDED.anonymous,
                    username = EXCLUDED.username
            "#,
        )
        .execute(pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
                INSERT INTO user_permissions
                    (user_id, token) SELECT 2, 'Category::View'
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i32,
    pub anonymous: bool,
    pub username: String,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id,
            anonymous: self.anonymous,
            username: self.username,
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        }
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}
impl SqlPermissionTokens {
    pub async fn get(id: i64, pool: &SqlitePool) -> Option<Vec<Self>> {
        //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM user_permissions WHERE user_id = $1;",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()?;
        Some(sql_user_perms)
    }
}

impl SqlUser {
    pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;

        Some(sqluser)
    }
}
