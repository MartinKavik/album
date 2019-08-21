#![allow(private_in_public)]

use seed::prelude::*;
use seed::fetch;
use seed::fetch::{Request, Method};
use futures::Future;
use serde::{Serialize, Deserialize};

use crate::toast;

///Model
pub struct Model {
	is_logged: bool,
	api_url: String,
    email: String,
    password: String
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
            email: "".to_string(),
            password: "".to_string()
		}
    }
}

impl Model {
    ///Constructor
    pub fn new(api_url: String) -> Self {
		Model {
			is_logged: false,
			api_url: api_url + "login",
            email: "".to_string(),
            password: "".to_string()
		}
	}
}

///Update
#[derive(Clone)]
pub enum Msg {
	SendMessage,
    MessageSent(fetch::FetchObject<ResponseBody>),
    Email(String),
    Password(String),
    Toast(toast::Toast),
    SaveToken(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
        Msg::SendMessage => {
            orders.skip().perform_cmd(send_message(model));
        },
        Msg::MessageSent(fetch_object) => match fetch_object.response() {
            Ok(response) => {
                orders.send_msg(Msg::SaveToken(response.data.token)).skip();
            }
            Err(_fail_reason) => {
                let toast = toast::Toast { 
                    is_error: true, 
                    msg: "Login error".to_string()
                };
                orders.send_msg(Msg::Toast(toast))
                    .skip();
            }
        },
        Msg::Email(email) => model.email = email,
        Msg::Password(password) => model.password = password,
        Msg::Toast(_toast) => (),
        Msg::SaveToken(_token) => ()
    }
}

fn send_message(model: &mut Model) -> impl Future<Item = Msg, Error = Msg> {
    let message = RequestBody {
        email: model.email.clone().into(),
        password: model.password.clone().into(),
    };
    Request::new(model.api_url.clone())
        .method(Method::Post)
        .header("Content-Type", "application/json")
        .send_json(&message)
        .fetch_json(Msg::MessageSent)
}

///View
pub fn view(model: &Model) -> impl View<Msg> {
	match model.is_logged {
		true => empty![],
		false => {
			div![class!("panel"),
				form![class!("panel-body"),
					div![class!("form-group"),
						label!["Email",  class!("form-label"), attrs!{At::For => "email"}],
						input![
                            class!("form-input"), 
                            attrs!{
                                At::Value => model.email;
                                At::Type => "text"; 
                                At::Id => "email"; 
                                At::Placeholder => "Email" },
                            input_ev(Ev::Input, Msg::Email)
                        ]
					],
					div![class!("form-group"),
						label!["Password", class!("form-label"), attrs!{At::For => "password"}],
						input![
                            class!("form-input"), 
                            attrs!{
                                At::Value => model.password;
                                At::Type => "password"; 
                                At::Id => "password"; 
                                At::Placeholder => "Password"
                                At::AutoComplete => true,
                            },
                            input_ev(Ev::Input, Msg::Password)]
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