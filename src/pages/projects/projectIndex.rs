use material_yew::MatButton;
use yew::prelude::*;
use yew_router::prelude::*;

mod projectInfoStructs;

#[function_component(Projects)]
pub fn projects_overview() -> Html {
    let projects = projectInfoStructs::getProjects();
    let projects = projects
        .iter()
        .map(|project| {
            html! {
                <p key={project.id}>{format!("{}: {}",
                    project.title,
                    project.year)}
                <br/>
                </p>
            }
        })
        .collect::<Html>();
    html! {
        <main>
            <h1>{"Projects"}</h1>
            {projects}
        </main>
    }
}
