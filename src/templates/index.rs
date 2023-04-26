use perseus::prelude::*;
use sycamore::prelude::*;

use crate::capsules::capsule::CAPSULE;
use crate::global_state::AppStateRx;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let some_api = global_state.some_api.get();
    let endpoint: &Signal<Option<String>> = create_signal(cx, None);

    #[cfg(client)]
    spawn_local_scoped(cx, async move {
        endpoint.set(Some(some_api.get_endpoint().await));
    });

    view! { cx,
        "Runtime set API endpoint: "
        (if endpoint.get().is_some() {
            view! {cx, (endpoint.get().as_ref().clone().unwrap()) }
        } else {
            view! {cx, "Loading" }
        })
        br() {}
        (CAPSULE.delayed_widget(cx,"",())) // This works properly but is unecessarily delayed
        // (CAPSULE.widget(cx,"",())) // This makes the build panic when using global state
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Capsule global state MRE" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}

