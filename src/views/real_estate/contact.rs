use maud::{html, Markup, PreEscaped};

use crate::views::real_estate::shared::render_contact_input;

pub fn render_contact_1() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            setupMarginNavbar("contact");
        </script>
      "#))
      div
        class="flex justify-center items-center py-36" style="background-image: url('https://d1qawt2l8egll1.cloudfront.net/prod/images/230926025644-contact.jpg'); background-color: rgba(0,0,0,0.5); background-blend-mode: overlay;"
      {
        div class="flex justify-center items-center gap-20 px-15 w-full max-w-360 text-white" {
          div class="flex flex-col gap-5" {
            div class="flex flex-col gap-2" {
              div class="font-bold text-xl" {
                "CONTACT US NOW"
              }
              div class="border-t border-t-[#868d9b] w-7" {}
            }
            p class="max-w-md font-bold text-[#868d9b] text-justify" {
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
          }
          div class="flex flex-col justify-center items-center gap-10" {
            div class="flex flex-col gap-5" {
              div class="flex flex-col gap-4" {
                div class="flex gap-7" {
                  (render_contact_input("First name"))
                  (render_contact_input("Last name"))
                }
                div class="flex gap-7" {
                  (render_contact_input("Phone number"))
                  (render_contact_input("Email Address"))
                }
                div {
                  (render_contact_input("Subject of inquiry"))
                }
                textarea placeholder="Your message" class="rounded-md" rows="7" {}
                div class="flex items-center gap-3" {
                  input type="checkbox" ;
                  label {
                    "I have read and agreed to the  Terms and Conditions   and  Privacy Policy"
                  }
                }
              }
              button class="bg-blue-500 hover:bg-blue-400 py-2 rounded-md w-full text-white cursor-pointer" {
                "Submit"
              }
            }
          }
        }
      }
    }
}

pub fn render_contact_2() -> Markup {
    html! {
      (PreEscaped(r#"
      <script type="module">
          import {setupMarginNavbar} from "/assets/js/app/navbar.js";
          setupMarginNavbar("contact");
      </script>
    "#))
      div
        class="flex justify-center items-center py-36" style="background-image: url('https://d1qawt2l8egll1.cloudfront.net/prod/images/230926025644-contact.jpg'); background-color: rgba(0,0,0,0.5); background-blend-mode: overlay;"
      {
        div class="flex justify-center gap-20 px-15 w-full max-w-360 text-white" {
          div class="flex flex-col gap-5" {
            div class="flex flex-col gap-2" {
              div class="font-bold text-xl" {
                "CONTACT US NOW"
              }
              div class="border-t border-t-[#868d9b] w-7" {}
            }
            p class="max-w-md font-bold text-[#868d9b] text-justify" {
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
            div class="rounded-md max-w-md overflow-hidden" {
              img class="w-full h-full" src="/assets/images/real_estate/google-map.webp"  alt="map";
            }
          }
          div class="flex flex-col justify-center items-center gap-10" {
            div class="flex flex-col gap-5" {
              div class="flex flex-col gap-4" {
                div class="flex gap-7" {
                  (render_contact_input("First name"))
                  (render_contact_input("Last name"))
                }
                div class="flex gap-7" {
                  (render_contact_input("Phone number"))
                  (render_contact_input("Email Address"))
                }
                div {
                  (render_contact_input("Subject of inquiry"))
                }
                textarea placeholder="Your message" class="rounded-md" rows="7" {}
                div class="flex items-center gap-3" {
                  input type="checkbox" ;
                  label {
                    "I have read and agreed to the  Terms and Conditions   and  Privacy Policy"
                  }
                }
              }
              button class="bg-blue-500 hover:bg-blue-400 py-2 rounded-md w-full text-white cursor-pointer" {
                "Submit"
              }
            }
          }
        }
      }
    }
}

pub fn render_contact_3() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            setupMarginNavbar("contact");
        </script>
      "#))
      div class="flex flex-col justify-center items-center pb-36" {
        div class="mb-15 w-full h-170" {
          img class="w-full h-full object-cover" src="https://d1qawt2l8egll1.cloudfront.net/prod/images/230926025644-contact.jpg" alt="contact";
        }
        div class="flex justify-center items-center gap-20 px-15 w-full max-w-360" {
          div class="flex flex-col gap-5" {
            div class="flex flex-col gap-2" {
              div class="font-bold text-xl" {
                "CONTACT US NOW"
              }
              div class="border-t border-t-[#868d9b] w-7" {}
            }
            p class="max-w-md font-bold text-[#868d9b] text-justify" {
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
            div class="rounded-md max-w-md overflow-hidden" {
              img class="w-full h-full" src="/assets/images/real_estate/google-map.webp"  alt="map";
            }
          }
          div class="flex flex-col justify-center items-center gap-10" {
            div class="flex flex-col gap-5" {
              div class="flex flex-col gap-4" {
                div class="flex gap-7" {
                  (render_contact_input("First name"))
                  (render_contact_input("Last name"))
                }
                div class="flex gap-7" {
                  (render_contact_input("Phone number"))
                  (render_contact_input("Email Address"))
                }
                div {
                  (render_contact_input("Subject of inquiry"))
                }
                textarea placeholder="Your message" class="rounded-md" rows="7" {}
                div class="flex items-center gap-3" {
                  input type="checkbox" ;
                  label {
                    "I have read and agreed to the  Terms and Conditions   and  Privacy Policy"
                  }
                }
              }
              button class="bg-blue-500 hover:bg-blue-400 py-2 rounded-md w-full text-white cursor-pointer" {
                "Submit"
              }
            }
          }
        }
      }
    }
}

pub fn render_contact_4() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {setupMarginNavbar} from "/assets/js/app/navbar.js";
            setupMarginNavbar("contact");
        </script>
      "#))
      div class="flex flex-col justify-center items-center pb-36" {
        div class="flex justify-center items-center mb-15 w-full h-170" style="background-image: url('https://d1qawt2l8egll1.cloudfront.net/prod/images/230926025644-contact.jpg'); background-color: rgba(0,0,0,0.5); background-blend-mode: overlay; background-position: center;" {
          span class="font-bold text-5xl text-white" { "Contact us now" }
        }
        div class="flex flex-col justify-center items-center gap-20 px-15 w-full max-w-360" {
          div class="flex flex-col gap-15 max-w-200" {
            div class="rounded-md w-full overflow-hidden" {
              img class="w-full h-full" src="/assets/images/real_estate/google-map.webp"  alt="map";
            }
            p class="font-bold text-[#868d9b] text-justify" {
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
          }
          div class="flex flex-col justify-center items-center gap-10 w-full max-w-200" {
            div class="flex flex-col gap-5 w-full" {
              div class="flex flex-col gap-4" {
                div class="flex gap-7" {
                  (render_contact_input("First name"))
                  (render_contact_input("Last name"))
                }
                div class="flex gap-7" {
                  (render_contact_input("Phone number"))
                  (render_contact_input("Email Address"))
                }
                div {
                  (render_contact_input("Subject of inquiry"))
                }
                textarea placeholder="Your message" class="rounded-md" rows="7" {}
                div class="flex items-center gap-3" {
                  input type="checkbox" ;
                  label {
                    "I have read and agreed to the  Terms and Conditions   and  Privacy Policy"
                  }
                }
              }
              button class="bg-blue-500 hover:bg-blue-400 py-2 rounded-md w-full text-white cursor-pointer" {
                "Submit"
              }
            }
          }
        }
      }
    }
}
