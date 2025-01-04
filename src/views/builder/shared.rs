pub struct MainNav {
    name: &'static str,
}

pub struct SubNav {
    name: &'static str,
    url: &'static str,
    push_url: &'static str,
}

pub const MAIN_NAV: &[MainNav] = &[MainNav {
    name: "Basic Setup",
}];

pub const SUB_NAV: &[SubNav] = &[
    SubNav {
        name: "Create website",
        url: "/builder/contents/website",
        push_url: "/builder/create-website",
    },
    SubNav {
        name: "Choose template",
        url: "/builder/contents/template",
        push_url: "/builder/select-template",
    },
    SubNav {
        name: "Setup data",
        url: "/builder/contents/data",
        push_url: "/builder/setup-data",
    },
];

pub fn render_main_nav(
    nav_menu: &[MainNav],
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
    nav_menu: &[SubNav],
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
                                hx-push-url=(nav.push_url)
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
