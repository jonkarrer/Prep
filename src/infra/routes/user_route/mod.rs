mod handle_account_ui;
mod handle_delete_account;
mod handle_modify_email;
mod handle_password_reset;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use handle_account_ui::*;
use handle_delete_account::*;
use handle_modify_email::*;
use handle_password_reset::*;
use poem::{get, post, EndpointExt, Route};

pub fn use_user_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/account", get(handle_account_ui))
        .at("/account/password_reset", post(handle_password_reset))
        .at("/account/modify_email", post(handle_modify_email))
        .at("/account/delete_account", post(handle_delete_account))
        .with(AuthGuard)
}
