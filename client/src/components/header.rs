use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Header(cx: Scope, prop: Option<String>) -> impl IntoView {
    view! {cx,
    <header class="header text-white font-semibold">
        <div class="center-y">
            <p class="header-text">"YT"</p>
        </div>
    </header>
    }
}

