use crate::components::footer::Footer;
use crate::components::intro_message::IntroMessage;
use crate::components::projects::Projects;

use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div class="pt-16 grid grid-cols-1 gap-4 bg-white">
            <div class="grid grid-cols-2 bg-gray-100">
                <div class="place-content-center ...">
                    <p class="text-4xl place-self-center p-20">
                        "An alliance of industry-leading PC ecosystem partners promoting secure, reusable, and trusted system software for client devices"
                    </p>
                </div>
                <div>
                    <img src="/images/laptop.jpg" class="p-9" />
                </div>
            </div>
            <div class="bg-white h-2"></div>
            <Projects />
            <div class="bg-white h-2"></div>
            <div class="bg-white">
                <IntroMessage />
            </div>
            <div>
                <Footer />
            </div>
        </div>
    }
}
