use seed::prelude::*;

///Model
#[derive(Debug, Clone)]
pub struct Model {
	files: Option<web_sys::FileList>
}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
			files: None
        }
    }
}

///Update
#[derive(Clone)]
pub enum Msg {
    Drop,
    DragOver,
	SendFiles(web_sys::Event)
}

pub fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Drop => {
            log!("drop");
        },
        Msg::DragOver => {
            //log!("dragover {:?}", event);
            orders.skip();
        },
		Msg::SendFiles(event) => {
			log!("{:?}", event);
		}
    }
}

///View
pub fn view(_model: &Model) -> impl View<Msg> {
    div![class!("upload__container"),
        div![class!("panel upload__panel"),
            simple_ev(Ev::DragOver, Msg::DragOver),
            simple_ev(Ev::Click, Msg::Drop),
            i![class!("icon icon-upload upload__icon")],
            span!["Upload"],
			form![
				attrs!{ 
					At::Method => "post"
				},
				input![
					attrs!{
						At::Type => "file",
						At::Accept => "image/png, image/jpeg",
						At::Multiple => true
					},
					raw_ev(Ev::Input, Msg::SendFiles)
				]
			]
        ]
    ]
}