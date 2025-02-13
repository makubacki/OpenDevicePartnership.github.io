use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::about::About;
use crate::pages::boot_firmware::BootFirmware;
use crate::pages::uefi::Uefi;
use crate::pages::contact::Contact;
use crate::pages::documentation::Documentation;
use crate::pages::embedded_controller::EmbeddedController;
use crate::pages::home::Home;
use crate::pages::windows_ec_services::WindowsEcServices;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Stylesheet id="leptos" href="/style/output.css" />

        // sets the document title
        <Title text="Open Device Partnership" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/about") view=About />
                <Route path=path!("/boot-firmware") view=BootFirmware />
                <Route path=path!("/uefi") view=Uefi />
                <Route path=path!("/contact") view=Contact />
                <Route path=path!("/documentation") view=Documentation />
                <Route path=path!("/embedded-controller") view=EmbeddedController />
                <Route path=path!("/windows-ec-services") view=WindowsEcServices />
            </Routes>
        </Router>
    }
}
