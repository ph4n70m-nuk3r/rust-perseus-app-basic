use perseus::prelude::*;
use sycamore::prelude::*;

fn about<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class = "default_div") {
            h1 { "About" }
            p { "This is the about page." }
            a(href = "", id = "index-link") { "Home!" }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About!" }
        link(rel="stylesheet", href=".perseus/static/default.css") {}
    }
}

/* This page doesn't use any 'state', so we can use the basic builder methods. */
pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about")
        .view(about)
        .head(head)
        .build()
}
