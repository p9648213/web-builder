use vy::PreEscaped;

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

pub fn render_home_page() -> String {
    vy::render! {
        <!DOCTYPE html>
        <html class="h-full bg-white" lang="en">
          <head>
            {render_main_builder_header()}
            <title>"Builder Home"</title>
          </head>
          <body hx-boost="true" hx-history="false" class="h-full">
            <div class="flex h-full w-full">
              <div
                class="border-r border-gray-200 xl:fixed xl:inset-y-0 xl:z-50 xl:flex xl:w-72 xl:flex-col"
              >
                <div
                  class="flex grow flex-col gap-y-5 overflow-y-auto px-6 ring-1 ring-white/5"
                >
                  <div class="flex h-16 shrink-0 items-center">
                    <img
                      class="h-12 w-auto"
                      src="/assets/images/logo.png"
                      alt="Your Company"
                    />
                  </div>
                  <nav class="flex flex-1 flex-col">
                    <ul role="list" class="flex flex-1 flex-col gap-y-7">
                      <li>
                        <ul role="list" class="-mx-2 space-y-1">
                          <li>
                            <a
                              href="#"
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 bg-slate-100 text-indigo-500"
                            >
                              "Basic Setup"
                            </a>
                          </li>
                          <li>
                            <a
                              href="#"
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-500 hover:bg-slate-100 hover:text-indigo-500"
                            >
                              "Deployments"
                            </a>
                          </li>
                          <li>
                            <a
                              href="#"
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-500 hover:bg-slate-100 hover:text-indigo-500"
                            >
                              "Activity"
                            </a>
                          </li>
                          <li>
                            <a
                              href="#"
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-500 hover:bg-slate-100 hover:text-indigo-500"
                            >
                              "Usage"
                            </a>
                          </li>
                          <li>
                            <a
                              href="#"
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-500 hover:bg-slate-100 hover:text-indigo-500"
                            >
                              "Settings"
                            </a>
                          </li>
                        </ul>
                      </li>
                    </ul>
                  </nav>
                </div>
              </div>

              <div class="w-full xl:pl-72">
                <main>
                  <h1 class="sr-only">"Account Settings"</h1>
                  <header class="border-b border-white/5">
                    <nav class="flex overflow-x-auto border-b border-gray-200 py-4">
                      <ul
                        role="list"
                        class="flex min-w-full flex-none gap-x-6 px-4 text-sm font-semibold leading-6 text-gray-500 sm:px-6 lg:px-8"
                      >
                        <li>
                          <div
                            class="text-indigo-500 cursor-pointer">
                              "Choose template"
                          </div>
                        </li>
                        <li>
                          <div
                            hx-get="builder/contents/data"
                            hx-trigger="click"
                            hx-target="#contents"
                            class="hover:text-indigo-500 cursor-pointer">"Setup Data"</div>
                        </li>
                        <li>
                          <div href="#" class="hover:text-indigo-500">"Billing"</div>
                        </li>
                        <li>
                          <div href="#" class="hover:text-indigo-500">"Teams"</div>
                        </li>
                        <li>
                          <div href="#" class="hover:text-indigo-500">"Integrations"</div>
                        </li>
                      </ul>
                    </nav>
                    <main id="contents" class="p-6" hx-get="builder/contents/template" hx-trigger="load"></main>
                  </header>
                </main>
              </div>
            </div>
          </body>
        </html>
    }
}

pub fn render_sub_nav<'a>(active_item: &str) -> PreEscaped<&'a str> {
    match active_item {
        "template" => {
            vy::lazy! {
              <li>
                <div
                  class="text-indigo-500 cursor-pointer">
                    "Choose template"
                </div>
              </li>
              <li>
                <div
                  hx-get="builder/contents/data"
                  hx-trigger="click"
                  hx-target="#contents"
                  class="hover:text-indigo-500 cursor-pointer">"Setup Data"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Billing"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Teams"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Integrations"</div>
              </li>
            }
        }
        "data" => {
            vy::lazy! {
              <li>
                <div
                  hx-get="builder/contents/template"
                  hx-trigger="click"
                  hx-target="#contents"
                  class="hover:text-indigo-500 cursor-pointer">
                    "Choose template"
                </div>
              </li>
              <li>
                <div class="text-indigo-500 cursor-pointer">"Setup Data"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Billing"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Teams"</div>
              </li>
              <li>
                <div href="#" class="hover:text-indigo-500">"Integrations"</div>
              </li>
            }
        }
        _ => vy::lazy! {
          ""
        },
    }
}
