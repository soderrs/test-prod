use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    auth::{register, sign_in},
    countries, me,
    me::profile,
    middlewares, profiles,
};

pub async fn app() -> Router {
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
                .layer(middleware::from_fn(
                    middlewares::authorize::authorize_middleware,
                ))
                .patch(profile::update_profile)
                .layer(middleware::from_fn(
                    middlewares::authorize::authorize_middleware,
                )),
        )
        .route(
            "/me/updatePassword",
            post(me::update_password::update_password).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/profiles/{login}",
            get(profiles::profile_by_login).layer(middleware::from_fn(
                middlewares::authorize::authorize_middleware,
            )),
        )
}
