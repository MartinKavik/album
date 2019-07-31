#![allow(private_in_public)]

use seed::prelude::*;
use seed::log;
use seed::fetch;
use seed::fetch::{Request, Method};
use futures::Future;
use serde::{Serialize, Deserialize};

///Model
pub struct Model {
	is_logged: bool,
	api_url: String,
}

#[derive(Serialize)]
struct RequestBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseBody {
    pub token: String,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
		Self {
			is_logged: false,
			api_url: "".to_string(),
		}
    }
}

impl Model {
    ///Constructor
    pub fn new(api_url: String) -> Self {
		Model {
			is_logged: false,
			api_url: api_url + "/login",
		}
	}
}

///Update
#[derive(Clone)]
pub enum Msg {
	SendMessage,
    MessageSent(fetch::FetchObject<ResponseBody>),
    OnFetchError {
        label: &'static str,
        fail_reason: fetch::FailReason,
    },
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
	match msg {
        Msg::SendMessage => {
            orders.skip().perform_cmd(send_message(model.api_url.clone()));
        },
        Msg::MessageSent(fetch_object) => match fetch_object.response() {
            Ok(response) => {
                log!(format!("Response data: {:#?}", response.data));
                orders.skip();
            }
            Err(fail_reason) => {
                orders
                    .send_msg(Msg::OnFetchError {
                        label: "Sending message failed",
                        fail_reason,
                    })
                    .skip();
            }
        },
        Msg::OnFetchError { label, fail_reason } => {
            log!(format!("Fetch error - {} - {:#?}", label, fail_reason));
            orders.skip();
        }
    }
}

fn send_message(api_url: String) -> impl Future<Item = Msg, Error = Msg> {
    let message = RequestBody {
        email: "test@test.com".into(),
        password: "55-Street".into(),
    };

    Request::new(api_url.into())
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .send_json(&message)
        .fetch_json(Msg::MessageSent)
}

///View
pub fn view(model: &Model) -> impl ElContainer<Msg> {

	log!("view");

	match model.is_logged {
		true => empty![],
		false => {
			div![class!("panel"),
				div![class!("panel-body"),
					div![class!("form-group"),
						label!["Email",  class!("form-label"), attrs!{At::For => "email"}],
						input![class!("form-input"), attrs!{At::Type => "text"; At::Id => "email"; At::Placeholder => "Email" }]
					],
					div![class!("form-group"),
						label!["Password", class!("form-label"), attrs!{At::For => "password"}],
						input![class!("form-input"), attrs!{At::Type => "password"; At::Id => "password"; At::Placeholder => "Password" }]
					]
				],
				div![class!("panel-footer"),
					button!["Login", 
						class!("btn btn-primary"),
						simple_ev(Ev::Click, Msg::SendMessage)
					]
				]
			]
		}
	}
}