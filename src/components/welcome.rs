use crate::components::intro_message::IntroMessage;
use crate::components::projects::Projects;

use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div class="pt-16 grid grid-cols-1 gap-4 custom-bg-white custom-text-gray-800">
            <div class="grid grid-cols-1 md:grid-cols-2 custom-bg-gray-100">
                <div class="flex items-center justify-center p-8 md:p-20">
                    <p class="text-2xl md:text-4xl text-center">
                        An alliance of industry-leading PC ecosystem partners promoting secure, reusable, and trusted system software for client devices
                    </p>
                </div>
                <div class="flex items-center justify-center p-8">
                    <img src="/images/laptop.jpg" class="w-full h-auto rounded-lg shadow-md" />
                </div>
            </div>
            <div class="custom-bg-white h-2"></div>
            <Projects />
            <div class="custom-bg-white h-2"></div>
            <div class="custom-bg-white">
                <IntroMessage />
            </div>
        </div>
    }
}
