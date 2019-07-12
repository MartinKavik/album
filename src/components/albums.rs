use seed::prelude::*;

///Model
pub struct Model {
    pub test: String
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            test: "albums".to_string()
        }
    }
}

///Update
#[derive(Clone)]
pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut Orders<Msg>) {}

///View
pub fn view(model: &Model) -> impl ElContainer<Msg> {
    span![model.test]
}