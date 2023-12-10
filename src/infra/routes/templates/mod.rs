use poem::{handler, http::StatusCode, web::Html, Error, IntoResponse, Request, Result, Route};

#[handler]
pub fn handle_ingredient_input_ui(req: &Request) -> Result<impl IntoResponse> {
    match req.header("HX-Request") {
        Some(_) => Ok(Html(
            r#"
            <div>
                <input
                    type="number"
                    name="amount" 
                    placeholder="amount"
                    oninput="handleAmountInput(event)"
                />
                <input
                    type="text"
                    name="unit"
                    placeholder="unit" 
                    oninput="handleUnitInput(event)"
                />
                <input 
                    type="text"
                    name="ingredient"
                    placeholder="ingredient"
                    oninput="handleIngredientInput(event)"
                />
            </div>
            "#,
        )),
        None => Err(Error::from_status(StatusCode::NOT_FOUND)),
    }
}

pub fn use_template_routes() -> Route {
    Route::new().at("/ingredient-input", handle_ingredient_input_ui)
}
