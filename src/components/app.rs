use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet};

use crate::components::search::SearchBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <div>
            <h1>
                "Hello World1"
            </h1>

            <SearchBar />
        </div>
    }
}
