use maud::{html, Markup, PreEscaped};

use crate::{
    models::rso_data::{SearchProperty, TextOrNum},
    views::icons::{
        bath_icon, bed_icon, buit_size_icon, certificate_icon, drop_down_icon, email_icon,
        facebook_icon_color, hamburger_icon, instagram_icon_color, linkedin_icon_color,
        location_icon, mail_icon, phone_icon, twitter_icon_color, youtube_icon_color,
    },
};

//.............................................................................
//.NNNN....NNN.....AAAA...AAAV.....VVVV.VBBBBBBBB.......AAAA......ARRRRRRRR....
//.NNNN....NNN.....AAAAA...AAV.....VVVV.VBBBBBBBBB......AAAA......ARRRRRRRRR...
//.NNNNN...NNN.....AAAAA...AAVV....VVV..VBBBBBBBBBB....AAAAAA.....ARRRRRRRRRR..
//.NNNNN...NNN....AAAAAA...AAVV...VVVV..VBB....BBBB....AAAAAA.....ARR....RRRR..
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV..VBB....BBBB....AAAAAAA....ARR....RRRR..
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV...VBBBBBBBBB....AAAAAAAA....ARRRRRRRRRR..
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV...VBBBBBBBBB....AAA..AAA....ARRRRRRRRR...
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV...VBBBBBBBBBB..BAAAAAAAAA...ARRRRRRR.....
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV....VBB.....BBB..BAAAAAAAAA...ARR..RRRR....
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV....VBB.....BBB..BAAAAAAAAAA..ARR...RRRR...
//.NNN...NNNNN..AAA.....AAA...VVVVV.....VBBBBBBBBBB.BBAA....AAAA..ARR...RRRR...
//.NNN....NNNN.NAAA.....AAAA..VVVVV.....VBBBBBBBBBB.BBA......AAA..ARR....RRRR..
//.NNN....NNNN.NAA......AAAA..VVVVV.....VBBBBBBBBB..BBA......AAAA.ARR.....RRR..
//.............................................................................

pub fn render_nav_bar_1() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
            showHideTopBarWhenScroll();
        </script>
      "#))
      header id="navbar" class="top-0 right-0 left-0 z-50 fixed flex justify-center bg-blue-500 px-4 py-4 transition-all duration-500" {
        div class="flex justify-between items-center w-full max-w-360 h-full" {
          div class="flex items-center gap-7" {
            div class="text-white cursor-pointer" {
              (hamburger_icon())
            }
            div
              hx-get="/section/real-estate/contents/home"
              hx-push-url="/"
              hx-target="main"
              class="h-10 cursor-pointer"
            {
              img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
            }
          }
          div class="flex items-center gap-2 h-full text-white" {
            div {
              img class="h-3.5 translate-y-0.5" src="/assets/images/real_estate/en-flag.webp";
            }
            span { "English" }
            div class="translate-y-0.5 cursor-pointer" {
              (drop_down_icon())
            }
          }
        }
      }
    }
}

pub fn render_nav_bar_2() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
            showHideTopBarWhenScroll();
        </script>
      "#))
      header id="navbar" class="top-0 right-0 left-0 z-50 fixed transition-all duration-500" {
        div class="flex justify-end gap-6 bg-white px-4 py-2 font-semibold text-sm" {
          div class="flex items-center gap-2" {
            (mail_icon())
            div class="flex gap-2" {
              span {"hanatest0102@gmail.com"}
              span {"|"}
              span {"nguyenhan0696@gmail.com"}
            }
          }
          div class="flex items-center gap-2" {
            (phone_icon())
            div class="flex gap-2" {
              span {"+34 0973477994"}
              span {"|"}
              span {"+34 39652874"}
            }
          }
        }
        div class="justify-center items-center gap-15 grid grid-cols-[auto_auto] bg-blue-500 px-4 py-4 h-full" {
          div
            hx-get="/section/real-estate/contents/home"
            hx-push-url="/"
            hx-target="main"
            class="h-10 cursor-pointer"
          {
            img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
          }
          div class="flex items-center gap-10 h-full font-[500] text-white" {
            a class="hover:opacity-80" href="#" {"Home"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"About us"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-30 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Contact us"}
                a href="#" class="hover:opacity-80" {"About us"}
                a href="#" class="hover:opacity-80" {"Meet the team"}
              }
            }
            a class="hover:opacity-80" href="#" {"Sell your propery"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"For sale"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Resales"}
                a href="#" class="hover:opacity-80" {"New development"}
              }
            }
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"For rent"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Short rental"}
                a href="#" class="hover:opacity-80" {"Long rental"}
              }
            }
          }
        }
      }
    }
}

