use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use dioxus::prelude::*;
use sqlx::sqlite::SqlitePool;

use crate::{
    database::{SqlPermissionTokens, SqlUser},
    models::User,
};

#[async_trait]
impl Authentication<User, i64, SqlitePool> for User {
    async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get_user(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[async_trait]
impl HasPermission<SqlitePool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
        self.permissions.contains(perm)
    }
}

pub struct Session(
    pub  axum_session_auth::AuthSession<
        crate::auth::User,
        i64,
        axum_session_auth::SessionSqlitePool,
        sqlx::SqlitePool,
    >,
);

impl std::ops::Deref for Session {
    type Target = axum_session_auth::AuthSession<
        crate::auth::User,
        i64,
        axum_session_auth::SessionSqlitePool,
        sqlx::SqlitePool,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}

#[async_trait]
impl<S: std::marker::Sync + std::marker::Send> axum::extract::FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum_session_auth::AuthSession::<
            crate::auth::User,
            i64,
            axum_session_auth::SessionSqlitePool,
            sqlx::SqlitePool,
        >::from_request_parts(parts, state)
        .await
        .map(Session)
        .map_err(|_| AuthSessionLayerNotFound)
    }
}
