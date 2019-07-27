use seed::prelude::*;

///Model
pub struct Model {
	is_logged: bool
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
			is_logged: false
		}
    }
}

///Update
#[derive(Clone)]
pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut Orders<Msg>) {}

///View
pub fn view(model: &Model) -> impl ElContainer<Msg> {
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