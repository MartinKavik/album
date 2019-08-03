use seed::prelude::*;

///Model
pub struct Model {}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

///Update
#[derive(Clone)]
pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut Orders<Msg>) {}

///View
pub fn view(_model: &Model) -> impl ElContainer<Msg> {
    header![class!("navbar header__navbar"),
        section![class!("navbar-section"),
            a!["Albums", class!("btn btn-link"), attrs!{At::Href => "/albums"} ]
        ],
        section![class!("navbar-center"),
            a![class!("header__imglink"),
                attrs!{At::Href => "/"},
                img![class!("header__img"),
                    attrs!{At::Src => "front/assets/image/camera.png"}
                ] 
            ],
        ],
		section![class!("navbar-section"),
			a!["Pictures", class!("btn btn-link"), attrs!{At::Href => "/pictures"} ],
        ]
    ]
}