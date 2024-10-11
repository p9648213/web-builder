use crate::{
    config::EnvConfig,
    models::{
        error::AppError,
        user::{User, UserDTO},
    },
    utilities::{hash::hash_password, jwt::create_token},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use axum_extra::extract::CookieJar;
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
    State(pool): State<Pool>,
    State(config): State<EnvConfig>,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let row = User::get_user_by_email(&login_form.email, &pool).await?;

    if let Some(row) = row {
        let user: UserDTO = User::from_row(row).map_err(|error| {
            tracing::error!("Couldn't convert row to UserDTO: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
            )
        })?;

        if user.password == login_form.password {
            let token = create_token(&config.jwt_secret, &user.email, &user.role, user.id, 60)?;

            let token_cookie: Cookie = Cookie::build(("token", token))
                .same_site(cookie::SameSite::Strict)
                .http_only(true)
                .path("/")
                .max_age(cookie::time::Duration::minutes(60))
                .into();

            let cookies = CookieJar::new().add(token_cookie);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", "/")
                .body(axum::body::Body::empty())
                .unwrap();

            Ok((cookies, response))
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
    State(pool): State<Pool>,
    Form(register_form): Form<RegisterForm>,
) -> impl IntoResponse {
    let row = User::get_user_by_email(&register_form.email, &pool).await?;

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

    User::insert_user(insert_user, &pool).await?;

    Ok([(
        "HX-Trigger",
        r#"{"toastmessage":{"type":"success","message":"User create successfully"}}"#,
    )])
}
