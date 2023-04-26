use perseus::prelude::*;
use serde::{Deserialize, Serialize};

use perseus::state::GlobalStateCreator;

use crate::some_api::SomeExternalApi;

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        .amalgamate_states_fn(amalgamate_states)
}

#[derive(Serialize, Deserialize, ReactiveState)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    pub some_var: bool,
    pub some_api: SomeExternalApi,
}

#[engine_only_fn]
pub async fn get_build_state() -> AppState {
    AppState {
        some_var: true,
        some_api: SomeExternalApi::new("".to_string()), // It's unfortunately not possible to have a different type
                                                        // for the build state and the request state, I would rather
                                                        // have left this out
                                                        // This will also only ever work as long as we don't need the api
                                                        // while building the app, so no SSR!
    }
}

#[engine_only_fn]
async fn get_request_state(_req: Request) -> AppState {
    AppState {
        some_var: true,
        some_api: SomeExternalApi::new(
            std::env::var("API_URL").expect("API_URL should be set at  runtime"),
        ),
    }
}

#[engine_only_fn]
async fn amalgamate_states(build_state: AppState, request_state: AppState) -> AppState {
    // Note here that I really manage 2 states, one at build time and the other at request time and
    // they have no relation
    AppState {
        some_var: build_state.some_var,

        some_api: request_state.some_api,
    }
}
