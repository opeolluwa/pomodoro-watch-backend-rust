use crate::handlers::{auth_handlers, pomodoro::PomodoroHandlers, user::UserInformationHandler};

use crate::pkg::state::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub async fn auth_routes() -> Router<AppState> {
    let password_reset_routes = Router::new()
        .route("/", post(auth_handlers::password_reset))
        .route(
            "/confirm-token",
            post(auth_handlers::confirm_password_reset_token),
        )
        .route("/new-password", post(auth_handlers::set_new_password));

    Router::new()
        .route("/sign-up", post(auth_handlers::sign_up))
        .route("/verify", post(auth_handlers::verify_email))
        .route(
            "/verify/new-token",
            get(auth_handlers::request_new_verification_token),
        )
        .nest("/password-reset", password_reset_routes)
        .route("/login", post(auth_handlers::login))
        .route("/refresh-token", get(auth_handlers::refresh_token))
        .route("/logout", post(auth_handlers::logout))
}

// the user profile routes
pub async fn user_information_routes() -> Router<AppState> {
    Router::new().route(
        "/profile",
        get(UserInformationHandler::get_profile).put(UserInformationHandler::update_profile),
    )
}

// the pomodoro endpoint
pub async fn pomodoro_routes() -> Router<AppState> {
    Router::new()
        .route("/:user_id", get(PomodoroHandlers::get_records))
        .route("/:user_id/new", post(PomodoroHandlers::save_record))
        .route("/:user_id/:record_id", put(PomodoroHandlers::update_record))
        .route(
            "/:user_id/:record_id",
            delete(PomodoroHandlers::delete_record),
        )
}
