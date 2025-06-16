use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full px-[120px] py-[60px] background_secondary">
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center">
                    <picture>
                        <source srcset="/images/dark/odplogo.svg" media="(prefers-color-scheme: dark)" />
                        <img
                            src="/images/light/odplogo.svg"
                            alt="Logo"
                            class="w-[114px] h-[40px] object-contain"
                        />
                    </picture>
                    <p class="ml-[30px] leading-[26px] p">
                        {"© 2025 Open Device Partnership"}
                    </p>
                </div>

                <a
                    href="https://github.com/OpenDevicePartnership/OpenDevicePartnership.github.io"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center px-[41px] py-[40px]"
                >
                    <picture>
                        <source srcset="/images/dark/github.svg" media="(prefers-color-scheme: dark)" />
                        <img
                            src="/images/light/github.svg"
                            alt="GitHub"
                        />
                    </picture>
                </a>
            </div>
        </footer>
    }
}