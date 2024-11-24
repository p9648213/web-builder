use crate::{
    models::website::Website,
    views::builder::home::{render_sub_nav, SUB_NAV},
};

pub fn render_choose_template(website: Option<Website>) -> maud::Markup {
    maud::html! {
        @if let Some(_) = website {
            section class="choose-template" {
                div class="relative flex flex-col border-slate-200 bg-white shadow-sm my-6 border rounded-lg w-96" {
                    div class="relative m-2.5 rounded-md h-56 text-white overflow-hidden" {
                        img class="w-full h-full object-cover" src="/assets//images/real_estate.webp" alt="card-image";
                    }
                    div class="p-4" {
                        div class="flex justify-between items-center mb-2 w-full" {
                            h6 class="font-semibold text-slate-800 text-xl" {
                                "Real Estate"
                            }
                            a class="flex items-center gap-2 text-indigo-500 group" href="/app/demo/realestate" {
                                div class="group" {
                                    "Preview"
                                }
                                svg class="group-hover:h-4 group-hover:w-4 flex-0 w-0 h-0 transition-all translate-y-[1px]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" {}
                                }
                            }
                        }
                        p class="text-slate-600" {
                            "Lorem ipsum dolor sit amet consectetur adipisicing elit.Consequatur molestiae eum aperiam dicta perferendis."
                        }
                    }
                    div class="mt-2 px-4 pt-0 pb-4" {
                        button class="bg-indigo-500 hover:bg-indigo-400 focus:bg-indigo-400 active:bg-indigo-400 disabled:opacity-50 shadow-md hover:shadow-lg focus:shadow-none active:shadow-none disabled:shadow-none px-4 py-2 border border-transparent rounded-md w-full font-semibold text-center text-sm text-white transition-all disabled:pointer-events-none" type="button" {
                            "Select"
                        }
                    }
                }
            }
        } @else {
            div {
                "You don't have any website"
            }
        }
        (render_sub_nav(SUB_NAV, "Choose template", Some("outerHTML")))
    }
}
