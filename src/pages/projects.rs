use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::projects_component::ProjectsComponent;
use crate::components::documentation_training::{DocLink, DocumentationTraining};


use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
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
                // Render a list of errors as strings - good for development purposes
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

            <div class="w-full min-h-screen" style="overflow-x: auto;">
                <Header />
                <ProjectsComponent />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
