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
    header![class!("navbar main__navbar"),
        section![class!("navbar-section"),
            a!["pictures", attrs!{At::Href => "/pictures"} ],
        ],
        section![class!("navbar-center"),
            a![img![attrs!{At::Src => "front/assets/image/camera.png"}],  attrs!{At::Href => "/"} ],
        ],
        section![class!("navbar-section"),
            a!["albums", attrs!{At::Href => "/albums"} ],
        ]
    ]
}