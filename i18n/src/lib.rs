use fluent_templates::{
    fluent_bundle::FluentValue, loader::langid, static_loader,
    LanguageIdentifier, Loader,
};
use leptos::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct Language {
    pub id: LanguageIdentifier,
    pub name: &'static str,
}

// Exposes LANGUAGES as a public static variable
include!(concat!(env!("OUT_DIR"), "/languages.rs"));

static_loader! {
    // Declare our `StaticLoader` named `LOCALES`.
    static LOCALES = {
        // The directory of localisations and fluent resources.
        locales: "../i18n/locales",
        // The language to falback on if something is not present.
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Language {
    pub fn translate(&self, key: &'static str) -> String {
        LOCALES.lookup(&self.id, key).unwrap()
    }

    pub fn translate_with_args(
        &self,
        key: &'static str,
        args: &HashMap<String, FluentValue<'_>>,
    ) -> String {
        LOCALES.lookup_with_args(&self.id, key, args).unwrap()
    }
}

impl Default for Language {
    fn default() -> Self {
        LANGUAGES[0].clone()
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match LanguageIdentifier::from_str(code) {
            Ok(target_lang) => match LANGUAGES
                .iter()
                .find(|lang| lang.id.matches(&target_lang, false, false))
            {
                Some(lang) => Ok(lang.clone()),
                None => {
                    let mut lazy_target_lang = target_lang.clone();
                    lazy_target_lang.region = None;
                    match LANGUAGES.iter().find(|lang| {
                        lang.id.matches(&lazy_target_lang, true, true)
                    }) {
                        Some(lang) => Ok(lang.clone()),
                        None => Err(()),
                    }
                }
            },
            Err(_) => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct LocaleSignal(pub RwSignal<Language>);

/// Macro to translate strings in the website
///
/// Use it like this:
///
/// ```rust,ignore
/// <p>{move || tr!("hello-world")}</p>
/// ```
///
/// You need to wrap in a `move` closure because is the way that Leptos
/// has to know that the string is reactive.
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        ((&expect_context::<::i18n::LocaleSignal>().0)().translate($key))
            .to_string()
    };
    ($key:expr, $args:expr) => {
        ((&expect_context::<::i18n::LocaleSignal>().0)()
            .translate_with_args($key, $args))
        .to_string()
    };
}

/// Macro to generate a closure that returns a translated string
///
/// Convenient wrapper for Leptos interactivity closures.
///
/// Use it like this:
/// ```rust,ignore
/// <p>{move_tr!("hello-world")}</p>
/// ```
///
/// The previous code is the same as:
/// ```rust,ignore
/// <p>{move || tr!("hello-world")}</p>
/// ```
#[macro_export]
macro_rules! move_tr {
    ($key:expr) => {
        move || $crate::tr!($key)
    };
    ($key:expr, $args:expr) => {
        move || $crate::tr!($key, $args)
    };
}
