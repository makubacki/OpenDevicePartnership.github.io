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
        DocLink { href: "https://opendevicepartnership.github.io/documentation/why/why.html", title: "Why ODP?" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/welcome.html", title: "Getting Started with ODP" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/tutorial/tutorial.html", title: "Tutorials" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/specs/specifications.html", title: "Specifications" },
        DocLink { href: "/governance", title: "Contributing to ODP" },
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
