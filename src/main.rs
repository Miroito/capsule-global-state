use perseus::prelude::*;

use crate::global_state::get_global_state_creator;

mod capsules;
mod global_state;
mod some_api;
mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .global_state_creator(get_global_state_creator())
        .template(crate::templates::index::get_template())
        .capsule_ref(&*crate::capsules::capsule::CAPSULE)
}
