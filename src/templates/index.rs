use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    message: String,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        div(class = "default_div") {
            h1 { "Home Page!" }
            p { (state.message.get()) }
            a(href = "about", id = "about-link") { "About!" }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Home" }
        link(rel="stylesheet", href=".perseus/static/default.css") {}
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        message: "This is the home page.".to_string(),
    }
}

/* This page uses 'state', so different functions are used to build the template. */
pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
