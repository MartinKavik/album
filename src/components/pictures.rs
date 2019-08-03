use seed::prelude::*;
use seed::fetch;
use seed::fetch::{Request};
use futures::Future;
//use serde::{Serialize, Deserialize};

///Model
pub struct Model {
    api_url: String,
    token: Option<String>,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            api_url: "".to_string(),
            token: None
        }
    }
}

impl Model {
    ///Constructor
    pub fn new(api_url: String) -> Self {
		Model {
			api_url: api_url + "picture",
            token: None
		}
	}
}

///Update
#[derive(Clone)]
pub enum Msg {
    SetToken(String),
    FetchData,
    DataFetched(fetch::FetchObject<Vec<u32>>)
}

fn fetch_data(api_url: String, token: String) -> impl Future<Item = Msg, Error = Msg> {
    log!(token);
    Request::new(api_url.clone())
        .header("token", &token) 
        .fetch_json(Msg::DataFetched)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetToken(token) => model.token = Some(token),
        Msg::FetchData => {
            match model.token.clone() {
                Some(token) => {
                    orders.skip()
                        .perform_cmd(fetch_data(model.api_url.clone(), token));
                },
                None => ()
            }
        },
        Msg::DataFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => log!(response.data),
                Err(_fail_reason) => {
                    /*orders
                        .send_msg(Msg::OnFetchError {
                            label: "Fetching repository info failed",
                            fail_reason,
                        })
                        .skip();*/
                }
            }
        },
    }
}

///View
pub fn view(_model: &Model) -> impl View<Msg> {
    span!["pictures"]
}