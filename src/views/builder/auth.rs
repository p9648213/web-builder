use crate::views::builder::header::render_main_builder_header;

//..........................................................
//.LLL...........OOOOOO........GGGGGG.....III..NNNN....NNN..
//.LLL.........OOOOOOOOOO....GGGGGGGGGG...III..NNNN....NNN..
//.LLL........OOOOOOOOOOOO...GGGGGGGGGGG..III..NNNNN...NNN..
//.LLL........OOOO....OOOO..GGGG....GGGG..III..NNNNN...NNN..
//.LLL........OOO......OOO..GGG......GG...III..NNNNNN..NNN..
//.LLL.......LOOO......OOOOOGGG...........III..NNNNNNN.NNN..
//.LLL.......LOOO......OOOOOGGG...GGGGGG..III..NNN.NNN.NNN..
//.LLL.......LOOO......OOOOOGGG...GGGGGG..III..NNN.NNNNNNN..
//.LLL........OOO......OOO..GGG...GGGGGG..III..NNN..NNNNNN..
//.LLL........OOOO....OOOO..GGGG.....GGG..III..NNN..NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO...GGGGGGGGGGG..III..NNN...NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG...III..NNN....NNNN..
//.LLLLLLLLLL....OOOOOO........GGGGGG.....III..NNN....NNNN..
//..........................................................

pub fn render_login_page(authenticity_token: String) -> String {
    vy::render! {
        <!DOCTYPE html>
        <html lang="en">
          <head>
            {render_main_builder_header()}
            <title>"Login"</title>
          </head>
          <body class="min-h-screen" hx-boost="true" hx-history="false">
            <div
              class="flex h-screen items-center justify-center px-4 py-12 sm:px-6 lg:px-8"
            >
              <div class="w-full max-w-sm space-y-10">
                <div>
                  <h2
                    class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900"
                  >
                    "Sign in to your account"
                  </h2>
                </div>
                <form
                  class="space-y-6"
                  id="login-form"
                  hx-post="/builder/auth/login"
                  hx-swap="none"
                  hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
                  hx-disabled-elt="find button"
                >
                  <input
                    type="hidden"
                    name="authenticity_token"
                    value={authenticity_token}
                  />
                  <div class="relative -space-y-px rounded-md shadow-sm">
                    <div
                      class="pointer-events-none absolute inset-0 z-10 rounded-md ring-1 ring-inset ring-gray-300"
                    ></div>
                    <div>
                      <label for="email-address" class="sr-only">"Email address"</label>
                      <input
                        id="email-address"
                        name="email"
                        type="email"
                        autocomplete="email"
                        class="relative block w-full rounded-t-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        placeholder="Email address"
                      />
                    </div>
                    <div>
                      <label for="password" class="sr-only">"Password"</label>
                      <input
                        id="password"
                        name="password"
                        type="password"
                        autocomplete="current-password"
                        class="relative block w-full rounded-b-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        placeholder="Password"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                      "Sign in"
                    </button>
                  </div>
                </form>

                <p class="text-center text-sm leading-6 text-gray-500">
                  "Not a member?"
                  <a
                    id="register-link"
                    href="/builder/auth/register"
                    class="font-semibold text-indigo-600 hover:text-indigo-500"
                    >" Sign Up"</a
                  >
                </p>
              </div>
            </div>
            <div id="toast"></div>
          </body>
        </html>
    }
}

//.............................................................................................
//.RRRRRRRRR....EEEEEEEEEE.....GGGGGG.....III....SSSSSS....TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRR....
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGG...III..SSSSSSSSS...TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGGG..III..SSSSSSSSSS..TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRR.....RRR..EEE.........GGGG....GGGG..III..SSS...SSSS......TTT....EEE.........RRR.....RRR..
//.RRR.....RRR..EEE.........GGG......GG...III..SSSS............TTT....EEE.........RRR.....RRR..
//.RRRRRRRRRRR..EEEEEEEEEE.EGGG...........III..SSSSSSS.........TTT....EEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRR...EEEEEEEEEE.EGGG...GGGGGG..III...SSSSSSSS.......TTT....EEEEEEEEEE..RRRRRRRRRR...
//.RRRRRRRR.....EEEEEEEEEE.EGGG...GGGGGG..III.....SSSSSSS......TTT....EEEEEEEEEE..RRRRRRRR.....
//.RRR..RRRR....EEE.........GGG...GGGGGG..III.........SSSS.....TTT....EEE.........RRR..RRRR....
//.RRR...RRRR...EEE.........GGGG.....GGG..III.ISSS....SSSS.....TTT....EEE.........RRR...RRRR...
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGGG..III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGG...III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR.....RRRR.EEEEEEEEEEE....GGGGGG.....III....SSSSSS........TTT....EEEEEEEEEEE.RRR.....RRR..
//.............................................................................................

pub fn render_register_page(authenticity_token: String) -> String {
    vy::render! {
        <!DOCTYPE html>
        <html lang="en">
          <head>
            {render_main_builder_header()}
            <title>"Register"</title>
          </head>
          <body class="min-h-screen" hx-boost="true" hx-history="false">
            <div
              class="flex h-screen items-center justify-center px-4 py-12 sm:px-6 lg:px-8"
            >
              <div class="w-full max-w-sm space-y-10">
                <div>
                  <h2
                    class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-900"
                  >
                    "Register account"
                  </h2>
                </div>
                <form
                  class="space-y-6"
                  id="login-form"
                  hx-post="/builder/auth/register"
                  hx-swap="none"
                  hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
                  hx-disabled-elt="find button"
                >
                  <input
                    type="hidden"
                    name="authenticity_token"
                    value={authenticity_token}
                  />
                  <div class="relative -space-y-px rounded-md shadow-sm">
                    <div
                      class="pointer-events-none absolute inset-0 z-10 rounded-md ring-1 ring-inset ring-gray-300"
                    ></div>
                    <div>
                      <label for="username" class="sr-only">"Username"</label>
                      <input
                        id="username"
                        name="username"
                        type="text"
                        autocomplete="on"
                        class="relative block w-full rounded-t-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        placeholder="Username"
                      />
                    </div>
                    <div>
                      <label for="email-address" class="sr-only">"Email address"</label>
                      <input
                        id="email-address"
                        name="email"
                        type="email"
                        autocomplete="email"
                        class="relative block w-full rounded-t-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        placeholder="Email address"
                      />
                    </div>
                    <div>
                      <label for="password" class="sr-only">"Password"</label>
                      <input
                        id="password"
                        name="password"
                        type="password"
                        autocomplete="current-password"
                        class="relative block w-full rounded-b-md border-0 py-1.5 text-gray-900 ring-1 ring-inset ring-gray-100 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        placeholder="Password"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                      "Register"
                    </button>
                  </div>
                </form>

                <p class="text-center text-sm leading-6 text-gray-500">
                  "Already have an account?"
                  <a
                    id="register-link"
                    href="/builder/auth/login"
                    class="font-semibold text-indigo-600 hover:text-indigo-500"
                    >" Login"
                  </a>
                </p>
              </div>
            </div>
            <div id="toast"></div>
          </body>
        </html>
    }
}
