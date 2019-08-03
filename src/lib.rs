#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::*;

#[path="./components/header.rs"]
mod header;
#[path="./components/home.rs"]
mod home;
#[path="./components/pictures.rs"]
mod pictures;
#[path="./components/albums.rs"]
mod albums;
#[path="./components/login.rs"]
mod login;
#[path="./model/toast.rs"]
pub mod toast;
#[path="./components/ctoast.rs"]
pub mod ctoast;

///Routes
fn routes(url: seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(0)
    }
    match url.path[0].as_ref() {
        "albums" => Msg::ChangePage(1),
		"pictures" => Msg::ChangePage(2),
        _ => Msg::ChangePage(0)
    }
}

///Model
struct Model {
    page_id: u32,
    token: Option<String>,
    header: header::Model,
	home: home::Model,
    albums: albums::Model,
	pictures: pictures::Model,
	login: login::Model,
    ctoast: ctoast::Model,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page_id: 0,
            token: None,
            header: header::Model::default(),
            home: home::Model::default(),
            albums: albums::Model::default(),
			pictures: pictures::Model::default(),
			login: login::Model::default(),
            ctoast: ctoast::Model::default(),
        }
    }
}

impl Model {
    ///Constructor
    pub fn new(api_url: String) -> Self {
        let login = login::Model::new(api_url.clone());
        let pictures = pictures::Model::new(api_url.clone());
        Model {
            page_id: 0,
            token: None,
            header: header::Model::default(),
            home: home::Model::default(),
            albums: albums::Model::default(),
			pictures: pictures,
			login: login,
            ctoast: ctoast::Model::default()
        }
    }
}

///Update
#[derive(Clone)]
enum Msg {
    ChangePage(u32),
    Header(header::Msg),
    Home(home::Msg),
	Albums(albums::Msg),
    Pictures(pictures::Msg),
    Login(login::Msg),
    CToast(ctoast::Msg)
}

///How we update the model
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::ChangePage(page_id) => {
            model.page_id = page_id;
             match page_id {
                 2 => {
                     *orders = call_update(pictures::update, pictures::Msg::FetchData, &mut model.pictures)
                        .map_message(Msg::Pictures);
                 },
                 _ => ()
             };
        }
		Msg::Home(msg) => {
            *orders = call_update(home::update, msg, &mut model.home)
                .map_message(Msg::Home);
        },
        Msg::Header(msg) => {
            *orders = call_update(header::update, msg, &mut model.header)
                .map_message(Msg::Header);
        },
		Msg::Albums(msg) => {
            *orders = call_update(albums::update, msg, &mut model.albums)
                .map_message(Msg::Albums);
        },
		Msg::Pictures(msg) => {
            *orders = call_update(pictures::update, msg, &mut model.pictures)
                .map_message(Msg::Pictures);
        },
		Msg::Login(msg) => {
            match msg.clone() {
                login::Msg::Toast(toast) => {
                    call_update(ctoast::update, ctoast::Msg::Show(toast), &mut model.ctoast)
                        .map_message(Msg::CToast);
                },
                login::Msg::SaveToken(token) => {
                    model.token = Some(token.clone());
                    call_update(pictures::update, pictures::Msg::SetToken(token.clone()), &mut model.pictures)
                        .map_message(Msg::Pictures);
                }
                _ => ()
            };
            *orders = call_update(login::update, msg.clone(), &mut model.login)
                .map_message(Msg::Login);
        },
        Msg::CToast(msg) => {
            *orders = call_update(ctoast::update, msg, &mut model.ctoast)
                .map_message(Msg::CToast);
        }
    }
}

///View
fn view(model: &Model) -> El<Msg> {
    div![
        ctoast::view(&model.ctoast).els().map_message(Msg::CToast),
        header::view(&model.header).els().map_message(Msg::Header),
        div![class!("main__container"),
            match model.token.clone() {
                Some(_token) => {
                    match model.page_id {
                        1 => div![albums::view(&model.albums).els().map_message(Msg::Albums)],
                        2 => div![pictures::view(&model.pictures).els().map_message(Msg::Pictures)],
                        _ =>  div![home::view(&model.home).els().map_message(Msg::Home)],
                    }
                },
                None => div![login::view(&model.login).els().map_message(Msg::Login)]
            }
        ]
    ]
}

#[wasm_bindgen]
pub fn render(api_url: &str) {
    let model = Model::new(api_url.to_string());
    seed::App::build(model, update, view)
        .routes(routes)
        .finish()
        .run();
}