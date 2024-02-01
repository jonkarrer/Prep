use crate::{
    app::interface::{Database, UserRepository},
    domain::entity::PasswordResetToken,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Html},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[poem::handler]
pub async fn handle_account_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    let reset_token = PasswordResetToken::new();
    repo.insert_password_reset_token(&reset_token, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    let tera = Tera::new("src/web/pages/user/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch all recipes
    let user = repo
        .get_user_by_id(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert::<str, &str>("user_email", &user.email);
    context.insert::<str, &str>("user_id", &user.user_id);
    context.insert::<str, &str>("csrf_token", &session.csrf_token);
    context.insert::<str, &str>("reset_token", &reset_token.password_reset_token);

    let rendered_html = tera
        .render("account.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Serve template
    Ok(Html(rendered_html))
}
