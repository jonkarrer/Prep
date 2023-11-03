mod health_check;
use health_check::*;

use poem::{get, Route};
pub fn app_router() -> Route {
    Route::new().at("/health_check", get(health_check))
}
