mod components;

use components::blockchain::BlockchainComponent;
use yew::prelude::*;
use yew::Renderer;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Blockchain Data Viewer" }</h1>
                <BlockchainComponent />
            </div>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}
