use crate::components::documentation_training::{DocumentationTraining, DocLink};
use leptos::prelude::*;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="bg-white">
            <div
                class="mx-auto flex items-start justify-between w-full"
            >
                <div class="pl-[20px] md:pl-[120px] flex flex-col gap-6">
                    <h1
                        class="py-4 font-geist text-[120px] font-normal"
                        style="line-height: 133px; letter-spacing: -2.8px;"
                    >
                        "Building the Future of Trusted System Software Together"
                    </h1>
                    <p
                        class="text-[35px] font-geist"
                        style="width: 900px; height: auto; letter-spacing: -0.7px; line-height: 49px"
                    >
                        "Leading technology partners creating secure, reusable, and reliable firmware for modern client devices."
                    </p>
                </div>

                <div class="flex flex-col">
                    <button
                        class="bg-[#F1F1F1] w-[478px] h-[176px] flex items-center justify-center px-[60px]"
                        style="border: none;"
                        on:click=move |_| window().location().set_href("/getting-started").unwrap()
                    >
                        <div class="flex flex-row items-center justify-center gap-4">
                            <span class="font-geist text-[35px] font-semibold"
                                  style="line-height: 42px; letter-spacing: -0.7px;">
                                "Getting Started"
                            </span>
                            <span class="font-geist text-[35px] font-semibold"
                                  style="line-height: 42px; letter-spacing: -0.7px;">
                                r"→"
                            </span>
                        </div>
                    </button>

                    <button
                        class="bg-[#E2E2E2] w-[478px] h-[176px] flex items-center justify-center px-[60px]"
                        style="border: none;"
                    >
                        <div class="flex flex-row items-center justify-center gap-4">
                            <span class="font-geist text-[35px] font-semibold"
                                  style="line-height: 42px; letter-spacing: -0.7px;">
                                "Non-technical"
                            </span>
                            <span class="font-geist text-[35px] font-semibold"
                                  style="line-height: 42px; letter-spacing: -0.7px;">
                                r"→"
                            </span>
                        </div>
                    </button>
                </div>                
            </div>

            // Image and text box to the left of the iframe
            <div class="flex flex-row justify-end pr-[120px] pt-10">
                <div
                    class="flex flex-col items-start h-[550px] justify-end"
                    style="padding-left: 120px;"
                >
                    <div class="flex flex-col items-start w-[420px]">
                        <img
                            src="/images/video.svg"
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
                        <span class="font-geist"
                              style="
                                  font-size: 60px;
                                  font-weight: 500;
                                  line-height: 66px;
                                  letter-spacing: -1.2px;
                                  display: block;
                                  text-align: left;
                              ">
                            "Training Videos"
                        </span>
                        <div style="height: 10px;"></div>
                        <span class="font-geist"
                              style="
                                  font-size: 35px;
                                  font-weight: 400;
                                  line-height: 42px;
                                  letter-spacing: -0.7px;
                                  display: block;
                                  text-align: left;
                              ">
                            "Learn how ODP projects help build secure, modern devices"
                        </span>
                    </div>
                </div>
                <iframe
                    width="1200"
                    height="550"
                    style="border-radius: 10px;"
                    src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
                    title="YouTube Video of the Open Device Partnership"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen>
                </iframe>
            </div>

            // Two more iframes below, side by side, with gap and padding
            <div
                class="flex flex-row justify-center items-start gap-[60px] pt-10"
                style="padding-left: 120px; padding-right: 120px;"
            >
                <iframe
                    width="800"
                    height="450"
                    style="border-radius: 10px;"
                    src="https://www.youtube.com/embed/VIDEO_ID_1"
                    title="Additional Training Video 1"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen>
                </iframe>
                <iframe
                    width="800"
                    height="450"
                    style="border-radius: 10px;"
                    src="https://www.youtube.com/embed/VIDEO_ID_2"
                    title="Additional Training Video 2"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen>
                </iframe>
            </div>
        </main>
    }
}