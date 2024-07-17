use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize, Clone, Debug)]
struct Block {
    index: usize,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Deserialize, Clone, Debug)]
struct BlockchainData {
    chain: Vec<Block>,
}

pub struct BlockchainComponent {
    blockchain_data: Option<BlockchainData>,
}

pub enum Msg {
    FetchData,
    ReceiveData(Result<BlockchainData, reqwasm::Error>),
}

impl Component for BlockchainComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchData);
        Self { blockchain_data: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchData => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = Request::get("http://127.0.0.1:3030/blockchain")
                        .send()
                        .await
                        .unwrap()
                        .json::<BlockchainData>()
                        .await;
                    link.send_message(Msg::ReceiveData(response));
                });
                false
            }
            Msg::ReceiveData(response) => {
                self.blockchain_data = response.ok();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{ "Blockchain Data" }</h2>
                {
                    if let Some(blockchain_data) = &self.blockchain_data {
                        html! {
                            <ul>
                                { for blockchain_data.chain.iter().map(|block| html!{
                                    <li>
                                        { format!("Block #{}: {} transactions", block.index, block.transactions.len()) }
                                    </li>
                                }) }
                            </ul>
                        }
                    } else {
                        html! { <p>{ "Loading..." }</p> }
                    }
                }
            </div>
        }
    }
}
