mod button;
pub mod language_selector;
mod third_party_extensions;

use crate::header::{
    nav::{
        button::{HeaderMenuButton, HeaderMenuLink},
        language_selector::LanguageSelector,
        third_party_extensions::ThirdPartyExtensions,
    },
    HeaderStateSignal,
};
use crate::svg::SVGDef;
use i18n::move_tr;
use leptos::*;
use simple_icons_macros::simple_icon_svg_path;

// UNPKG icon is not available in simple-icons
// Requested at https://github.com/simple-icons/simple-icons/issues/8475
static UNPKG_ICON_SVG_PATH: &str =
    "M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm0 2.002A10.009 10.009 0 0 1 22.009 12.01 10.009 10.009 0 0 1 12 22.02 10.009 10.009 0 0 1 1.991 12.01 10.009 10.009 0 0 1 12 2.003zm2.141 3.928-.031 4.047-.034 4.046c-.73 2.109-3.414 2.109-3.992.007-.075-.31-.118-1.798-.118-4.282v-3.81H6.84l.043 4.142c.054 4.475.085 4.742.662 5.91.353.684 1.21 1.444 1.97 1.723 1.391.524 3.682.524 5.073 0 .75-.279 1.553-.976 1.938-1.681.589-1.103.654-1.627.654-6.016l.01-4.014-3.04-.065z";

static LEGAL_DISCLAIMER_SVG_PATH: &str = "m23.9 9.7-3.54-7.89-.005-.01a.542.542 0 0 0-.041-.076l-.014-.018a.533.533 0 0 0-.122-.122l-.015-.011a.528.528 0 0 0-.08-.044l-.024-.009a.527.527 0 0 0-.067-.02l-.028-.007a.524.524 0 0 0-.096-.01h-6.85c-1.02-1.52-1.02-1.54-2 0h-6.86a.543.543 0 0 0-.096.01l-.028.007a.516.516 0 0 0-.067.02l-.024.01a.537.537 0 0 0-.08.043l-.015.011a.51.51 0 0 0-.057.047l-.02.02a.543.543 0 0 0-.045.055l-.014.018a.522.522 0 0 0-.041.075l-.005.01v.001L.116 9.72a.531.531 0 0 0-.096.304c0 2.28 1.86 4.14 4.14 4.14s4.14-1.86 4.14-4.14a.53.53 0 0 0-.096-.304l-3.25-6.37 6.07-.023v18.2c-2.55.294-7.01.381-7 2.5h16c0-2.03-4.48-2.27-7-2.5v-18.1l5.69-.02-2.92 6.49c0 .002 0 .003-.002.005l-.006.018a.545.545 0 0 0-.023.075l-.005.02a.524.524 0 0 0-.01.092v.008c0 2.28 1.86 4.14 4.14 4.14 2.28 0 4.14-1.86 4.14-4.14a.528.528 0 0 0-.12-.332z";

/// Header menu
///
/// Menu with:
///
/// - Links to the different resources of the Simple Icons ecosystem built by [`HeaderMenuLink`].
/// - Button to open the menu on small screens built by [`HeaderMenuButton`].
/// - Button to open third party extensions table built by [`HeaderMenuButton`].
#[component]
pub fn HeaderMenu() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <HeaderMenuLink
                    title=move_tr!("main-repository")
                    href="https://github.com/simple-icons/simple-icons"
                    svg_path=simple_icon_svg_path!("github")
                />
                <HeaderMenuLink
                    title=move || "npm".to_string()
                    href="https://www.npmjs.com/package/simple-icons"
                    svg_path=simple_icon_svg_path!("npm")
                />
                <HeaderMenuLink
                    title=move || "Packagist".to_string()
                    href="https://packagist.org/packages/simple-icons/simple-icons"
                    svg_path=simple_icon_svg_path!("packagist")
                />
                <HeaderMenuLink
                    title=move_tr!("jsdelivr")
                    href="https://www.jsdelivr.com/package/npm/simple-icons"
                    svg_path=simple_icon_svg_path!("jsdelivr")
                />
                <HeaderMenuLink
                    title=move_tr!("unpkg")
                    href="https://unpkg.com/browse/simple-icons/"
                    svg_path=UNPKG_ICON_SVG_PATH
                />
                <HeaderMenuLink
                    title=move_tr!("discord")
                    href="https://discord.gg/vUXFa7t5xJ"
                    svg_path=simple_icon_svg_path!("discord")
                />
                <HeaderMenuLink
                    title=move_tr!("open-collective")
                    href="https://opencollective.com/simple-icons"
                    svg_path=simple_icon_svg_path!("opencollective")
                />
                <HeaderMenuLink
                    title=move_tr!("legal-disclaimer")
                    href="https://github.com/simple-icons/simple-icons/blob/master/DISCLAIMER.md"
                    svg_path=LEGAL_DISCLAIMER_SVG_PATH
                />
                <ThirdPartyExtensions/>
                <LanguageSelector/>
            </ul>
            <ul>
                <HeaderMenuBurgerButton/>
                <HeaderMenuCloseButton/>
            </ul>
        </nav>
    }
}

/// Header menu burger button
///
/// Button to open the menu on mobile devices
#[component]
pub fn HeaderMenuBurgerButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;

    view! {
        <HeaderMenuButton
            on:click=move |_| header_state.update(|state| state.toggle_menu())
            title=move_tr!("open-menu")
            svg_path="M1.412 3.53A1.412 1.412 0 0 0 0 4.94a1.412 1.412 0 0 0 1.412 1.412h21.176A1.412 1.412 0 0 0 24 4.94a1.412 1.412 0 0 0-1.412-1.412Zm0 7.058A1.412 1.412 0 0 0 0 12a1.412 1.412 0 0 0 1.412 1.412h21.176A1.412 1.412 0 0 0 24 12a1.412 1.412 0 0 0-1.412-1.412Zm0 7.06A1.412 1.412 0 0 0 0 19.057a1.412 1.412 0 0 0 1.412 1.413h21.176A1.412 1.412 0 0 0 24 19.059a1.412 1.412 0 0 0-1.412-1.412Z"
            class=move || if header_state().menu_open { "hidden" } else { "block lg:hidden" }
        />
    }
}

/// Button to close the menu on mobile devices
#[component]
pub fn HeaderMenuCloseButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;

    view! {
        <HeaderMenuButton
            title=move_tr!("close-menu")
            class=move || if header_state().menu_open { "block" } else { "hidden" }
            svg_path=SVGDef::Cross.d()
            on:click=move |_| header_state.update(|state| state.toggle_menu())
        />
    }
}
