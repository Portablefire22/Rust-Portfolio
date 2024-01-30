use material_yew::MatButton;
use yew::prelude::*;
use yew_router::prelude::*;

#[path = "pages/projects/projectIndex.rs"]
mod projectIndex;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Index,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<Index/>},
        Route::Projects => html! {<projectIndex::Projects/>},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Index)]
fn index() -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <indexContainer>
            <h1>{"KittyGoesMeow"}</h1>
            <p>{"Professional Silliness :3"}</p>
            <span onclick={Callback::from(move |_| navigator.push(&Route::Projects))}>
                <MatButton label="Enter Hell" icon={AttrValue::from("arrow_forward")}
                    trailing_icon=true raised=true/>
            </span>
        </indexContainer>
    }
}

/* Colour scheme
 * Catppuccin Mocha
 *
 * Colour Palette:
 * https://github.com/catppuccin/catppuccin#-palette
 *
 * Style Guide:
 * https://github.com/catppuccin/catppuccin/blob/main/docs/style-guide.md
 */
