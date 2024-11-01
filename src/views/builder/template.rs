pub fn render_choose_template() -> String {
    vy::render! {
      <section>
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
      </section>
    }
}
