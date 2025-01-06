use maud::{html, Markup, PreEscaped};

use crate::{
    models::rso_data::{SearchProperty, TextOrNum},
    views::icons::{
        bath_icon, bed_icon, buit_size_icon, drop_down_icon, email_icon, hamburger_icon,
        location_icon, mail_icon, phone_icon,
    },
};

//..............................................
//.NNNN....NNN.....AAAA...AAAV.....VVVV....111..
//.NNNN....NNN.....AAAAA...AAV.....VVVV...1111..
//.NNNNN...NNN.....AAAAA...AAVV....VVV...11111..
//.NNNNN...NNN....AAAAAA...AAVV...VVVV..V11111..
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV..V11111..
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV...V1.111..
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV......111..
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV......111..
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV.......111..
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV.......111..
//.NNN...NNNNN..AAA.....AAA...VVVVV........111..
//.NNN....NNNN.NAAA.....AAAA..VVVVV........111..
//.NNN....NNNN.NAA......AAAA..VVVVV........111..
//..............................................

pub fn render_nav_bar_1() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
            showHideTopBarWhenScroll();
        </script>
      "#))
      header id="navbar" class="flex justify-center top-0 right-0 left-0 z-50 fixed transition-all duration-500 bg-blue-500 px-4 py-4" {
        div class="flex justify-between items-center w-full h-full max-w-360" {
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
          div class="flex items-center gap-2 text-white" {
            span { "EN" }
            div class="cursor-pointer" {
              (drop_down_icon())
            }
          }
        }
      }
    }
}

//................................................
//.NNNN....NNN.....AAAA...AAAV.....VVVV..22222....
//.NNNN....NNN.....AAAAA...AAV.....VVVV.V2222222..
//.NNNNN...NNN.....AAAAA...AAVV....VVV.VV22..222..
//.NNNNN...NNN....AAAAAA...AAVV...VVVV.VV2...222..
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV.......222..
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV.......2222..
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV......2222...
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV.....22222...
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV.....22222....
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV....V2222.....
//.NNN...NNNNN..AAA.....AAA...VVVVV.....V222......
//.NNN....NNNN.NAAA.....AAAA..VVVVV....VV2222222..
//.NNN....NNNN.NAA......AAAA..VVVVV....VV2222222..
//................................................

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

//................................................
//.NNNN....NNN.....AAAA...AAAV.....VVVV..33333....
//.NNNN....NNN.....AAAAA...AAV.....VVVV.V333333...
//.NNNNN...NNN.....AAAAA...AAVV....VVV..V33.3333..
//.NNNNN...NNN....AAAAAA...AAVV...VVVV.VV33.3333..
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV......3333..
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV......3333...
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV......3333...
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV........333..
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV.........333..
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV...VV33..333..
//.NNN...NNNNN..AAA.....AAA...VVVVV....VV33.3333..
//.NNN....NNNN.NAAA.....AAAA..VVVVV.....V333333...
//.NNN....NNNN.NAA......AAAA..VVVVV......33333....
//................................................

