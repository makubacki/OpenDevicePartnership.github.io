use leptos::*;
use leptos::html::*;
use leptos::prelude::*;

#[derive(Clone)]
pub struct TeamMember {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub role: &'static str,
    pub github_username: &'static str,
}

#[component]
pub fn TeamGrid(
    #[prop(into)] members: Vec<TeamMember>,
) -> impl IntoView {
    view! {
        <style>
            {r#"
            .grid-container {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
                max-width: calc(5 * (150px + 60px) - 60px);
                gap: 60px;
                justify-items: center;
                padding: 20px 0 60px 0;
                margin: 0 auto;
            }
            .member-card {
                text-align: left;
                width: 150px;
            }
            .member-image {
                width: 150px;
                height: 150px;
                background-color: #ccc;
                background-size: cover;
                background-position: center;
                margin-bottom: 8px;
            }
            .member-name {
                font-weight: bold;
                margin-bottom: 4px;
            }
            .member-role {
                font-size: 0.9em;
                margin-bottom: 2px;
            }
            .member-username {
                font-size: 0.85em;
                color: #555;
            }
            "#}
        </style>

        <div class="grid-container">
            {members.into_iter().map(|member| {
                view! {
                    <div class="member-card">
                        <div class="member-image"></div>
                        <div class="member-name">{format!("{} {}", member.first_name, member.last_name)}</div>
                        <div class="member-role">{member.role}</div>
                        <div class="member-username">{"GitHub: "}{member.github_username}</div>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
