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

    let project_title = "Secure Embedded Controller (EC)";
    let project_what = "Secure EC is ...";
    let project_why = "Secure EC is important because ...";

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
