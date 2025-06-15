use leptos::prelude::*;

#[derive(Clone)]
pub struct DocLink {
    pub href: &'static str,
    pub title: &'static str,
}

#[component]
pub fn DocumentationTraining(
    #[prop(default = vec![])] links: Vec<DocLink>
) -> impl IntoView {
    view! {
        <section
            class="flex flex-row items-start"
            style="padding: 120px;"
        >
            {/* Left: Image and text box */}
            <div class="flex flex-col items-start" style="min-width: 420px; align-items: flex-start;">
                <img
                    src="/images/documentation.svg"
                    alt="Documentation Icon"
                    style="
                        width: 150px;
                        height: 150px;
                        object-fit: contain;
                        display: block;
                        margin-bottom: 16px;
                    "
                />
                <span
                    class="font-geist"
                    style="
                        font-size: 60px;
                        font-weight: 500;
                        line-height: 66px;
                        letter-spacing: -1.2px;
                        text-align: left;
                        display: block;
                    "
                >
                    "Documentation Training"
                </span>
                <div style="height: 10px;"></div>
                <span
                    class="font-geist"
                    style="
                        font-size: 25px;
                        font-weight: 400;
                        line-height: 36px;
                        letter-spacing: -0.7px;
                        text-align: left;
                        display: block;
                    "
                >
                    "Start Developing with ODP Standards"
                </span>
            </div>

            {/* Spacer between left and right */}
            <div style="width: 200px;"></div>

            {/* Right: List of hyperlinks */}
            <ul class="flex flex-col pt-4">
                {links.into_iter().map(|link| view! {
                    <li>
                        <a href=link.href class="flex items-center text-[35px] font-geist font-medium underline">
                            {format!("→ {}", link.title)}
                        </a>
                    </li>
                }).collect_view()}
            </ul>
        </section>
    }
}