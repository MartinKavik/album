use seed::prelude::*;
use seed::fetch;
use seed::fetch::{Request};
use futures::Future;
use serde::{Serialize, Deserialize};

use crate::toast;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Picture {
    pub id: u32,
    pub data: String,
    pub model: String,
    pub date: String,
    pub latitude: String,
    pub longitude: String,
}

///Update
#[derive(Clone)]
pub enum Msg {
    SetToken(String),
    FetchIds,
    IdsFetched(fetch::FetchObject<Vec<u32>>),
    Toast(toast::Toast),
}

fn fetch_ids(api_url: String, token: String) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(api_url.clone())
        .header("token", &token) 
        .fetch_json(Msg::IdsFetched)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetToken(token) => model.token = Some(token),
        Msg::FetchIds => {
            match model.token.clone() {
                Some(token) => {
                    orders.skip()
                        .perform_cmd(fetch_ids(model.api_url.clone(), token));
                },
                None => ()
            }
        },
        Msg::IdsFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => log!(response.data),
                Err(_fail_reason) => {
                    let toast = toast::Toast { 
                        is_error: true, 
                        msg: "Error getting pictures".to_string()
                    };
                    orders.send_msg(Msg::Toast(toast))
                        .skip();
                }
            }
        },
        Msg::Toast(_toast) => (),
    }
}

///View
pub fn view(_model: &Model) -> impl View<Msg> {
    span!["pictures"]
}