pub fn render_nav_bar_3() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
            showHideTopBarWhenScroll();
        </script>
      "#))
      header id="navbar" class="top-0 right-0 left-0 z-50 fixed flex justify-center px-4 py-4 transition-all duration-500" {
        div class="flex justify-between items-center gap-5 px-10 py-3 rounded-[40px] h-full" style="background-color: rgba(128,128,128,0.5)" {
          div
            hx-get="/section/real-estate/contents/home"
            hx-push-url="/"
            hx-target="main"
            class="h-10 cursor-pointer"
          {
            img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
          }
          div class="flex items-center gap-7 h-full font-[500] text-white" {
            a class="hover:opacity-50" href="#" {"Home"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-50 cursor-pointer" {
                span {"About us"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-30 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
                a href="#" class="hover:opacity-50" {"Contact us"}
                a href="#" class="hover:opacity-50" {"About us"}
                a href="#" class="hover:opacity-50" {"Meet the team"}
              }
            }
            a class="hover:opacity-50" href="#" {"Sell your propery"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-50 cursor-pointer" {
                span {"For sale"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
                a href="#" class="hover:opacity-50" {"Resales"}
                a href="#" class="hover:opacity-50" {"New development"}
              }
            }
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-50 cursor-pointer" {
                span {"For rent"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
                a href="#" class="hover:opacity-50" {"Short rental"}
                a href="#" class="hover:opacity-50" {"Long rental"}
              }
            }
            button class="bg-blue-500 hover:bg-blue-400 px-4 py-2 rounded-3xl cursor-pointer" {
              "Contact Us"
            }
          }
        }
      }
    }
}

pub fn render_nav_bar_4() -> Markup {
    html! {
      (PreEscaped(r#"
      <script type="module">
          import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
          showHideTopBarWhenScroll();
      </script>
    "#))
      header id="navbar" class="top-0 right-0 left-0 z-50 fixed transition-all duration-500" {
        div class="justify-center items-center gap-15 grid grid-cols-[auto_auto] bg-blue-500 px-4 py-4 h-full" {
          div class="flex items-center gap-10 h-full font-[500] text-white" {
            a class="hover:opacity-80" href="#" {"Home"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"About us"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-30 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Contact us"}
                a href="#" class="hover:opacity-80" {"About us"}
                a href="#" class="hover:opacity-80" {"Meet the team"}
              }
            }
            a class="hover:opacity-80" href="#" {"Sell your propery"}
            div
              hx-get="/section/real-estate/contents/home"
              hx-push-url="/"
              hx-target="main"
              class="h-10 cursor-pointer"
            {
              img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
            }
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"For sale"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Resales"}
                a href="#" class="hover:opacity-80" {"New development"}
              }
            }
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"For rent"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 bg-blue-400 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" {
                a href="#" class="hover:opacity-80" {"Short rental"}
                a href="#" class="hover:opacity-80" {"Long rental"}
              }
            }
            div class="flex items-center gap-2 h-full" {
              div {
                img class="h-3.5 translate-y-0.5" src="/assets/images/real_estate/en-flag.webp";
              }
              span { "English" }
              div class="translate-y-0.5 cursor-pointer" {
                (drop_down_icon())
              }
            }
          }
        }
      }
    }
}

