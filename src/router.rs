use crate::handlers::{
    auth_handlers::AuthHandlers, pomodoro::PomodoroHandlers, user::UserInformationHandler,
};
use crate::pkg;
use crate::pkg::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(AuthHandlers::sign_up))
        .route("/verify", post(AuthHandlers::verify_email))
        .route(
            "/verify/new-token",
            get(AuthHandlers::request_new_verification_token),
        )
        .nest("/password-reset", password_reset_routes())
        .route("/login", post(AuthHandlers::login))
        .route("/refresh-token", get(AuthHandlers::refresh_token))
        .route("/logout", post(AuthHandlers::logout))
}

fn password_reset_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(AuthHandlers::password_reset))
        .route(
            "/confirm-token",
            post(AuthHandlers::confirm_password_reset_token),
        )
        .route("/new-password", post(AuthHandlers::set_new_password))
}

// the user profile routes
pub fn user_information_routes() -> Router<AppState> {
    Router::new().route(
        "/profile",
        get(UserInformationHandler::get_profile).put(UserInformationHandler::update_profile),
    )
}

// the pomodoro endpoint
pub fn pomodoro_routes() -> Router<AppState> {
    Router::new()
        .route("/:user_id", get(PomodoroHandlers::get_records))
        .route("/:user_id/new", post(PomodoroHandlers::save_record))
        .route("/:user_id/:record_id", put(PomodoroHandlers::update_record) )
        .route("/:user_id/:record_id", delete(PomodoroHandlers::delete_record))
}
