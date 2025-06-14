use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full px-[120px] py-[60px] bg-[#F1F1F1]">
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center">
                    <img
                        src="/images/odplogo.svg"
                        alt="Logo"
                        class="w-[114px] h-[40px] object-contain"
                    />
                    <p class="ml-[30px] text-[#171717] font-geist text-[20px] font-normal not-italic leading-[26px]">
                        {"© 2025 Open Device Partnership"}
                    </p>
                </div>

                <a
                        href="https://github.com/OpenDevicePartnership/OpenDevicePartnership.github.io"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-[41px] py-[40px]"
                    >
                        <img
                            src="/images/github.svg"
                            alt="GitHub"
                            />
                </a>
            </div>
        </footer>
    }
}