//! Localization system of simple-icons website

use leptos::*;
use std::collections::HashMap;
use web_sys;

#[derive(Clone, Copy)]
pub struct Language {
    /// Language code
    pub code: &'static str,
    /// Language name
    pub name: &'static str,
}

pub static LANGUAGES: [Language; 9] = [
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "es",
        name: "Español",
    },
    Language {
        code: "fr",
        name: "Français",
    },
    Language {
        code: "it",
        name: "Italiano",
    },
    Language {
        code: "ja-JP",
        name: "日本語",
    },
    Language {
        code: "pt",
        name: "Português",
    },
    Language {
        code: "zh-CN",
        name: "中文 (简体)",
    },
    Language {
        code: "zh-HK",
        name: "中文 (香港)",
    },
    Language {
        code: "zh-TW",
        name: "中文 (繁體)",
    },
];

impl Language {
    pub fn new() -> Self {
        initial_language_from_localstorage_or_navigator_languages()
    }

    pub fn translate(&self, key: &'static str) -> String {
        TRANSLATIONS
            .get(self.code)
            .and_then(|translations| translations.get(key))
            .unwrap_or(&key.to_string())
            .to_string()
    }
}

impl From<&str> for Language {
    fn from(code: &str) -> Self {
        LANGUAGES
            .iter()
            .find(|lang| lang.code == code)
            .unwrap_or(&LANGUAGES[0])
            .clone()
    }
}

impl Default for Language {
    fn default() -> Self {
        LANGUAGES[0]
    }
}

// Translations are generated by the build.rs script
include!(concat!(env!("OUT_DIR"), "/translations.rs"));

/// State of the localization
#[derive(Clone, Copy)]
pub struct LocaleState {
    /// Current language of the website
    pub current_language: Language,
}

impl LocaleState {
    pub fn new() -> Self {
        LocaleState {
            current_language: Language::new(),
        }
    }
}

impl LocaleState {
    pub fn set_current_language(&mut self, language_code: &str) {
        self.current_language = Language::from(language_code);
        set_language_in_localstorage(self.current_language);
    }
}

#[derive(Copy, Clone)]
pub struct LocaleStateSignal(pub RwSignal<LocaleState>);

fn initial_language_from_navigator_languages() -> Option<Language> {
    let languages = web_sys::window().unwrap().navigator().languages().to_vec();
    for raw_language in languages {
        let mut language =
            raw_language.as_string().expect("Language not parseable");
        if language.contains('-') {
            language = language.split_once('-').unwrap().0.to_string();
        }
        if let Some(lang) = Some(Language::from(language.as_str())) {
            return Some(lang);
        }
    }
    None
}

fn initial_language_from_localstorage_or_navigator_languages() -> Language {
    match initial_language_from_localstorage() {
        Some(lang) => lang,
        None => match initial_language_from_navigator_languages() {
            Some(lang) => lang,
            None => Language::default(),
        },
    }
}

fn initial_language_from_localstorage() -> Option<Language> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    match local_storage.get_item("language") {
        Ok(Some(language)) => Some(Language::from(language.as_str())),
        _ => None,
    }
}

pub fn set_language_in_localstorage(lang: Language) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    local_storage.set_item("language", lang.code).unwrap();
}

#[macro_export]
macro_rules! gettext_impl {
    ($cx:ident, $key:expr) => {
        (&use_context::<::i18n::LocaleStateSignal>($cx)
            .unwrap()
            .0
            .get()
            .current_language
            .translate($key))
            .to_string()
    };
}

#[macro_export]
macro_rules! replace_impl {
    ($key:expr, $($replacements:expr),+) => {
        {
            let mut string = $key.to_string();
            $(
                string = string.replacen("{}", $replacements, 1);
            )+
            string
        }
    };
}

/// Macro to translate strings in the website
///
/// Use it like this:
///
/// ```rust,ignore
/// <p>{move || gettext!(cx, "Hello world!")}</p>
/// ```
///
/// You need to wrap in a `move` closure because is the way that Leptos
/// has to know that the string is reactive.
///
/// ## Interpolation
///
/// You can interpolate variables in the string like with `format!()`,
/// but only `{}` interpolations are supported.
///
/// ```rust,ignore
/// <p>{move || gettext!(cx, "{} {}!", "Hello", "world")}</p>
/// ```
#[macro_export]
macro_rules! gettext {
    ($cx:ident, $key:expr) => {
        $crate::gettext_impl!($cx, $key)
    };
    ($cx:ident, $key:expr, $($replacements:expr),+) => {
        $crate::replace_impl!($crate::gettext_impl!($cx, $key), $($replacements),+)
    };
}

/// Macro to generate a closure that returns a translated string
///
/// Convenient wrapper for Leptos interactivity closures.
///
/// Use it like this:
/// ```rust,ignore
/// <p>{move_gettext!(cx, "Hello world!")}</p>
/// ```
///
/// The previous code is the same as:
/// ```rust,ignore
/// <p>{move || gettext!(cx, "Hello world!")}</p>
/// ```
#[macro_export]
macro_rules! move_gettext {
    ($cx:ident, $key:expr) => {
        move||$crate::gettext!($cx, $key)
    };
    ($cx:ident, $key:expr, $($replacements:expr),+) => {
        move||$crate::gettext!($cx, $key, $($replacements),+)
    };
}