use crate::views::builder::header::render_main_builder_header;

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
                              class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 bg-slate-100 text-indigo-500 hover:bg-slate-100 hover:text-indigo-500"
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
                          <a href="#" class="text-indigo-500">"Choose your template"</a>
                        </li>
                        <li>
                          <a href="#" class="hover:text-indigo-500">"Notifications"</a>
                        </li>
                        <li>
                          <a href="#" class="hover:text-indigo-500">"Billing"</a>
                        </li>
                        <li>
                          <a href="#" class="hover:text-indigo-500">"Teams"</a>
                        </li>
                        <li>
                          <a href="#" class="hover:text-indigo-500">"Integrations"</a>
                        </li>
                      </ul>
                    </nav>
                    <div class="p-6">
                      <div>
                        <div
                          class="relative my-6 flex w-96 flex-col rounded-lg border border-slate-200 bg-white shadow-sm"
                        >
                          <div
                            class="relative m-2.5 h-56 overflow-hidden rounded-md text-white"
                          >
                            <img
                              class="h-full w-full object-cover"
                              src="/assets//images/real_estate.webp"
                              alt="card-image"
                            />
                          </div>
                          <div class="p-4">
                            <div class="mb-2 flex w-full items-center justify-between">
                              <h6 class="text-xl font-semibold text-slate-800">
                                "Real Estate"
                              </h6>
                              <a
                                class="group flex items-center gap-2 text-indigo-500"
                                href="/app/demo/realestate"
                              >
                                <div class="group">"Preview"</div>
                                <svg
                                  xmlns="http://www.w3.org/2000/svg"
                                  class="flex-0 h-0 w-0 translate-y-[1px] transition-all group-hover:h-4 group-hover:w-4"
                                  fill="none"
                                  viewBox="0 0 24 24"
                                  stroke="currentColor"
                                >
                                  <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M14 5l7 7m0 0l-7 7m7-7H3"
                                  />
                                </svg>
                              </a>
                            </div>

                            <p class="text-slate-600">
                              "Lorem ipsum dolor sit amet consectetur adipisicing elit.
                              Consequatur molestiae eum aperiam dicta perferendis."
                            </p>
                          </div>

                          <div class="mt-2 px-4 pb-4 pt-0">
                            <button
                              class="w-full rounded-md border border-transparent bg-indigo-500 px-4 py-2 text-center text-sm font-semibold text-white shadow-md transition-all hover:bg-indigo-400 hover:shadow-lg focus:bg-indigo-400 focus:shadow-none active:bg-indigo-400 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
                              type="button"
                            >
                              "Select"
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  </header>
                </main>
              </div>
            </div>
          </body>
        </html>
    }
}
