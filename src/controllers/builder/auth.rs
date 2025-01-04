use crate::{
    models::{error::AppError, user::User},
    utilities::hash::{compare_password, hash_password},
    views::builder::auth::{render_login_page, render_register_page},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Form,
};
use axum_csrf::CsrfToken;
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
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
) -> Result<impl IntoResponse, AppError> {
    let row = User::get_user_by_email(&login_form.email, &pg_pool, vec!["id", "password"]).await?;

    if let Some(row) = row {
        let user = User::try_from(row);

        let user_password = user.password.ok_or_else(|| {
            tracing::error!("No password column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let user_id = user.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        if compare_password(&login_form.password, &user_password)? {
            session.set("id", &user_id);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("HX-Location", "/builder/create-website")
                .body(axum::body::Body::empty())
                .unwrap();

            Ok(response)
        } else {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid username or password",
            ));
        }
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password",
        ));
    }
}

pub async fn get_login_page(token: CsrfToken) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    (
        token,
        Html(render_login_page(authenticity_token).into_string()),
    )
}

//.............................................................................................
//.RRRRRRRRR....EEEEEEEEEE.....GGGGGG.....III....SSSSSS....TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRR....
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGG...III..SSSSSSSSS...TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGGG..III..SSSSSSSSSS..TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRR.....RRR..EEE.........GGGG....GGGG..III..SSS...SSSS......TTT....EEE.........RRR.....RRR..
//.RRR.....RRR..EEE.........GGG......GG...III..SSSS............TTT....EEE.........RRR.....RRR..
//.RRRRRRRRRRR..EEEEEEEEEE.EGGG...........III..SSSSSSS.........TTT....EEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRR...EEEEEEEEEE.EGGG...GGGGGG..III...SSSSSSSS.......TTT....EEEEEEEEEE..RRRRRRRRRR...
//.RRRRRRRR.....EEEEEEEEEE.EGGG...GGGGGG..III.....SSSSSSS......TTT....EEEEEEEEEE..RRRRRRRR.....
//.RRR..RRRR....EEE.........GGG...GGGGGG..III.........SSSS.....TTT....EEE.........RRR..RRRR....
//.RRR...RRRR...EEE.........GGGG.....GGG..III.ISSS....SSSS.....TTT....EEE.........RRR...RRRR...
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGGG..III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGG...III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR.....RRRR.EEEEEEEEEEE....GGGGGG.....III....SSSSSS........TTT....EEEEEEEEEEE.RRR.....RRR..
//.............................................................................................

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
            "Input can not be empty",
        ));
    }

    let row = User::get_user_by_email(&register_form.email, &pg_pool, vec!["id"]).await?;

    if let Some(_) = row {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Email already exists",
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

pub async fn get_register_page(token: CsrfToken) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token().unwrap_or("".to_owned());

    (
        token,
        Html(render_register_page(authenticity_token).into_string()),
    )
}

//..............................................................................
//.LLL...........OOOOOO........GGGGGG........OOOOOO.....UUU....UUUU..TTTTTTTTT..
//.LLL.........OOOOOOOOOO....GGGGGGGGGG....OOOOOOOOOO...UUU....UUUU..TTTTTTTTT..
//.LLL........OOOOOOOOOOOO...GGGGGGGGGGG..OOOOOOOOOOOO..UUU....UUUU..TTTTTTTTT..
//.LLL........OOOO....OOOO..GGGG....GGGG..OOOO....OOOO..UUU....UUUU......TTT....
//.LLL........OOO......OOO..GGG......GG...OOO......OOO..UUU....UUUU......TTT....
//.LLL.......LOOO......OOOOOGGG..........GOOO......OOOO.UUU....UUUU......TTT....
//.LLL.......LOOO......OOOOOGGG...GGGGGG.GOOO......OOOO.UUU....UUUU......TTT....
//.LLL.......LOOO......OOOOOGGG...GGGGGG.GOOO......OOOO.UUU....UUUU......TTT....
//.LLL........OOO......OOO..GGG...GGGGGG..OOO......OOO..UUU....UUUU......TTT....
//.LLL........OOOO....OOOO..GGGG.....GGG..OOOO....OOOO..UUUU...UUUU......TTT....
//.LLLLLLLLLL.OOOOOOOOOOOO...GGGGGGGGGGG..OOOOOOOOOOOO..UUUUUUUUUUU......TTT....
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG....OOOOOOOOOO....UUUUUUUUU.......TTT....
//.LLLLLLLLLL....OOOOOO........GGGGGG........OOOOOO.......UUUUUUU........TTT....
//..............................................................................

pub async fn logout(session: Session<SessionRedisPool>) -> Result<impl IntoResponse, AppError> {
    session.destroy();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("HX-Location", "/builder/auth/login")
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}
