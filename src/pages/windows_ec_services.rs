use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::documentation_training::{DocLink, DocumentationTraining};

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn WindowsEcServices() -> impl IntoView {

    let links = vec![
        DocLink { href: "https://docs.odp.example.com/getting-started", title: "Getting Started with ODP" },
        DocLink { href: "https://docs.odp.example.com/api", title: "ODP Firmware Development Guide" },
        DocLink { href: "https://docs.odp.example.com/tutorials", title: "Embedded Controller Services Specifications" },
        DocLink { href: "https://docs.odp.example.com/faq", title: "Contributing to ODP" },
    ];

    let project_title = "Unified Embedded Controller Interface";
    let project_summary = "";
    let project_what = "The Unified Windows EC Service interface defines runtime coordination between firmware components using async message-passing. 
    Each service manages a domain — like power, battery, or host communication — and exposes a structured protocol. 
    Components register with services and receive commands for events, capabilities, and state changes. 
    This model enables loosely coupled subsystems, observability, and test injection — without sacrificing platform coherence.";
    let project_why = "Without a common interface, EC firmware becomes tangled and brittle. 
    Unified EC Services -- designed for Windows Platforms -- bring structure and predictability by defining how components interact at runtime. 
    With async protocols and policy-aware lifecycles, they support clean separation of concerns and cross-subsystem coordination. 
    Whether debugging power flows or integrating a new device, these services provide the glue, guardrails, and visibility you need.";


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
                <ProjectIntroduction project_title=project_title project_summary=project_summary project_what=project_what project_why=project_why />
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}