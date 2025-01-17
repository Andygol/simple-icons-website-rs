use crate::copy::copy_setting_copied_transition_in_element;
use i18n::move_tr;
use i18n::Language;
use leptos::{ev::MouseEvent, *};
use std::collections::HashMap;
use types::SimpleIcon;
use web_sys;

pub fn get_icon_localized_title(
    icon: &'static SimpleIcon,
    language: &Language,
) -> &'static str {
    if let Some(aliases) = icon.aliases {
        if let Some(loc) = aliases.loc {
            let current_lang_region = language.id.to_string();
            let current_lang = language.id.language.to_string();

            for (lang, loc_title) in loc {
                if *lang == current_lang_region {
                    return loc_title;
                }
            }

            for (lang, loc_title) in loc {
                let mut loc_language = lang.to_string();
                if loc_language.contains('-') {
                    loc_language =
                        loc_language.split('-').next().unwrap().to_string();
                }
                if loc_language == current_lang {
                    return loc_title;
                }
            }
        }
    }
    icon.title
}

/// Icon grid item title
#[component]
pub fn IconGridItemTitle(
    /// Brand title
    brand_name: Memo<&'static str>,
    /// Slug
    slug: &'static str,
) -> impl IntoView {
    let container_title = move_tr!("copy-icon-slug", &{
        let mut map = HashMap::new();
        map.insert("icon".to_string(), brand_name().into());
        map.insert("slug".to_string(), slug.into());
        map
    });
    view! {
        <h2
            title=container_title
            tabindex=0
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                spawn_local(copy_setting_copied_transition_in_element(slug.to_string(), target));
            }
        >

            {brand_name}
        </h2>
    }
}