//.........................................................................................
//....CCCCCCCC.....OOOOOOOO....NNNN....NNN.TTTTTTTTTTT...AAAAA......CCCCCCCCC.CCTTTTTTTTT..
//...CCCCCCCCCC...OOOOOOOOOO...NNNNN...NNN.TTTTTTTTTTT...AAAAA.....CCCCCCCCCCCCCTTTTTTTTT..
//..CCCCCCCCCCCC.OOOOOOOOOOOO..NNNNN...NNN.TTTTTTTTTTT..AAAAAA.....CCCCCCCCCCC.CTTTTTTTTT..
//..CCCC....CCCC.OOOO....OOOO..NNNNNN..NNN.....TTT......AAAAAAA...ACCCC....CCCC...TTTT.....
//.CCCC......C..OOOO......OOOO.NNNNNN..NNN.....TTT......AAAAAAA...ACCC.....CC.....TTTT.....
//.CCCC.........OOOO......OOOO.NNNNNNN.NNN.....TTT.....AAAA.AAA...ACCC............TTTT.....
//.CCCC.........OOOO......OOOO.NNN.NNN.NNN.....TTT.....AAAA.AAAA..ACCC............TTTT.....
//.CCCC.........OOOO......OOOO.NNN.NNNNNNN.....TTT....AAAAAAAAAA..ACCC............TTTT.....
//.CCCC......CC.OOOO......OOOO.NNN..NNNNNN.....TTT....AAAAAAAAAA..ACCC.....CCC....TTTT.....
//..CCCC....CCCC.OOOO....OOOO..NNN..NNNNNN.....TTT....AAAAAAAAAAA.ACCCC....CCCC...TTTT.....
//..CCCCCCCCCCCC.OOOOOOOOOOOO..NNN...NNNNN.....TTT...TAAA....AAAA..CCCCCCCCCCC....TTTT.....
//...CCCCCCCCCC...OOOOOOOOOO...NNN...NNNNN.....TTT...TAAA.....AAA..CCCCCCCCCCC....TTTT.....
//....CCCCCCCC.....OOOOOOOO....NNN....NNNN.....TTT...TAAA.....AAAA...CCCCCCCC.....TTTT.....
//.........................................................................................

