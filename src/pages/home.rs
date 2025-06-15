use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::main::Main;
use crate::components::documentation_training::{DocLink, DocumentationTraining};

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // Documentation links for the DocumentationTraining section
    let links = vec![
        DocLink { href: "https://docs.odp.example.com/getting-started", title: "Getting Started with ODP" },
        DocLink { href: "https://docs.odp.example.com/api", title: "ODP Firmware Development Guide" },
        DocLink { href: "https://docs.odp.example.com/tutorials", title: "Embedded Controller Services Specifications" },
        DocLink { href: "https://docs.odp.example.com/faq", title: "Contributing to ODP" },
    ];

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="w-full min-h-screen overflow-x-hidden">
                <Header />
                <Main />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
