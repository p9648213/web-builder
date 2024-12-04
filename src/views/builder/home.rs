use crate::views::builder::head::render_main_builder_header;

pub struct Nav {
    name: &'static str,
    url: &'static str,
}

pub const MAIN_NAV: &[Nav] = &[Nav {
    name: "Basic Setup",
    url: "/builder/contents/basic-setup",
}];

pub const SUB_NAV: &[Nav] = &[
    Nav {
        name: "Create website",
        url: "/builder/contents/website",
    },
    Nav {
        name: "Choose template",
        url: "/builder/contents/template",
    },
    Nav {
        name: "Setup data",
        url: "/builder/contents/data",
    },
];

pub fn render_home_page() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
          (render_main_builder_header())
        }
        body hx-boost="true" {
            title {
                "Builder Home"
            }
            div class="flex w-full h-full" {
                div class="xl:z-50 xl:fixed xl:inset-y-0 xl:flex xl:flex-col border-gray-200 border-r xl:w-72" {
                    div class="flex flex-col gap-y-5 px-6 ring-1 ring-white/5 overflow-y-auto grow" {
                        div class="flex items-center h-16 shrink-0" {
                            img class="w-auto h-12" src="/assets/images/logo.png" alt="Your Company";
                        }
                        (render_main_nav(MAIN_NAV, "Basic Setup", None))
                    }
                }
                div class="xl:pl-72 w-full" {
                    main {
                        h1 class="sr-only" {
                            "Account Settings"
                        }
                        header class="border-white/5 border-b" {
                            nav id="sub-nav" {}
                            main id="contents" class="p-6" hx-get="/builder/contents/website" hx-trigger="load" {}
                        }
                    }
                }
            }
            div id="toast" {}
        }
    }
}

pub fn render_main_nav(
    nav_menu: &[Nav],
    highlight_item: &str,
    oob_swap: Option<&str>,
) -> maud::Markup {
    maud::html! {
        nav id="main-nav" hx-swap-oob=[oob_swap] class="flex flex-col flex-1" {
            ul class="flex flex-col flex-1 gap-y-7" role="list" {
                li {
                    ul class="space-y-1 -mx-2" role="list" {
                        @for nav in nav_menu {
                            @if nav.name == highlight_item {
                                li {
                                    div
                                        class="flex gap-x-3 bg-slate-100 p-2 rounded-md font-semibold text-indigo-500 text-sm leading-6 cursor-pointer group"
                                        href="#"
                                    {
                                        (nav.name)
                                    }
                                }
                            } @else {
                                li {
                                    div
                                        class="flex gap-x-3 hover:bg-slate-100 p-2 rounded-md font-semibold text-gray-500 text-sm hover:text-indigo-500 leading-6 cursor-pointer group"
                                        href="#"
                                    {
                                        (nav.name)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_sub_nav(
    nav_menu: &[Nav],
    highlight_item: &str,
    oob_swap: Option<&str>,
) -> maud::Markup {
    maud::html! {
        nav id="sub-nav" hx-swap-oob=[oob_swap] class="flex border-gray-200 py-4 border-b overflow-x-auto" {
            ul class="flex flex-none gap-x-6 px-4 sm:px-6 lg:px-8 min-w-full font-semibold text-gray-500 text-sm leading-6" role="list" {
                @for nav in nav_menu {
                    @if nav.name == highlight_item {
                        li {
                            div class="text-indigo-500 cursor-pointer" {
                                (nav.name)
                            }
                        }
                    } @else {
                        li {
                            div
                                class="hover:text-indigo-500 cursor-pointer"
                                hx-get=(nav.url)
                                hx-trigger="click"
                                hx-target="#contents"
                            {
                                (nav.name)
                            }
                        }
                    }
                }
            }
        }
    }
}
