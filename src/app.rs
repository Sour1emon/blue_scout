use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::{FallbackPage, HomePage, SettingsPage, ViewDataPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="dark">
            <head>
                <script>{include_str!("../embed/theme.js")}</script>
                <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                // injects a stylesheet into the document <head>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <Stylesheet id="leptos" href="/pkg/blue_scout.css" />
                <link
                    // href="https://cdn.jsdelivr.net/npm/daisyui@5"
                    href="/tailwind.css"
                    rel="stylesheet"
                    type="text/css"
                />
                <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="4682's Scouting Site" />

        <Router>
            <main>
                <Routes fallback=FallbackPage>
                    <Route path=StaticSegment("/") view=HomePage />
                    <Route path=StaticSegment("/view-data") view=ViewDataPage />
                    <Route path=StaticSegment("/settings") view=SettingsPage />
                </Routes>
            </main>
        </Router>
    }
}
