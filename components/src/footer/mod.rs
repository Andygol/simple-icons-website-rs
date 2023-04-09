//! App footer

mod about;

use about::*;
use i18n::move_gettext;
use leptos::*;
use macros::simple_icon_svg_path;

static TWITTER_ICON_SVG_PATH: &str = simple_icon_svg_path!("twitter");

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="flex flex-col justify-between py-8 text-sm">
            <ReportProblems/>
            <div class="flex flex-row justify-between">
                <About/>
                <TwitterButton/>
            </div>
            <a
                class=concat!(
                    "w-full text-center mt-6 hover:underline",
                    " text-[var(--link-color)] hover:text-[var(--link-color-hover)]",
                )
                href="https://github.com/simple-icons/simple-icons-website"
            >
                {move_gettext!(cx, "Made with ❤️ on GitHub")}
            </a>
        </footer>
    }
}

#[component]
fn ReportLink(
    cx: Scope,
    /// Link URL
    href: &'static str,
    /// Link content
    children: Children,
) -> impl IntoView {
    view! { cx,
        <a
            class=concat!(
                // Light theme colors
                "text-[#00e] hover:text-[#3434ee]",
                " visited:text-[#551a8b]",
                // Dark theme colors
                " dark:text-[#227fff] dark:hover:text-[#3c8eff]",
                " dark:visited:text-[#a990bd]",
            )
            href=href
        >
            {children(cx)}
        </a>
    }
}

#[component]
pub fn ReportProblems(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col py-8">
            <p>
                {move_gettext!(cx, "Icon missing?")}
                {" "}
                <ReportLink
                    href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml"
                >
                    {move_gettext!(cx, "Submit a request")}
                </ReportLink>
            </p>
            <p>
                {move_gettext!(cx, "Icon outdated?")}
                {" "}
                <ReportLink
                    href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml"
                >
                    {move_gettext!(cx, "Report outdated icon")}
                </ReportLink>
            </p>
        </div>
    }
}

#[component]
pub fn TwitterButton(cx: Scope) -> impl IntoView {
    view! { cx,
        <a
            class=concat!(
                "flex flex-row items-center h-0 my-auto align-middle bg-[#1DA1F2]",
                " text-white rounded-md px-3 py-5"
            )
            rel="noopener"
            role="button"
            target="_blank"
            href="https://twitter.com/intent/tweet?url=https://simpleicons.org&amp;text=Simple%20Icons%3A%20free%20SVG%20icons%20for%20popular%20brands."
        >
                <svg fill="white" class="h-4 mr-3" role="img" viewBox="0 0 24 24">
                    <path d=TWITTER_ICON_SVG_PATH/>
                </svg>
                <span>{move_gettext!(cx, "Share this on Twitter!")}</span>
        </a>
    }
}