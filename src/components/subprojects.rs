use leptos::prelude::*;

#[component]
pub fn SubProjects() -> impl IntoView {
    view! {
        <div>
            <div class="grid grid-cols-4 bg-white gap-10 h-24 pl-24 pr-24">
                <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/boot-firmware">
                    <p class="text-center font-bold p-4">Boot Firmware</p>
                </a>
                <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/embedded-controller">
                    <p class="text-center font-bold p-4">Embedded Controller Firmware</p>
                </a>
                <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/windows-ec-services">
                    <p class="text-center font-bold p-4">Windows-EC Services</p>
                </a>
                <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/mptf">
                    <p class="text-center font-bold p-4">Modern Power&Thermal Framework</p>
                </a>
            </div>
        </div>
    }
}