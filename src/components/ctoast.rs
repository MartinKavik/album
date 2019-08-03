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
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::Show(toast) => {
            log!(toast.msg);
            model.is_visible = true;
            model.is_error = toast.is_error;
            model.msg = toast.msg;
        }
    }
}

///View
pub fn view(model: &Model) -> impl ElContainer<Msg> {
    let display = match model.is_visible {
        true => "",
        false => "d-none"
    };
    let color = match model.is_error {
        true => "toast-error",
        false => "toast-success"
    };
    let class = ["toast", "ctoast", display, color].join(" ");
    let class_str: &str = &class;
    span![class!(class_str),
        model.msg,
        button![class!("btn btn-clear float-right")]
    ]
}