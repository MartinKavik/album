use seed::prelude::*;
use serde::{Serialize, Deserialize};

///Model
pub struct Model {
    pub id: u32
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            id: 0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Picture {
    pub id: u32,
    pub data: String,
    pub model: String,
    pub date: String,
    pub latitude: String,
    pub longitude: String,
}

///Update
#[derive(Clone)]
pub enum Msg {}

pub fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

///View
pub fn view(model: &Model) -> impl View<Msg> {
    img![
         attrs!{At::Id => model.id}
    ]
}