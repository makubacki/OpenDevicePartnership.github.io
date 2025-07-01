use leptos::prelude::*;

#[component]
pub fn ProjectIntroduction(
    #[prop(into)] project_title: String,
    #[prop(into)] project_what: String,
    #[prop(into)] project_why: String,
) -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px;
            "
        >
            <div class="flex flex-row gap-[80px]">
                {/* Left Column */}
                <div class="flex flex-col items-start" style="width: 700px;">
                    <span
                        class="h1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_title}
                    </span>
                </div>
                {/* Right Column */}
                <div class="flex flex-col items-start" style="width: 600px;">
                    {/* WHAT label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_what}
                    </span>
                    {/* WHY label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {project_why}
                    </span>
                </div>
            </div>
        </section>
    }
}