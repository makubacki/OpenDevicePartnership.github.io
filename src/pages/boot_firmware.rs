use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn BootFirmware() -> impl IntoView {

    let links = vec![
        DocLink { href: "https://docs.odp.example.com/getting-started", title: "Getting Started with ODP" },
        DocLink { href: "https://docs.odp.example.com/api", title: "ODP Firmware Development Guide" },
        DocLink { href: "https://docs.odp.example.com/tutorials", title: "Embedded Controller Services Specifications" },
        DocLink { href: "https://docs.odp.example.com/faq", title: "Contributing to ODP" },
    ];

    let project_title = "Boot Firmware (Patina)";
    let project_summary = "A secure and efficient boot firmware for Windows devices";

    let project_what = "Patina is a Rust-based boot firmware platform that rethinks the full UEFI boot flow as a modular system.
It replaces the legacy UEFI dispatcher with a clear, dependency-injected structure that defines policy and behavior explicitly.
Patina supports the same boot phases as UEFI, but with modern Rust interfaces, testable components, and reproducible builds making it ideal for secure and maintainable firmware.";

    let project_why = "The world has changed - UEFI needs a reboot.
Patina replaces brittle C-based firmware with a secure, modular Rust implementation designed for today’s threats.
Embracing modern patterns and dependency injection, Patina simplifies audits, shrinks attack surfaces, and enables predictable firmware behavior.
It’s time for firmware to act like real software — and Patina makes that possible.";

    let nodes_data = r#"[{"id": 0, "name": "patina-qemu", "url": "https://github.com/OpenDevicePartnership/patina-qemu", "classification": "platform", "order": 1}, {"id": 1, "name": "patina-dxe-core-qemu", "url": "https://github.com/OpenDevicePartnership/patina-dxe-core-qemu", "classification": "driver", "order": 2}, {"id": 2, "name": "patina", "url": "https://github.com/OpenDevicePartnership/patina", "classification": "core", "order": 3}, {"id": 3, "name": "patina-readiness-tool", "url": "https://github.com/OpenDevicePartnership/patina-readiness-tool", "classification": "utilities", "order": 4}]"#; 
    let links_data = r#"[{"source": 0, "target": 1}, {"source": 0, "target": 3}, {"source": 1, "target": 2}]"#;

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
                <RepositoryGraph nodes=nodes_data links=links_data/>
                <DocumentationTraining links=links />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
