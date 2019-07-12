#[macro_use]
extern crate seed;
use seed::prelude::*;
use seed::*;

#[path="./components/header.rs"]
mod header;
#[path="./components/albums.rs"]
mod albums;

///Routes
fn routes(url: seed::Url) -> Msg {
    //log!(url.path);
    if url.path.is_empty() {
        return Msg::ChangePage(0)
    }
    match url.path[0].as_ref() {
        "albums" => Msg::ChangePage(1),
        _ => Msg::ChangePage(0)
    }
}

///Model
struct Model {
    page_id: u32,
    albums: albums::Model,
    header: header::Model
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page_id: 0,
            albums: albums::Model::default(),
            header: header::Model::default()
        }
    }
}

///Update
#[derive(Clone)]
enum Msg {
    ChangePage(u32),
    Example(albums::Msg),
    Header(header::Msg)
}

///How we update the model
fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::ChangePage(page_id) => model.page_id = page_id,
        Msg::Example(msg) => {
            *orders = call_update(albums::update, msg, &mut model.albums)
            .map_message(Msg::Example);
        },
        Msg::Header(msg) => {
            *orders = call_update(header::update, msg, &mut model.header)
            .map_message(Msg::Header);
        }
    }
}


// View
/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> El<Msg> {
    div![
        /*header![class!("navbar main__navbar"),
            section![class!("navbar-section"),
                a!["home", attrs!{At::Href => "/"} ],
            ],
            section![class!("navbar-center"),
                img![attrs!{At::Src => "front/assets/image/rust.png"}]
            ],
            section![class!("navbar-section"),
                a!["page_test", attrs!{At::Href => "/page_test"} ],
            ]
        ],*/
        header::view(&model.header).els().map_message(Msg::Header),
        div![
            match model.page_id {
                1 => div![albums::view(&model.albums).els().map_message(Msg::Example)],
                _ =>  span!["home"]
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