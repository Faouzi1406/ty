//use crate::components::header;
use crate::components::header::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/home" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone)]
struct SomeData {
    name: String,
    lastname: String,
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let wow = SomeData {
        name: "This is very nice".to_string(),
        lastname: "goodbye".to_string(),
    };

    let (value, setValue) = create_signal(cx, wow.clone());

    view! { cx,
        <Header prop=None />
    }
}