pub fn render_nav_bar_3() -> Markup {
    html! {
      (PreEscaped(r#"
        <script type="module">
            import {showHideTopBarWhenScroll} from "/assets/js/app/navbar.js";
            showHideTopBarWhenScroll();
        </script>
      "#))
      header id="navbar" class="flex justify-center top-0 right-0 left-0 z-50 fixed transition-all duration-500 px-4 py-4" {
        div class="flex justify-between items-center h-full px-10 py-3 rounded-[40px] gap-5" style="background-color: rgba(128,128,128,0.5)" {
          div
            hx-get="/section/real-estate/contents/home"
            hx-push-url="/"
            hx-target="main"
            class="h-10 cursor-pointer"
          {
            img class="h-full" src="https://cdn.resales-online.com/public/804pf2s7h1/agencies/3/3.jpg";
          }
          div class="flex items-center gap-7 h-full font-[500] text-white" {
            a class="hover:opacity-80" href="#" {"Home"}
            div class="relative flex items-center h-full group" {
              div class="flex items-center gap-1 hover:opacity-80 cursor-pointer" {
                span {"About us"}
                div class="translate-y-0.5" {
                  (drop_down_icon())
                }
              }
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-30 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
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
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
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
              div class="group-hover:visible top-10 z-10 absolute flex flex-col gap-3 opacity-0 group-hover:opacity-100 px-3 py-2 rounded-md max-h-0 group-hover:max-h-24 whitespace-nowrap transition-all duration-500 invisible overflow-hidden" style="background-color: rgba(128,128,128,0.5)" {
                a href="#" class="hover:opacity-80" {"Short rental"}
                a href="#" class="hover:opacity-80" {"Long rental"}
              }
            }
            button class="px-4 py-2 bg-blue-500 hover:bg-blue-400 rounded-3xl cursor-pointer" {
              "Contact Us"
            }
          }
        }
      }
    }
}

//................................................
//.NNNN....NNN.....AAAA...AAAV.....VVVV.....444...
//.NNNN....NNN.....AAAAA...AAV.....VVVV....4444...
//.NNNNN...NNN.....AAAAA...AAVV....VVV....44444...
//.NNNNN...NNN....AAAAAA...AAVV...VVVV....44444...
//.NNNNNN..NNN....AAAAAAA...AVV...VVVV...444444...
//.NNNNNNN.NNN...AAAA.AAA...AVVV..VVV...V444444...
//.NNN.NNN.NNN...AAA..AAAA..AVVV.VVVV...V44.444...
//.NNN.NNNNNNN...AAAAAAAAA...VVV.VVVV..VV4..444...
//.NNN..NNNNNN..AAAAAAAAAA...VVVVVVV...VV4444444..
//.NNN..NNNNNN..AAAAAAAAAAA..VVVVVVV...VV4444444..
//.NNN...NNNNN..AAA.....AAA...VVVVV.........444...
//.NNN....NNNN.NAAA.....AAAA..VVVVV.........444...
//.NNN....NNNN.NAA......AAAA..VVVVV.........444...
//................................................

pub fn render_nav_bar_4() -> Markup {
    html! {
      "Nav 4"
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

//............................................................................
//.FFFFFFFFFF...OOOOOOOO......OOOOOOOO...OTTTTTTTTTT.EEEEEEEEEE..RRRRRRRRRR...
//.FFFFFFFFFF..OOOOOOOOOO....OOOOOOOOOO..OTTTTTTTTTT.EEEEEEEEEE..RRRRRRRRRRR..
//.FFFFFFFFFF.OOOOOOOOOOOO..OOOOOOOOOOOO.OTTTTTTTTTT.EEEEEEEEEE..RRRRRRRRRRR..
//.FFF........OOOO....OOOO..OOOO....OOOO.....TTT.....EEE.........RRR.....RRR..
//.FFF.......FOOO......OOOOOOOO......OOOO....TTT.....EEE.........RRR.....RRR..
//.FFF.......FOOO......OOOOOOOO......OOOO....TTT.....EEEEEEEEEE..RRR...RRRRR..
//.FFFFFFFFFFFOOO......OOOOOOOO......OOOO....TTT.....EEEEEEEEEE..RRRRRRRRRRR..
//.FFFFFFFFFFFOOO......OOOOOOOO......OOOO....TTT.....EEEEEEEEEE..RRRRRRRRRR...
//.FFFFFFFFFFFOOO......OOOOOOOO......OOOO....TTT.....EEE.........RRRRRRRRRR...
//.FFF........OOOO....OOOO..OOOO....OOOO.....TTT.....EEE.........RRR...RRRR...
//.FFF........OOOOOOOOOOOO..OOOOOOOOOOOO.....TTT.....EEEEEEEEEEE.RRR....RRRR..
//.FFF.........OOOOOOOOOO....OOOOOOOOOO......TTT.....EEEEEEEEEEE.RRR....RRRR..
//.FFF..........OOOOOOOO......OOOOOOOO.......TTT.....EEEEEEEEEEE.RRR.....RRR..
//............................................................................

pub fn render_footer() -> Markup {
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
          div class="flex flex-col gap-3" {
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
    let total_pictures = *&property.pictures.count;

    html! {
      div class="relative flex flex-col gap-2 shadow-md rounded-lg overflow-hidden picture-container" {
        div class="relative picture-slider-container" {
          div class="flex h-42 transition-transform duration-500 picture-slider" {
            input type="hidden" value=(total_pictures) ;
            @for picture in &property.pictures.picture {
              img class="w-full h-full pointer-events-none shrink-0" src=(picture.picture_url);
            }
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
