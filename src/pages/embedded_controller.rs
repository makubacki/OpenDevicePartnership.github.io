use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::ec_repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn EmbeddedController() -> impl IntoView {

    let links = vec![
        DocLink { href: "https://docs.odp.example.com/getting-started", title: "Getting Started with ODP" },
        DocLink { href: "https://docs.odp.example.com/api", title: "ODP Firmware Development Guide" },
        DocLink { href: "https://docs.odp.example.com/tutorials", title: "Embedded Controller Services Specifications" },
        DocLink { href: "https://docs.odp.example.com/faq", title: "Contributing to ODP" },
    ];

    let project_title = "Secure Embedded Controller";
    let project_what = "The ODP Secure EC stack is a Rust-based firmware platform for modern embedded controllers, supporting both discrete and integrated ECs.

It provides modular subsystems for power sequencing, thermal policy, event routing, and more. 
Components are defined by traits, composed into devices, and managed by a shared runtime that drives platform behavior.
Built for portability and testability, it supports both std and no-std builds and integrates cleanly with real-time runtimes like Embassy.";

    let project_why = "Embedded Controllers do more than ever — yet many EC stacks are stuck in the past.
The ODP EC firmware rethinks the EC as a secure, modular orchestrator for power, telemetry, and system policy. With clearly scoped components and Rust’s safety guarantees, it helps you move faster, catch bugs earlier, and support diverse platforms with confidence.
It’s a modern foundation for building reliable, adaptable EC firmware — not just patching legacy code.";

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
                <ProjectIntroduction project_title=project_title project_what=project_what project_why=project_why />
                <RepositoryGraph />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
