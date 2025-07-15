use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::landing_page::LandingPage;
use crate::components::documentation_training::DocLink;


use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn GettingStarted() -> impl IntoView {
    // Documentation links for the DocumentationTraining section
    let _links = vec![
        DocLink { href: "https://opendevicepartnership.github.io/documentation/why/why.html", title: "Why ODP?" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/welcome.html", title: "Getting Started with ODP" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/tutorial/tutorial.html", title: "Tutorials" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/specs/specifications.html", title: "Specifications" },
        DocLink { href: "/community", title: "Contributing to ODP" },
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
                <LandingPage />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
