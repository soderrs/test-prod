use crate::{
    auth::{register, sign_in},
    countries,
    me::{self, profile},
    middlewares::{self, authorize::AppState},
    profiles,
};
use axum::{
    middleware::{self},
    routing::{get, post},
    Router,
};

pub async fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(crate::root))
        .route("/ping", get(crate::ping))
        .route("/countries", get(countries::countries))
        .route("/countries/{alpha2}", get(countries::country_by_id))
        .route("/auth/register", post(register::register_user))
        .route("/auth/sign-in", post(sign_in::sign_in))
        .route(
            "/me/profile",
            get(profile::get_profile)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::authorize::authorize_middleware,
                ))
                // .with_state(state.clone())
                // .layer(middleware::from_fn(middlewares::authorize::check_token))
                .patch(profile::update_profile)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::authorize::authorize_middleware,
                )), // .with_state(state.clone()),
        )
        .route(
            "/me/updatePassword",
            post(me::update_password::update_password)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::authorize::authorize_middleware,
                ))
                .with_state(state.clone()),
        )
        .route(
            "/profiles/{login}",
            get(profiles::profile_by_login).layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::authorize::authorize_middleware,
            )),
        )
}
