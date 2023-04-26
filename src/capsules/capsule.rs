use lazy_static::lazy_static;
use perseus::prelude::*;
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

lazy_static! {
    pub static ref CAPSULE: Capsule<PerseusNodeType, ()> = get_capsule();
}

fn capsule<G: Html>(cx: Scope, _: ()) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    view! { cx,
        "Global state is "
        (*global_state.some_var.get())
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, ()> {
    Capsule::build(Template::build("greeting"))
        .empty_fallback()
        .view(capsule)
        .build()
}
