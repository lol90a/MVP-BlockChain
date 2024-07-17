pub mod components;

use components::blockchain::BlockchainComponent;
use yew::prelude::*;
use yew::Renderer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Blockchain Explorer" }</h1>
            <BlockchainComponent />
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
