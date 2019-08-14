#![allow(private_in_public)]

use seed::prelude::*;
use seed::fetch;
use seed::fetch::{Request};
use futures::Future;
use serde::{Serialize, Deserialize};

use crate::toast;

#[path="./upload.rs"]
pub mod upload;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PictureThumb {
    pub id: u32,
    pub thumb: String,
}

///Model
pub struct Model {
    api_url: String,
    token: Option<String>,
    ids: Vec<u32>,
    pics: Vec<PictureThumb>,
    upload: upload::Model,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            api_url: "".to_string(),
            token: None,
            ids: Vec::new(),
            pics: Vec::new(),
            upload: upload::Model::default(),
        }
    }
}

impl Model {
    ///Constructor
    pub fn new(api_url: String) -> Self {
		Model {
			api_url: api_url + "picture",
            token: None,
            ids: Vec::new(),
            pics: Vec::new(),
            upload: upload::Model::default(),
		}
	}
}

///Update
#[derive(Clone)]
pub enum Msg {
    SetToken(String),
    FetchIds,
    IdsFetched(fetch::FetchObject<Vec<u32>>),
    LoadSomePics,
    FetchPic(u32),
    PicFetched(fetch::FetchObject<PictureThumb>),
    Toast(toast::Toast),
    Upload(upload::Msg),
}

fn fetch_ids(api_url: String, token: String) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(api_url.clone())
        .header("token", &token) 
        .fetch_json(Msg::IdsFetched)
}

fn fetch_pic(api_url: String, token: String, id: u32) -> impl Future<Item = Msg, Error = Msg> {
    Request::new(api_url.clone() + "/" + &id.to_string() + "/thumb")
        .header("token", &token) 
        .fetch_json(Msg::PicFetched)
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetToken(token) => model.token = Some(token),
        Msg::FetchIds => {
            match model.token.clone() {
                Some(token) => {
                    orders.skip()
                        .perform_cmd(fetch_ids(model.api_url.clone(), token));
                },
                None => ()
            }
        },
        Msg::IdsFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => {
                    model.ids = response.data;
                    orders.send_msg(Msg::LoadSomePics);
                }
                Err(_fail_reason) => {
                    let toast = toast::Toast { 
                        is_error: true, 
                        msg: "Error getting pictures ids".to_string()
                    };
                    orders.send_msg(Msg::Toast(toast))
                        .skip();
                }
            }
        },
        Msg::LoadSomePics => {
            model.pics = Vec::new();
            //Only 20
            for &id in model.ids.iter().take(20) {
                orders.send_msg(Msg::FetchPic(id));
            };
        },
        Msg::FetchPic(id) => {
            match model.token.clone() {
                Some(token) => {
                    orders.skip()
                        .perform_cmd(fetch_pic(model.api_url.clone(), token, id));
                },
                None => ()
            }
        },
        Msg::PicFetched(fetch_object) => {
            match fetch_object.response() {
                Ok(response) => {
                    model.pics.push(response.data);
                }
                Err(_fail_reason) => {
                    let toast = toast::Toast { 
                        is_error: true, 
                        msg: "Error getting picture".to_string()
                    };
                    orders.send_msg(Msg::Toast(toast))
                        .skip();
                }
            }
        },
        Msg::Toast(_toast) => (),
        Msg::Upload(msg) => {
            upload::update(msg, &mut model.upload, &mut orders.proxy(Msg::Upload));
        },
    }
}

///View
pub fn view(model: &Model) -> impl View<Msg> {
    div![class!("picture__container"),
        upload::view(&model.upload).els().map_message(Msg::Upload),
        div![class!("picture__list"),
            model.pics.iter().map(|pic| 
                img![class!("picture__img"),
                    attrs!{
                        At::Id => pic.id; 
                        At::Alt => pic.id, 
                        At::Src => format!("data:image/png;base64,{}", &pic.thumb)
                    }
                ]
            )
        ]
    ]
}