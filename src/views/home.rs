use dioxus::prelude::*;

use crate::components::{Button, Input, InputType};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        div {
            Input { oninput: move |event: Event<FormData>| { email.set(event.value()) } }
            Input {
                r#type: InputType::Password,
                oninput: move |event: Event<FormData>| { password.set(event.value()) },
            }
            Button {
                onclick: move |_| {
                    async move {
                        println!("{:?}", login().await);
                    }
                },
                "Login"
            }
        }
    }
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    Ok("Hello World".to_string())
    // let session: crate::auth::Session = extract().await?;
    // if let Some(user) = &session.current_user {
    //     Ok(user.username.to_string())
    // } else {
    //     Err(ServerFnError::new("User not found"))
    // }
}

#[server(Login)]
pub async fn login() -> Result<(), ServerFnError> {
    // let auth: crate::auth::Session = extract().await?;
    // auth.login_user(2);
    Ok(())
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    // let method: axum::http::Method = extract().await?;
    // let auth: crate::auth::Session = extract().await?;
    // let current_user = auth.current_user.clone().unwrap_or_default();
    //
    // // lets check permissions only and not worry about if they are anon or not
    // if !axum_session_auth::Auth::<crate::models::User, i64, sqlx::SqlitePool>::build(
    //     [axum::http::Method::POST],
    //     false,
    // )
    // .requires(axum_session_auth::Rights::any([
    //     axum_session_auth::Rights::permission("Category::View"),
    //     axum_session_auth::Rights::permission("Admin::View"),
    // ]))
    // .validate(&current_user, &method, None)
    // .await
    // {
    //     return Ok(format!(
    //         "User {}, Does not have permissions needed to view this page please login",
    //         current_user.username
    //     ));
    // }
    //
    Ok(format!(
        "User has Permissions needed. Here are the Users permissions {}",
        2,
    ))
}
