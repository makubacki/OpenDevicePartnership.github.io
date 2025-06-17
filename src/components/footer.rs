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

                <div class="flex flex-row items-center">
                    <a
                        href="https://github.com/OpenDevicePartnership/OpenDevicePartnership.github.io"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-[24px] py-[40px]"
                    >
                        <picture>
                            <source srcset="/images/dark/github.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/github.svg"
                                alt="GitHub"
                            />
                        </picture>
                    </a>
                    <a
                        href="https://opendevicepartnership.zulipchat.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-[24px] py-[40px]"
                    >
                        <picture>
                            <source srcset="/images/dark/zulip.png" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/zulip.png"
                                alt="Zulip"
                            />
                        </picture>
                    </a>

                    <a
                        href="https://discord.gg/a8cEfTDQN4"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="flex items-center justify-center px-[24px] py-[40px]"
                    >
                        <picture>
                            <source srcset="/images/dark/discord.png" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/discord.png"
                                alt="Discord"
                            />
                        </picture>
                    </a>
                </div>
            </div>
        </footer>
    }
}