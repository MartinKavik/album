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

///Routes
fn routes(url: seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(0)
    }
    match url.path[0].as_ref() {
		"login" => Msg::ChangePage(1),
        "albums" => Msg::ChangePage(2),
		"pictures" => Msg::ChangePage(3),
        _ => Msg::ChangePage(0)
    }
}

///Model
struct Model {
    page_id: u32,
    header: header::Model,
	home: home::Model,
    albums: albums::Model,
	pictures: pictures::Model,
	login: login::Model,
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page_id: 0,
            header: header::Model::default(),
            home: home::Model::default(),
            albums: albums::Model::default(),
			pictures: pictures::Model::default(),
			login: login::Model::default(),
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
}

///How we update the model
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::ChangePage(page_id) => model.page_id = page_id,
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
            *orders = call_update(login::update, msg, &mut model.login)
            .map_message(Msg::Login);
        },
    }
}

///View
fn view(model: &Model) -> El<Msg> {
    div![
        header::view(&model.header).els().map_message(Msg::Header),
        div![
            match model.page_id {
                1 => div![login::view(&model.login).els().map_message(Msg::Login)],
				2 => div![albums::view(&model.albums).els().map_message(Msg::Albums)],
				3 => div![pictures::view(&model.pictures).els().map_message(Msg::Pictures)],
                _ =>  div![home::view(&model.home).els().map_message(Msg::Home)],
            }
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}