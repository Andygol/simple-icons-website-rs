pub mod pdf;
pub mod svg;

use crate::controls::button::ControlButtonText;
use crate::storage::{
    conversion_get_from_localstorage, set_on_localstorage, LocalStorage,
};
use crate::Url;
use i18n::{move_tr, tr};
use leptos::{document, window, *};
pub use pdf::download_pdf;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
pub use svg::download_svg;
use wasm_bindgen::JsCast;
use web_sys;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum DownloadType {
    #[default]
    SVG,
    PDF,
}

impl FromStr for DownloadType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "svg" => Ok(Self::SVG),
            "pdf" => Ok(Self::PDF),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DownloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SVG => write!(f, "svg"),
            Self::PDF => write!(f, "pdf"),
        }
    }
}

pub fn provide_download_type_context() {
    provide_context(DownloadTypeSignal(create_rw_signal(
        initial_download_type(),
    )));
}

#[derive(Copy, Clone)]
pub struct DownloadTypeSignal(pub RwSignal<DownloadType>);

fn initial_download_type() -> DownloadType {
    match Url::params::get_param!(DownloadType, DownloadType) {
        Some(download_type) => {
            set_download_type_on_localstorage(&download_type);
            download_type
        }
        None => match get_download_type_from_localstorage() {
            Some(download_type) => download_type,
            None => DownloadType::default(),
        },
    }
}

fn get_download_type_from_localstorage() -> Option<DownloadType> {
    conversion_get_from_localstorage!(DownloadType, DownloadType)
}

fn set_download_type_on_localstorage(download_type: &DownloadType) {
    set_on_localstorage!(DownloadType, &download_type.to_string())
}

#[component]
pub fn DownloadFileTypeControl() -> impl IntoView {
    let download_type = expect_context::<DownloadTypeSignal>().0;
    let download_svg_title = move_tr!("download-filetype", &{
        let mut map = HashMap::new();
        map.insert("filetype".to_string(), tr!("svg").into());
        map
    });
    let download_pdf_title = move_tr!("download-filetype", &{
        let mut map = HashMap::new();
        map.insert("filetype".to_string(), tr!("pdf").into());
        map
    });

    view! {
        <div class="control">
            <label>{move_tr!("download")}</label>
            <div class="flex flex-row">
                <ControlButtonText
                    text=move_tr!("svg")
                    title=download_svg_title
                    active=move || { download_type() == DownloadType::SVG }
                    on:click=move |_| {
                        download_type
                            .update(move |state| {
                                *state = DownloadType::SVG;
                                set_download_type_on_localstorage(state);
                            });
                    }
                />

                <ControlButtonText
                    text=move_tr!("pdf")
                    title=download_pdf_title
                    active=move || { download_type() == DownloadType::PDF }
                    on:click=move |_| {
                        download_type
                            .update(|state| {
                                *state = DownloadType::PDF;
                                set_download_type_on_localstorage(state);
                            });
                    }
                />

            </div>
        </div>
    }
}

/// Download a SVG icon by its slug
pub fn download(filename: &str, href: &str) {
    let link = document()
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    link.set_attribute("class", "hidden").unwrap();
    link.set_attribute("download", filename).unwrap();
    link.set_attribute("href", href).unwrap();
    let body = document().body().unwrap();
    body.append_child(&link).unwrap();
    link.click();
    body.remove_child(&link).unwrap();
}
