use crate::views::builder::header::render_main_builder_header;

struct Nav {
    name: &'static str,
}

const MAIN_NAV: &[Nav] = &[Nav {
    name: "Basic Setup",
}];

const SUB_NAV: &[Nav] = &[
    Nav {
        name: "Choose template",
    },
    Nav { name: "Setup Data" },
];

pub fn render_home_page() -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
          (render_main_builder_header())
        }
        body {
            title {
                "Builder Home"
            }
            div class="flex h-full w-full" {
                div class="border-r border-gray-200 xl:fixed xl:inset-y-0 xl:z-50 xl:flex xl:w-72 xl:flex-col" {
                    div class="flex grow flex-col gap-y-5 overflow-y-auto px-6 ring-1 ring-white/5" {
                        div class="flex h-16 shrink-0 items-center" {
                            img class="h-12 w-auto" src="/assets/images/logo.png" alt="Your Company";
                        }
                        nav class="flex flex-1 flex-col" {
                            ul class="flex flex-1 flex-col gap-y-7" role="list" {
                                li {
                                    ul class="-mx-2 space-y-1" role="list" {
                                        @for nav in MAIN_NAV {
                                          @if nav.name == "Basic Setup" {
                                            li {
                                                a class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 bg-slate-100 text-indigo-500" href="#" {
                                                    "Basic Setup"
                                                }
                                            }
                                          }else{
                                            li {
                                                a class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-500 hover:bg-slate-100 hover:text-indigo-500" href="#" {
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
                div class="w-full xl:pl-72" {
                    main {
                        h1 class="sr-only" {
                            "Account Settings"
                        }
                        header class="border-b border-white/5" {
                            nav class="flex overflow-x-auto border-b border-gray-200 py-4" {
                                ul class="flex min-w-full flex-none gap-x-6 px-4 text-sm font-semibold leading-6 text-gray-500 sm:px-6 lg:px-8" role="list" {
                                    li {
                                        div class="text-indigo-500 cursor-pointer" {
                                            "Choose template"
                                        }
                                    }
                                    li {
                                        div class="hover:text-indigo-500 cursor-pointer" hx-get="builder/contents/data" hx-trigger="click" hx-target="#contents" {
                                            "Setup Data"
                                        }
                                    }
                                    li {
                                        div class="hover:text-indigo-500" href="#" {
                                            "Billing"
                                        }
                                    }
                                    li {
                                        div class="hover:text-indigo-500" href="#" {
                                            "Teams"
                                        }
                                    }
                                    li {
                                        div class="hover:text-indigo-500" href="#" {
                                            "Integrations"
                                        }
                                    }
                                }
                            }

                            main id="contents" class="p-6" hx-get="builder/contents/template" hx-trigger="load" {
                            }
                        }
                    }
                }
            }
        }
    }
}
