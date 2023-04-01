use yew::prelude::*;
mod components;
mod data;
mod model;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <center>
                <h1>{"Tally"}</h1>
            </center>
            <components::tally_table::TallyTable />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
