use seed::prelude::*;
use wasm_bindgen::JsCast;

///Model
#[derive(Debug, Clone)]
pub struct Model {}

///Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

///Update
#[derive(Clone)]
pub enum Msg {
    Drop,
    DragOver,
	FileChanged(Option<web_sys::FileList>)
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
		Msg::FileChanged(files_opt) => {
            match files_opt {
                Some(files) => {
                    for i in 0..files.length() - 1 {
                        let file_opt = files.get(i);
                        match file_opt {
                            Some(file) => {
                                let form_data = web_sys::FormData::new().unwrap();
                                //form_data.append_with_blob("file", file);
                                log!("{:?}", form_data);
                            },
                            None => ()
                        }
                    }
                },
                None => ()
            }
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
                    raw_ev(Ev::Input, |event| {
                        let files = event
                            .target()
                            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .and_then(|file_input| file_input.files());
                        Msg::FileChanged(files)
                    }),
					attrs!{
						At::Type => "file",
						At::Accept => "image/png, image/jpeg",
						At::Multiple => true
					},
					
				]
			]
        ]
    ]
}