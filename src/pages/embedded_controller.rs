use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::project_introduction::ProjectIntroduction;
use crate::components::documentation_training::{DocLink, DocumentationTraining};
use crate::components::repo_view::RepositoryGraph;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn EmbeddedController() -> impl IntoView {

    let links = vec![
        DocLink { href: "https://opendevicepartnership.github.io/documentation/why/why.html", title: "Why ODP?" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/welcome.html", title: "Getting Started with ODP" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/intro/tutorial/tutorial.html", title: "Tutorials" },
        DocLink { href: "https://opendevicepartnership.github.io/documentation/specs/specifications.html", title: "Specifications" },
        DocLink { href: "/governance", title: "Contributing to ODP" },
    ];

    let project_title = "Secure Embedded Controller";
    let project_summary = "A hardened firmware platform for modern embedded controllers";
    let project_what = "The ODP Secure EC stack is a Rust-based firmware platform for modern embedded controllers, supporting both discrete and integrated ECs.

It provides modular subsystems for power sequencing, thermal policy, event routing, and more. 
Components are defined by traits, composed into devices, and managed by a shared runtime that drives platform behavior.
Built for portability and testability, it supports both std and no-std builds and integrates cleanly with real-time runtimes like Embassy.";

    let project_why = "Embedded Controllers do more than ever — yet many EC stacks are stuck in the past.
The ODP EC firmware rethinks the EC as a secure, modular orchestrator for power, telemetry, and system policy. With clearly scoped components and Rust’s safety guarantees, it helps you move faster, catch bugs earlier, and support diverse platforms with confidence.
It’s a modern foundation for building reliable, adaptable EC firmware — not just patching legacy code.";

    let nodes_data = r#"[{"id": 0, "name": "soc-embedded-controller", "url": "https://github.com/OpenDevicePartnership/embedded-power-sequence", "classification": "platform", "order": 1}, {"id": 1, "name": "embedded-services", "url": "https://github.com/OpenDevicePartnership/embedded-services", "classification": "services", "order": 2}, {"id": 2, "name": "embedded-usb-pd", "url": "https://github.com/OpenDevicePartnership/embedded-usb-pd", "classification": "subsystem", "order": 3}, {"id": 3, "name": "embedded-power-sequence", "url": "https://github.com/OpenDevicePartnership/embedded-power-sequence", "classification": "subsystem", "order": 3}, {"id": 4, "name": "embedded-batteries", "url": "https://github.com/OpenDevicePartnership/embedded-batteries", "classification": "subsystem", "order": 3}, {"id": 5, "name": "embedded-cfu", "url": "https://github.com/OpenDevicePartnership/embedded-cfu", "classification": "subsystem", "order": 3}, {"id": 6, "name": "embedded-sensors", "url": "https://github.com/OpenDevicePartnership/embedded-sensors", "classification": "subsystem", "order": 3}, {"id": 7, "name": "embedded-fans", "url": "https://github.com/OpenDevicePartnership/embedded-fans", "classification": "subsystem", "order": 3}, {"id": 8, "name": "embedded-keyboard-rs ", "url": "https://github.com/OpenDevicePartnership/embedded-keyboard-rs", "classification": "subsystem", "order": 3}, {"id": 9, "name": "tps6699x", "url": "https://github.com/OpenDevicePartnership/tps6699x", "classification": "drivers", "order": 4}, {"id": 10, "name": "tmp108", "url": "https://github.com/OpenDevicePartnership/tmp108", "classification": "drivers", "order": 4}, {"id": 11, "name": "is31fl3743b", "url": "https://github.com/OpenDevicePartnership/is31fl3743b", "classification": "drivers", "order": 4}, {"id": 12, "name": "pcal6416a", "url": "https://github.com/OpenDevicePartnership/pcal6416a", "classification": "drivers", "order": 4}, {"id": 13, "name": "bq25773", "url": "https://github.com/OpenDevicePartnership/bq25773", "classification": "drivers", "order": 4}, {"id": 14, "name": "bq40z50", "url": "https://github.com/OpenDevicePartnership/bq40z50", "classification": "drivers", "order": 4}, {"id": 15, "name": "bq25770g", "url": "https://github.com/OpenDevicePartnership/bq25770g", "classification": "drivers", "order": 4}, {"id": 16, "name": "embedded-hal", "url": "https://github.com/rust-embedded/embedded-hal", "classification": "embedded HAL", "order": 5}, {"id": 17, "name": "embassy-imxrt", "url": "https://github.com/OpenDevicePartnership/embassy-imxrt", "classification": "MCU core HALs", "order": 6}, {"id": 18, "name": "mimxrt600-fcb", "url": "https://github.com/OpenDevicePartnership/mimxrt600-fcb", "classification": "MCU PACs", "order": 7}, {"id": 19, "name": "mimxrt633s-pac ", "url": "https://github.com/OpenDevicePartnership/mimxrt633s-pac", "classification": "MCU PACs", "order": 7}]"#;
    let links_data = r#"[{"source": 0, "target": 17}, {"source": 0, "target": 1}, {"source": 1, "target": 2}, {"source": 1, "target": 3}, {"source": 1, "target": 4}, {"source": 1, "target": 5}, {"source": 1, "target": 6}, {"source": 1, "target": 7}, {"source": 1, "target": 8}, {"source": 2, "target": 9}, {"source": 4, "target": 14}, {"source": 4, "target": 15}, {"source": 4, "target": 16}, {"source": 6, "target": 10}, {"source": 12, "target": 16}, {"source": 16, "target": 18}, {"source": 17, "target": 19}]"#;

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
