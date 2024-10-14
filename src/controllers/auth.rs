use crate::{
    config::EnvConfig,
    models::{
        error::AppError,
        user::{User, UserDTO},
    },
    utilities::{
        hash::{compare_password, hash_password},
        jwt::create_token,
    },
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use axum_extra::extract::CookieJar;
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
use cookie::Cookie;
use deadpool_postgres::Pool;
use serde::Deserialize;

//..........................................................
//.LLLL.........OOOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//.LLLL........OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNNN..NNNN..
//.LLLL.......OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNNN..NNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG..GGGGG.GIIII.NNNNNN.NNNN..
//.LLLL......LOOOO....OOOOOOGGGG....GGG..GIIII.NNNNNN.NNNN..
//.LLLL......LOOO......OOOOOGGG..........GIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOOO....OOOOOOGGGG.GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG....GGGGGIIII.NNNN.NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNN..NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNN..NNNNN..
//.LLLLLLLLLL....OOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//..........................................................

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn login(
    session: Session<SessionRedisPool>,
    State(pg_pool): State<Pool>,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let row = User::get_user_by_email(&login_form.email, &pg_pool).await?;

    if let Some(row) = row {
        let user: UserDTO = User::from_row(row).map_err(|error| {
            tracing::error!("Couldn't convert row to UserDTO: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
            )
        })?;

        if compare_password(&login_form.password, &user.password)? {
            session.set("id", &user.id);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", "/")
                .body(axum::body::Body::empty())
                .unwrap();

            Ok(response)
        } else {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ));
        }
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ));
    }
}

pub async fn register(
    State(pg_pool): State<Pool>,
    Form(register_form): Form<RegisterForm>,
) -> impl IntoResponse {
    if register_form.email.is_empty()
        || register_form.password.is_empty()
        || register_form.username.is_empty()
    {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Input can not be empty".to_string(),
        ));
    }

    let row = User::get_user_by_email(&register_form.email, &pg_pool).await?;

    if let Some(_) = row {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Email already exists".to_string(),
        ));
    }

    let password_hash = hash_password(&register_form.password)?;

    let insert_user = User::new(
        None,
        Some(register_form.username),
        Some(password_hash),
        Some(register_form.email),
    );

    User::insert_user(insert_user, &pg_pool).await?;

    Ok([(
        "HX-Trigger",
        r#"{"toastmessage":{"type":"success","message":"User create successfully"}}"#,
    )])
}
