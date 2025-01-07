use crate::{
    auth, countries,
    friends::{self, list::list_friends},
    me,
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
        .route("/auth/register", post(auth::register::register_user))
        .route("/auth/sign-in", post(auth::sign_in::sign_in))
        .route(
            "/me/profile",
            get(me::profile::get_profile)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::authorize::authorize_middleware,
                ))
                .patch(me::profile::update_profile)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    middlewares::authorize::authorize_middleware,
                )),
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
        .route(
            "/friends",
            get(list_friends).layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/friends/add",
            post(friends::add::add_friend).layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::authorize::authorize_middleware,
            )),
        )
        .route(
            "/friends/remove",
            post(friends::remove::remove_friend).layer(middleware::from_fn_with_state(
                state.clone(),
                middlewares::authorize::authorize_middleware,
            )),
        )
}
