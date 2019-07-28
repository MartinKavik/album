use seed::prelude::*;
use seed::log;
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
		log!("model");
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
			api_url: api_url,
		}
	}
}

///Update
#[derive(Clone)]
pub enum Msg {
	
}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut Orders<Msg>) {
	log!("update");
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
					button!["Login", class!("btn btn-primary")]
				]
			]
		}
	}
}