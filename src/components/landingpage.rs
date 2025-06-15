use leptos::prelude::*;
use crate::components::image_button::ImageButton;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div
            style="
                width: auto;
                height: auto;
                padding: 120px;
            "
        >
            <h1
                style="
                    text-align: center;
                    font-family: Geist, sans-serif;
                    font-size: 110px;
                    font-style: normal;
                    font-weight: 400;
                    line-height: 95%;
                    letter-spacing: -2.8px;
                "
            >
                "An Open Collaboration for Secure, Modern Devices"
            </h1>

            <div class="flex flex-row justify-center items-center mt-[80px] gap-[80px]">
                <div
                    style="
                        width: 840px;
                        height: 570px;
                        background: #E5E7EB;
                        border-radius: 24px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-family: Geist, sans-serif;
                        font-size: 40px;
                        color: #888;
                        text-align: center;
                    "
                >
                    Photo Placeholder
                </div>
                <div
                    style="
                        width: 650px;
                        font-family: Geist, sans-serif;
                        font-size: 35px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 140%;
                        letter-spacing: -0.7px;
                        color: #000;
                        text-align: left;
                    "
                >
                    {"The Open Device Partnership (ODP) is a global initiative to make it easier for developers and device makers to build secure, efficient, and reliable client devices for cross-platform needs and certified environments."}
                    <br /><br />
                    {"By combining open standards with collaborative development practices, ODP reduces complexity, improves security, and accelerates innovation across diverse silicon and hardware platforms."}
                </div>
            </div>
        </div>

        // Value Proposition Section
        <section
            style="
                padding: 80px 120px;
                background: #F1F1F1;
            "
        >
            <div>
                <h2
                    style="
                        font-family: Geist, sans-serif;
                        font-size: 60px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 110%;
                        letter-spacing: -1.2px;
                        margin-bottom: 56px;
                        text-align: left;
                    "
                >
                    {"Value Proposition"}
                </h2>
                <div class="flex flex-row gap-[60px]">
                    {/* Column 1 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <img
                            src="/images/lock.svg"
                            alt="Security Icon"
                            style="
                                display: flex;
                                width: 150px;
                                height: 150px;
                                padding: 12.5px 18.75px;
                                justify-content: center;
                                align-items: center;
                                aspect-ratio: 1/1;
                                margin-bottom: 24px;
                            "
                        />
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 600;
                                line-height: 120%;
                                letter-spacing: -0.7px;
                                margin-bottom: 12px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Enhanced Security"}
                        </span>
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 25px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 130%;
                                letter-spacing: -0.25px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Security threats continue to evolve. ODP takes a proactive approach: reducing attack surfaces, using secure hardware features, leveraging the memory-safe Rust language, and designing every component with security-first principles."}
                        </span>
                    </div>
                    {/* Column 2 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <img
                            src="/images/checkcircle.svg"
                            alt="Interoperability Icon"
                            style="
                                display: flex;
                                width: 150px;
                                height: 150px;
                                padding: 12.5px 18.75px;
                                justify-content: center;
                                align-items: center;
                                aspect-ratio: 1/1;
                                margin-bottom: 24px;
                            "
                        />
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 600;
                                line-height: 120%;
                                letter-spacing: -0.7px;
                                margin-bottom: 12px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Standardization"}
                        </span>
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 25px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 130%;
                                letter-spacing: -0.25px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {r#"Many device firmware components are "invisible plumbing" — necessary but costly to build and maintain. ODP's standards-based approach simplifies this infrastructure, maximizing reuse across devices, architectures (x86 and ARM), and generations."#}
                        </span>
                    </div>
                    {/* Column 3 */}
                    <div class="flex flex-col items-start" style="width: 400px;">
                        <img
                            src="/images/fastforward.svg"
                            alt="Innovation Icon"
                            style="
                                display: flex;
                                width: 150px;
                                height: 150px;
                                padding: 12.5px 18.75px;
                                justify-content: center;
                                align-items: center;
                                aspect-ratio: 1/1;
                                margin-bottom: 24px;
                            "
                        />
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 600;
                                line-height: 120%;
                                letter-spacing: -0.7px;
                                margin-bottom: 12px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Accelerated Development"}
                        </span>
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 25px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 130%;
                                letter-spacing: -0.25px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Open collaboration means sharing solutions, reducing duplicated work, and speeding up the development of high-quality products."}
                        </span>
                    </div>
                </div>
            </div>
        </section>

        // ODP Projects Section
        <section
            style="
                padding: 120px 0;
                background: white;
            "
        >
            <div style="max-width: 960px; margin-left: 120px;">
                <h2
                    style="
                        color: #171717;
                        font-family: Geist, sans-serif;
                        font-size: 60px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 110%;
                        letter-spacing: -1.2px;
                        text-align: left;
                        margin-bottom: 32px;
                    "
                >
                    {"ODP Projects"}
                </h2>
                <p
                    style="
                        color: #171717;
                        font-family: Geist, sans-serif;
                        font-size: 25px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 130%;
                        letter-spacing: -0.25px;
                        text-align: left;
                        margin-bottom: 0;
                        max-width: 100%;
                    "
                >
                    {"While ODP’s first projects focus on boot firmware and embedded controller software, the partnership welcomes new ideas aligned with our core goals: security, efficiency, and broad reusability."}
                    <br /><br />
                    {"More information about project governance and contribution opportunities will be shared soon. In the meantime, questions and ideas are welcome — contact the ODP administrators!"}
                </p>
            </div>
        </section>

        // Boot Firmware Buttons Section
        <section
            style="
                padding: 0 120px 120px 120px;
                background: white;
            "
        >
            <div class="flex flex-row gap-[60px] justify-start">
                <ImageButton href="/boot-firmware" img_src="/images/patina.svg" alt="Boot Firmware" />
                <ImageButton href="/embedded-controller" img_src="/images/ec.svg" alt="Embedded Controller" />
                <ImageButton href="/windows-ec-services" img_src="/images/ECServices.svg" alt="EC Services" />
            </div>
        </section>

        // Training Videos Section (left aligned with bottom iframe)
        <section
            style="
                padding: 80px 120px 0 120px;
                background: #fff;
            "
        >
            <div style="width: 100%; max-width: 920px;">
                <div class="flex flex-row items-start gap-[40px]">
                    <div class="flex flex-col items-start" style="width: 320px;">
                        <img
                            src="/images/video.svg"
                            alt="Video Icon"
                            style="
                                width: 150px;
                                height: 150px;
                                object-fit: contain;
                                display: block;
                                margin-bottom: 16px;
                            "
                        />
                        <span
                            style="
                                font-family: Geist, sans-serif;
                                font-size: 60px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 110%;
                                letter-spacing: -1.2px;
                                color: #171717;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Training Videos"}
                        </span>
                        <span
                            style="
                                font-family: Geist, sans-serif;
                                font-size: 25px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 130%;
                                letter-spacing: -0.25px;
                                color: #171717;
                                display: block;
                                text-align: left;
                                margin-top: 12px;
                            "
                        >
                            {"Learn how ODP projects help build secure, modern devices"}
                        </span>
                    </div>
                    <iframe
                        width="600"
                        height="338"
                        style="border-radius: 10px;"
                        src="https://www.youtube.com/embed/YOUR_VIDEO_ID"
                        title="YouTube Video of the Open Device Partnership"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen
                    ></iframe>
                </div>
                <div style="margin-top: 40px;">
                    <iframe
                        width="920"
                        height="518"
                        style="border-radius: 10px; display: block;"
                        src="https://www.youtube.com/embed/SECOND_VIDEO_ID"
                        title="Additional Training Video"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen
                    ></iframe>
                </div>
            </div>
        </section>

        // Two Columns Section
        <section
            style="
                padding: 80px 120px;
                background: #fff;
            "
        >
            <div class="flex flex-row gap-[60px]">
                {/* Column 1 */}
                <div class="flex flex-col items-start" style="flex: 1;">
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 60px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 110%;
                            letter-spacing: -1.2px;
                            margin-bottom: 24px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Partner-Oriented Vision"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"ODP is an inclusive partnership open to OEMs, ODMs, silicon vendors, hardware developers, security researchers, and anyone committed to improving device software foundations."}
                    </span>
                </div>
                {/* Column 2 */}
                <div class="flex flex-col items-start" style="flex: 1;">
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 60px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 110%;
                            letter-spacing: -1.2px;
                            margin-bottom: 24px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Get Involved!"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Explore our documentation, clone our public repositories, and contribute your expertise. Together, we can raise the standard for trusted devices."}
                    </span>
                </div>
            </div>
        </section>
    }
}
