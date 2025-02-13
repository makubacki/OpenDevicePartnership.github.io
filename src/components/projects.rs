use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="p-4 bg-white dark:bg-gray-800">
            <p class="pb-8 pl-24">
                <span class="font-bold text-3xl text-black dark:text-white">Current Projects</span>
            </p>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-12 pl-28">
                <a
                    class="btn-custom"
                    href="/boot-firmware"
                >
                    <p class="text-xl text-center">Boot Firmware</p>
                </a>
                <a
                    class="btn-custom"
                    href="/embedded-controller"
                >
                    <p class="text-xl text-center">Embedded Controller Firmware</p>
                </a>
                <a
                    class="btn-custom"
                    href="/windows-ec-services"
                >
                    <p class="text-xl text-center">Standard Windows-EC Services</p>
                </a>
            </div>
        </div>
    }
}