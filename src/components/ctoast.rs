use seed::prelude::*;

use crate::toast;

///Model
pub struct Model {
    pub msg: String,
    pub is_error: bool,
    pub is_visible: bool,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            msg: "".to_string(),
            is_error: false,
            is_visible: false,
        }
    }
}

///Update
#[derive(Clone)]
pub enum Msg {
    Show(toast::Toast),
    Hide
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::Show(toast) => {
            model.is_visible = true;
            model.is_error = toast.is_error;
            model.msg = toast.msg;
        }
        Msg::Hide => model.is_visible = false
    }
}

///View
pub fn view(model: &Model) -> impl ElContainer<Msg> {
    let display = match model.is_visible {
        true => "ctoast__toast--visible",
        false => "ctoast__toast--hidden"
    };
    let color = match model.is_error {
        true => "toast-error",
        false => "toast-success"
    };
    let class = ["toast ctoast__toast", color, display].join(" ");
    let class_str: &str = &class;
    div![class!("ctoast__container"),
        span![class!(class_str),
            model.msg,
            button![
                class!("btn btn-clear float-right"),
                simple_ev(Ev::Click, Msg::Hide)
            ]
        ]
    ]
}