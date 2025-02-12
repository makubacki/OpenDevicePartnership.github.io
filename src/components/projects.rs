use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div>
            <p class="pb-8 pl-28">
                <span class="font-bold text-2xl">"Current Projects"</span>
            </p>
            <div class="grid grid-cols-3 bg-white gap-20 h-24 pl-32 pr-32">
                <a
                    class="place-content-center bg-blue-200 rounded-md ring-4 ring-blue-300"
                    href="/boot-firmware"
                >
                    <p class="text-xl text-center font-bold p-4">"Boot Firmware"</p>
                </a>
                <a
                    class="place-content-center bg-blue-200 rounded-md ring-4 ring-blue-300"
                    href="/embedded-controller"
                >
                    <p class="text-xl text-center font-bold p-4">"Embedded Controller Firmware"</p>
                </a>
                <a
                    class="place-content-center bg-blue-200 rounded-md ring-4 ring-blue-300"
                    href="/windows-ec-services"
                >
                    <p class="text-xl text-center font-bold p-4">"Standard Windows-EC Services"</p>
                </a>
            </div>
        </div>
    }
}
