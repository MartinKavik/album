use seed::prelude::*;

///Model
pub struct Model {
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
        }
    }
}

///Update
#[derive(Clone)]
pub enum Msg {
    Drop,
    DragOver
}

pub fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Drop => {
            log!("drop");
        },
        Msg::DragOver => {
            //log!("dragover {:?}", event);
            orders.skip();
        }
    }
}

///View
pub fn view(_model: &Model) -> impl View<Msg> {
    div![class!("panel upload__panel"),
        simple_ev(Ev::DragOver, Msg::DragOver),
        simple_ev(Ev::Click, Msg::Drop)
    ]
}