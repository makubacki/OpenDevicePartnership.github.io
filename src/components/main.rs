use leptos::prelude::*;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="background_primary">
            <div
                class="mx-auto flex items-start justify-between w-full"
            >
                <div class="pl-[20px] md:pl-[120px] flex flex-col gap-6">
                    <h1
                        class="h1 py-4"
                        style="text-align: left;"
                    >
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p
                        class="p1"
                        style="width: 900px; height: auto;"
                    >
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>

                <div class="flex flex-col">
                    <button
                        class="background_secondary w-[478px] h-[176px] flex items-center justify-center px-[60px]"
                        style="border: none;"
                        on:click=move |_| window().location().set_href("/getting-started").unwrap()
                    >
                        <div class="flex flex-row items-center justify-center gap-4">
                            <span class="h3">
                                "Getting Started"
                            </span>
                            <span class="h3">
                                r"→"
                            </span>
                        </div>
                    </button>

                    <button
                        class="background_tertiary w-[478px] h-[176px] flex items-center justify-center px-[60px]"
                        style="border: none;"
                        on:click=move |_| window().location().set_href("/projects").unwrap()
                    >
                        <div class="flex flex-row items-center justify-center gap-4">
                            <span class="h3">
                                "Projects"
                            </span>
                            <span class="h3">
                                r"→"
                            </span>
                        </div>
                    </button>
                </div>                
            </div>

            {/* Training Videos Section */}
            <div class="flex flex-col pt-10" style="padding-left: 120px; padding-right: 120px;">
                {/* Top row: icon/text + main iframe */}
                <div class="flex flex-row items-start w-full">
                    <div class="flex flex-col items-start w-[420px] mr-[60px]">
                        <picture>
                            <source srcset="/images/dark/video.svg" media="(prefers-color-scheme: dark)" />
                            <img
                                src="/images/light/video.svg"
                                alt="Video Icon"
                                style="
                                    width: 150px;
                                    height: 150px;
                                    padding: 0;
                                    object-fit: contain;
                                    display: block;
                                    margin-bottom: 16px;
                                "
                            />
                        </picture>
                        <span class="h2"
                              style="
                                  display: block;
                                  text-align: left;
                              ">
                            "Training Videos"
                        </span>
                        <div style="height: 10px;"></div>
                        <span class="p1"
                              style="
                                  display: block;
                                  text-align: left;
                              ">
                            "Learn how ODP projects help build secure, modern devices"
                        </span>
                    </div>
                    <iframe
                        width="1200"
                        height="690"
                        style="border-radius: 10px; display: block;"
                        src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
                        title="YouTube Video of the Open Device Partnership"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen>
                    </iframe>
                </div>

                {/* Bottom row: two iframes, aligned with above */}
                <div class="flex flex-row items-start gap-[60px] pt-10 w-full">
                    <iframe
                        width="810"
                        height="450"
                        style="border-radius: 10px; flex-shrink: 0;"
                        src="https://www.youtube.com/embed/VIDEO_ID_1"
                        title="Additional Training Video 1"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen>
                    </iframe>
                    <iframe
                        width="810"
                        height="450"
                        style="border-radius: 10px; flex-shrink: 0;"
                        src="https://www.youtube.com/embed/VIDEO_ID_2"
                        title="Additional Training Video 2"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen>
                    </iframe>
                </div>
            </div>
        </main>
    }
}