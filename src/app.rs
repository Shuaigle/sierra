use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::home::*;
use crate::pages::play::*;
use crate::components::layout::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sierra.css"/>

        <Title text="Sierra"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <div class="h-lvh relative w-full px-1 py-1 bg-gray-900">
            <Header/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/play" view=PlayPage/>
                </Routes>
            </main>
        </div>

        </Router>
    }
}