pub fn render_contact() -> Markup {
    html! {
      div class="flex justify-center items-center text-white" style="background-image: url('https://d1qawt2l8egll1.cloudfront.net/prod/images/230926025644-contact.jpg'); background-color: rgba(0,0,0,0.5); background-blend-mode: overlay;" {
        div class="flex flex-col justify-center items-center gap-20 px-15 py-30 w-full max-w-360" {
          div class="flex items-center gap-2 w-full" {
            div class="border-t-3 border-t-white w-4" {}
            div class="font-bold text-lg uppercase" {
              "Inquiry About This Property"
            }
          }
          div class="flex flex-col justify-center items-center gap-10 max-w-4xl" {
            div class="w-full text-lg" {
              "Do not hesitate contacting us, we will be happy to help you"
            }
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

pub fn render_contact_input(label: &str) -> Markup {
    html! {
      div class="flex flex-col gap-2 w-full" {
        label {
          (label)
        }
        input class="rounded-md";
      }
    }
}

//.............................................................................
//.FFFFFFFFFF....OOOOOO........OOOOOO.....TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRR.....
//.FFFFFFFFFF..OOOOOOOOOO....OOOOOOOOOO...TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR...
//.FFFFFFFFFF.OOOOOOOOOOOO..OOOOOOOOOOOO..TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR...
//.FFF........OOOO....OOOO..OOOO....OOOO......TTT....EEE.........RRR.....RRR...
//.FFF........OOO......OOO..OOO......OOO......TTT....EEE.........RRR.....RRR...
//.FFFFFFFFF.FOOO......OOOOOOOO......OOOO.....TTT....EEEEEEEEEE..RRRRRRRRRRR...
//.FFFFFFFFF.FOOO......OOOOOOOO......OOOO.....TTT....EEEEEEEEEE..RRRRRRRRRR....
//.FFFFFFFFF.FOOO......OOOOOOOO......OOOO.....TTT....EEEEEEEEEE..RRRRRRRR......
//.FFF........OOO......OOO..OOO......OOO......TTT....EEE.........RRR..RRRR.....
//.FFF........OOOO....OOOO..OOOO....OOOO......TTT....EEE.........RRR...RRRR....
//.FFF........OOOOOOOOOOOO..OOOOOOOOOOOO......TTT....EEEEEEEEEEE.RRR....RRRR...
//.FFF.........OOOOOOOOOO....OOOOOOOOOO.......TTT....EEEEEEEEEEE.RRR....RRRR...
//.FFF...........OOOOOO........OOOOOO.........TTT....EEEEEEEEEEE.RRR.....RRRR..
//.............................................................................

pub fn render_footer_1() -> Markup {
    html! {
      div class="flex flex-col justify-center items-center bg-blue-500 w-full text-white" {
        div class="flex flex-col gap-5 px-15 py-15 w-ful w-full max-w-360" {
          div class="flex justify-between w-full" {
            div class="flex gap-30" {
              div class="flex flex-col gap-4"  {
                div class="rounded-sm max-w-60 overflow-hidden" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
                  img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
                }
                div class="max-w-85 text-justify" {
                  "Welcome to DEMO AGENCY, your trusted partner in the world of real estate. With years of experience and a passion for exceptional service, we specialize in helping clients buy, sell, and invest in properties. Our dedicated team of professionals is committed to delivering personalized solutions and exceeding your expectations every step of the way."
                }
              }
              div class="flex gap-15" {
                div class="flex flex-col gap-3" {
                  div class="cursor-pointer" { "Home" }
                  div class="cursor-pointer" { "Contact us" }
                  div class="cursor-pointer" { "About us" }
                  div class="cursor-pointer" { "Meet the team" }
                  div class="cursor-pointer" { "Sell your property" }
                }
                div class="flex flex-col gap-3" {
                  div class="cursor-pointer" { "Resales" }
                  div class="cursor-pointer" { "New development" }
                  div class="cursor-pointer" { "Short rental" }
                  div class="cursor-pointer" { "Long rental" }
                }
              }
            }
            div class="flex flex-col gap-4" {
              div class="flex items-center gap-2" {
                div class="translate-y-0.5" {
                  (email_icon())
                }
                "hanatest0102@gmail.com"
              }
              div class="flex items-center gap-2" {
                div class="translate-y-0.5" {
                  (email_icon())
                }
                "nguyenhan0696@gmail.com"
              }
              div class="flex items-center gap-2" {
                (phone_icon())
                "Sale Manager +34 0973477994"
              }
              div class="flex items-center gap-2" {
                (phone_icon())
                "Sale Manager +34 39652874"
              }
              div class="flex items-center gap-2" {
                (location_icon())
                "Testing Address 4"
              }
            }
          }
          div class="border-white border-b-1 w-full" {}
          div class="flex justify-between" {
            div class="flex flex-col gap-3" {
              div { "Raicv: 123456789F" }
              div {
                "Copyright © 2023 - DEMO AGENCY. All Rights Reserved."
              }
              div class="flex divide-x divide-white text-sm" {
                div class="pr-3 cursor-pointer" { "Terms and Conditions" }
                div class="px-3 cursor-pointer" { "Privacy Policy" }
                div class="px-3 cursor-pointer" { "Cookie Policy" }
                div class="px-3 cursor-pointer" { "Cookie Settings" }
              }
            }
            div {
              div class="flex items-center gap-3" {
                (instagram_icon_color())
                (twitter_icon_color())
                (linkedin_icon_color())
                (facebook_icon_color())
                (youtube_icon_color())
              }
            }
          }
        }
      }
    }
}

pub fn render_footer_2() -> Markup {
    html! {
      div class="flex flex-col justify-center items-center bg-blue-500 w-full text-white" {
        div class="flex flex-col gap-5 px-15 py-10 w-ful w-full max-w-360" {
          div class="flex justify-between w-full" {
            div class="flex flex-col justify-between" {
                div class="flex flex-col gap-4"  {
                  div class="rounded-sm max-w-60 overflow-hidden" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
                    img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
                  }
                  div class="max-w-85 text-justify" {
                    div class="flex items-center gap-2" {
                      (location_icon())
                      "Testing Address 4"
                    }
                  }
                }
                div class="flex gap-3" {
                  (certificate_icon())
                  (certificate_icon())
                  (certificate_icon())
                  (certificate_icon())
                }
            }
            div class="flex flex-col gap-7" {
              div class="flex flex-col gap-4" {
                div class="flex items-center gap-2" {
                  div class="translate-y-0.5" {
                    (email_icon())
                  }
                  "hanatest0102@gmail.com"
                }
                div class="flex items-center gap-2" {
                  div class="translate-y-0.5" {
                    (email_icon())
                  }
                  "nguyenhan0696@gmail.com"
                }
                div class="flex items-center gap-2" {
                  (phone_icon())
                  "Sale Manager +34 0973477994"
                }
                div class="flex items-center gap-2" {
                  (phone_icon())
                  "Sale Manager +34 39652874"
                }
              }
              div {
                div class="flex items-center gap-3" {
                  (instagram_icon_color())
                  (twitter_icon_color())
                  (linkedin_icon_color())
                  (facebook_icon_color())
                  (youtube_icon_color())
                }
              }
            }
          }
          div class="border-white border-b-1 w-full" {}
          div class="flex justify-between" {
            div class="flex flex-col gap-3" {
              div { "Raicv: 123456789F" }
              div {
                "Copyright © 2023 - DEMO AGENCY. All Rights Reserved."
              }
            }
            div class="flex items-end" {
              div class="flex divide-x divide-white h-fit text-sm" {
                div class="pr-3 cursor-pointer" { "Terms and Conditions" }
                div class="px-3 cursor-pointer" { "Privacy Policy" }
                div class="px-3 cursor-pointer" { "Cookie Policy" }
                div class="px-3 cursor-pointer" { "Cookie Settings" }
              }
            }
          }
        }
      }
    }
}

pub fn render_footer_3() -> Markup {
    html! {
      div class="flex flex-col justify-center items-center bg-blue-500 w-full text-white" {
        div class="flex flex-col gap-5 px-15 py-15 w-ful w-full max-w-360" {
          div class="flex justify-between w-full" {
            div class="flex flex-col gap-4"  {
              div class="rounded-sm max-w-60 overflow-hidden" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
                img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
              }
              div {
                "Copyright © 2023 - DEMO AGENCY. All Rights Reserved."
              }
              div { "Raicv: 123456789F" }
              div {
                div class="flex items-center gap-3" {
                  (instagram_icon_color())
                  (twitter_icon_color())
                  (linkedin_icon_color())
                  (facebook_icon_color())
                  (youtube_icon_color())
                }
              }
              div class="flex flex-col gap-3 text-sm" {
                div class="cursor-pointer" { "Terms and Conditions" }
                div class="cursor-pointer" { "Privacy Policy" }
                div class="cursor-pointer" { "Cookie Policy" }
                div class="cursor-pointer" { "Cookie Settings" }
              }
            }
            div class="flex flex-col justify-between" {
              div class="flex gap-7" {
                div class="flex gap-3" {
                  div class="font-bold text-sm translate-y-0.5" { "Company" }
                  div class="flex flex-col gap-3" {
                    span { "About us" }
                    span { "Contact" }
                    span { "Sell your property" }
                    span { "Meet the team" }
                  }
                }
                div class="flex gap-3" {
                  div class="font-bold text-sm translate-y-0.5" { "Sale" }
                  div class="flex flex-col gap-3" {
                    span { "For sale" }
                    span { "Nev Development" }
                  }
                }
                div class="flex gap-3" {
                  div class="font-bold text-sm translate-y-0.5" { "Rent" }
                  div class="flex flex-col gap-3" {
                    span { "Long term" }
                    span { "Short term" }
                  }
                }
              }
              div class="flex gap-3" {
                (certificate_icon())
                (certificate_icon())
                (certificate_icon())
                (certificate_icon())
              }
            }
            div class="flex flex-col gap-4" {
              div class="flex items-center gap-2 text-white" {
                div {
                  img class="h-3.5 translate-y-0.5" src="/assets/images/real_estate/en-flag.webp";
                }
                span { "English" }
                div class="translate-y-0.5 cursor-pointer" {
                  (drop_down_icon())
                }
              }
              div class="flex items-center gap-2" {
                div class="translate-y-0.5" {
                  (email_icon())
                }
                "hanatest0102@gmail.com"
              }
              div class="flex items-center gap-2" {
                div class="translate-y-0.5" {
                  (email_icon())
                }
                "nguyenhan0696@gmail.com"
              }
              div class="flex items-center gap-2" {
                (phone_icon())
                "Sale Manager +34 0973477994"
              }
              div class="flex items-center gap-2" {
                (phone_icon())
                "Sale Manager +34 39652874"
              }
            }
          }
        }
      }
    }
}

pub fn render_footer_4() -> Markup {
    html! {
      div class="flex flex-col justify-center items-center bg-blue-500 w-full text-white" {
        div class="flex flex-col gap-5 px-15 py-15 w-ful w-full max-w-360" {
          div class="flex justify-between w-full" {
            div class="flex gap-30" {
              div class="flex flex-col gap-4" {
                div class="flex items-center gap-2" {
                  div class="translate-y-0.5" {
                    (email_icon())
                  }
                  "hanatest0102@gmail.com"
                }
                div class="flex items-center gap-2" {
                  div class="translate-y-0.5" {
                    (email_icon())
                  }
                  "nguyenhan0696@gmail.com"
                }
                div class="flex items-center gap-2" {
                  (phone_icon())
                  "Sale Manager +34 0973477994"
                }
                div class="flex items-center gap-2" {
                  (phone_icon())
                  "Sale Manager +34 39652874"
                }
                div class="flex items-center gap-2" {
                  (location_icon())
                  "Testing Address 4"
                }
              }
              div class="flex gap-15" {
                div class="flex flex-col gap-3" {
                  div class="cursor-pointer" { "About us" }
                  div class="cursor-pointer" { "Contact us" }
                  div class="cursor-pointer" { "Sell your property" }
                  div class="cursor-pointer" { "Meet the team" }
                }
                div class="flex flex-col gap-3" {
                  div class="cursor-pointer" { "Resales" }
                  div class="cursor-pointer" { "New development" }
                }
                div class="flex flex-col gap-3" {
                  div class="cursor-pointer" { "Resales" }
                  div class="cursor-pointer" { "New development" }
                }
              }
            }
            div class="flex gap-3" {
              (certificate_icon())
              (certificate_icon())
              (certificate_icon())
              (certificate_icon())
            }
            div {
              div class="flex items-center gap-2 text-white" {
                div {
                  img class="h-3.5 translate-y-0.5" src="/assets/images/real_estate/en-flag.webp";
                }
                span { "English" }
                div class="translate-y-0.5 cursor-pointer" {
                  (drop_down_icon())
                }
              }
            }
          }
          div class="border-white my-5 border-b-1 w-full" {}
          div class="flex justify-between items-center" {
            div class="flex flex-col gap-3" {
              div class="flex gap-4"  {
                div class="rounded-sm max-w-60 h-fit overflow-hidden" style="box-shadow: rgba(14, 30, 37, 0.12) 0px 2px 4px 0px, rgba(14, 30, 37, 0.32) 0px 2px 16px 0px;" {
                  img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
                }
                div class="max-w-100 text-justify" {
                  "Welcome to DEMO AGENCY, your trusted partner in the world of real estate. With years of experience and a passion for exceptional service, we specialize in helping clients buy, sell, and invest in properties. Our dedicated team of professionals is committed to delivering personalized solutions and exceeding your expectations every step of the way."
                }
              }
              div { "Raicv: 123456789F" }
              div {
                "Copyright © 2023 - DEMO AGENCY. All Rights Reserved."
              }
            }
            div class="flex flex-col gap-12" {
              div class="flex justify-end items-center gap-3" {
                (instagram_icon_color())
                (twitter_icon_color())
                (linkedin_icon_color())
                (facebook_icon_color())
                (youtube_icon_color())
              }
              div class="flex gap-4 text-sm" {
                div class="cursor-pointer" { "Terms and Conditions" }
                div class="cursor-pointer" { "Privacy Policy" }
                div class="cursor-pointer" { "Cookie Policy" }
                div class="cursor-pointer" { "Cookie Settings" }
              }
            }
          }
        }
      }
    }
}

//....................................................................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....PPPPPPPPP...EEEEEEEEEE..RRRRRRRRR....TTTTTTTTTTTYYY....YYY..
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYY....YYY..
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR..TTTTTTTTTTTYYYY..YYYY..
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYY..YYY...
//.PPP....PPPP.RRR.....RRR..OOO......OOO..PPP....PPPP.EEE.........RRR.....RRR......TTT.....YYYYYYYY...
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRRR......TTT......YYYYYY....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.PPPPPPPPPP..EEEEEEEEEE..RRRRRRRRRR.......TTT.......YYYY.....
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.PPPPPPPPP...EEEEEEEEEE..RRRRRRRR.........TTT.......YYYY.....
//.PPP.........RRR..RRRR....OOO......OOO..PPP.........EEE.........RRR..RRRR........TTT.......YYYY.....
//.PPP.........RRR...RRRR...OOOO....OOOO..PPP.........EEE.........RRR...RRRR.......TTT.......YYYY.....
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY.....
//.PPP.........RRR....RRRR...OOOOOOOOOO...PPP.........EEEEEEEEEEE.RRR....RRRR......TTT.......YYYY.....
//.PPP.........RRR.....RRRR....OOOOOO.....PPP.........EEEEEEEEEEE.RRR.....RRRR.....TTT.......YYYY.....
//....................................................................................................

pub fn render_property_card(property: &SearchProperty, listing_type: &str) -> Markup {
    let mut total_pictures = 0;

    let render_main_image = if let Some(main_image) = &property.main_image {
        total_pictures = 1;
        html! {
          img class="w-full h-full pointer-events-none shrink-0" src=(main_image);
        }
    } else {
        html! {}
    };

    let render_images = if let Some(images) = &property.pictures {
        total_pictures = total_pictures + images.count;
        html! {
          @for picture in &images.picture {
            img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
          }
        }
    } else {
        html! {}
    };

    html! {
      div class="relative flex flex-col gap-2 shadow-md rounded-lg overflow-hidden picture-container" {
        div class="relative picture-slider-container" {
          div class="flex h-42 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures);
            (render_main_image)
            (render_images)
          }
          div class="bottom-2 left-[50%] absolute flex gap-2 max-w-18 -translate-x-[50%] overflow-hidden pictures-dots" {
            @for i in 0..total_pictures as u8 {
              @if i == 0 {
                div class="bg-blue-500 p-1 rounded-full cursor-pointer" {}
              } @else {
                div class="bg-blue-200 p-1 rounded-full cursor-pointer" {}
              }
            }
          }
        }
        div
          hx-get=(format!("/section/real-estate/contents/property?id={}&type={}", property.reference, listing_type))
          hx-push-url=(format!("/property?id={}&type={}", property.reference, listing_type))
          hx-trigger="click"
          hx-target="main"
          class="flex flex-col justify-between gap-2 px-3 py-2 h-full cursor-pointer"
        {
          div class="font-bold" {
            @if property.newdev_name == "" {
              (property.property_type.name_type)
            }@else {
              (property.newdev_name)
            }
          }
          div class="flex flex-col gap-2" {
            div class="font-bold text-blue-500 text-lg" {
              (property.price) " €"
            }
            div class="text-sm" {
              (property.location)
            }
            div class="flex gap-4 text-sm" {
              div class="flex items-center gap-2" {
                (bed_icon())
                (property.bedrooms)
              }
              div class="flex items-center gap-2" {
                (bath_icon())
                (property.bathrooms)
              }
              div class="flex items-center gap-2" {
                (buit_size_icon())
                @match &property.built {
                    TextOrNum::Text(built) => (built),
                    TextOrNum::Num(built) => (built),
                }
                @if property.dimensions == "Metres" {
                  "m²"
                }
              }
            }
          }
        }
      }
    }
}
