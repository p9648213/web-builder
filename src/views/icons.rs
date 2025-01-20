use maud::{html, Markup};

pub fn footer_next_icon() -> Markup {
    html! {
        svg class="lucide-move-right lucide" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M18 8L22 12L18 16" {
            }
            path d="M2 12H22" {
            }
        }
    }
}

pub fn footer_back_icon() -> Markup {
    html! {
        svg class="lucide-move-left lucide" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M6 8L2 12L6 16" {
            }
            path d="M2 12H22" {
            }
        }
    }
}

pub fn bed_icon() -> Markup {
    html! {
        svg class="lucide lucide-bed" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M2 4v16" {
            }
            path d="M2 8h18a2 2 0 0 1 2 2v10" {
            }
            path d="M2 17h20" {
            }
            path d="M6 8v9" {
            }
        }
    }
}

pub fn bed_icon_light() -> Markup {
    html! {
        svg class="lucide lucide-bed" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" {
            path d="M2 4v16" {
            }
            path d="M2 8h18a2 2 0 0 1 2 2v10" {
            }
            path d="M2 17h20" {
            }
            path d="M6 8v9" {
            }
        }
    }
}

pub fn bath_icon() -> Markup {
    html! {
        svg class="lucide lucide-bath" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M10 4 8 6" {
            }
            path d="M17 19v2" {
            }
            path d="M2 12h20" {
            }
            path d="M7 19v2" {
            }
            path d="M9 5 7.621 3.621A2.121 2.121 0 0 0 4 5v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" {
            }
        }
    }
}

pub fn bath_icon_light() -> Markup {
    html! {
        svg class="lucide lucide-bath" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" {
            path d="M10 4 8 6" {
            }
            path d="M17 19v2" {
            }
            path d="M2 12h20" {
            }
            path d="M7 19v2" {
            }
            path d="M9 5 7.621 3.621A2.121 2.121 0 0 0 4 5v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" {
            }
        }
    }
}

pub fn buit_size_icon() -> Markup {
    html! {
        svg class="lucide lucide-table-2" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18" {
            }
        }
    }
}

pub fn next_icon() -> Markup {
    html! {
        svg class="lucide-chevron-right lucide" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke-width="2" width="24" height="24" stroke-linecap="round" {
            path d="m9 18 6-6-6-6" {}
        }
    }
}

pub fn previous_icon() -> Markup {
    html! {
        svg class="lucide-chevron-left lucide" stroke-width="2" width="24" viewBox="0 0 24 24" stroke-linecap="round" fill="none" stroke-linejoin="round" height="24" xmlns="http://www.w3.org/2000/svg" {
            path d="m15 18-6-6 6-6" {}
        }
    }
}

pub fn check_icon() -> Markup {
    html! {
        svg class="lucide lucide-circle-check" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="#98FB98" stroke="#2E6F40" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            circle cx="12" cy="12" r="10" {
            }
            path d="m9 12 2 2 4-4" {
            }
        }
    }
}

pub fn star_icon() -> Markup {
    html! {
        svg class="lucide lucide-star" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#FFA43C" stroke="#FFA43C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z" {
            }
        }
    }
}

pub fn email_icon() -> Markup {
    html! {
        svg class="lucide lucide-mail" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" {
            rect width="20" height="16" x="2" y="4" rx="2" {
            }
            path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" {
            }
        }
    }
}

pub fn location_icon() -> Markup {
    html! {
        svg class="lucide-map-pin lucide" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" {
            path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0" {
            }
            circle cx="12" cy="10" r="3" {
            }
        }
    }
}

pub fn buit_size_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-pencil-ruler" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13" {
            }
            path d="m8 6 2-2" {
            }
            path d="m18 16 2-2" {
            }
            path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17" {
            }
            path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" {
            }
            path d="m15 5 4 4" {
            }
        }
    }
}

pub fn plot_size_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-layers" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z" {
            }
            path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12" {
            }
            path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17" {
            }
        }
    }
}

pub fn useful_size_gray_icon() -> Markup {
    html! {
        svg class="lucide-grid-2x2 lucide" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M12 3v18" {
            }
            path d="M3 12h18" {
            }
            rect x="3" y="3" width="18" height="18" rx="2" {
            }
        }
    }
}

pub fn terrace_size_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-fence" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M4 3 2 5v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" {
            }
            path d="M6 8h4" {
            }
            path d="M6 18h4" {
            }
            path d="m12 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" {
            }
            path d="M14 8h4" {
            }
            path d="M14 18h4" {
            }
            path d="m20 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" {
            }
        }
    }
}

pub fn bedroom_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-bed-double" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" {
            }
            path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" {
            }
            path d="M12 4v6" {
            }
            path d="M2 18h20" {
            }
        }
    }
}

pub fn bathroom_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-bath" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M10 4 8 6" {
            }
            path d="M17 19v2" {
            }
            path d="M2 12h20" {
            }
            path d="M7 19v2" {
            }
            path d="M9 5 7.621 3.621A2.121 2.121 0 0 0 4 5v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" {
            }
        }
    }
}

pub fn ibi_tax_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-omega" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M3 20h4.5a.5.5 0 0 0 .5-.5v-.282a.52.52 0 0 0-.247-.437 8 8 0 1 1 8.494-.001.52.52 0 0 0-.247.438v.282a.5.5 0 0 0 .5.5H21" {
            }
        }
    }
}

pub fn basura_tax_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-square-percent" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            rect width="18" height="18" x="3" y="3" rx="2" {
            }
            path d="m15 9-6 6" {
            }
            path d="M9 9h.01" {
            }
            path d="M15 15h.01" {
            }
        }
    }
}

pub fn community_fee_gray_icon() -> Markup {
    html! {
        svg class="lucide lucide-newspaper" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#868d9b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2" {
            }
            path d="M18 14h-8" {
            }
            path d="M15 18h-5" {
            }
            path d="M10 6h8v4h-8V6Z" {
            }
        }
    }
}

pub fn hamburger_icon() -> Markup {
    html! {
        svg class="lucide lucide-menu" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            line x1="4" x2="20" y1="12" y2="12" {
            }
            line x1="4" x2="20" y1="6" y2="6" {
            }
            line x1="4" x2="20" y1="18" y2="18" {
            }
        }
    }
}

pub fn facebook_icon_color() -> Markup {
    html! {
        svg width="28" height="28" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" {
            g clip-path="url(#clip0_17_24)" {
                path d="M48 24C48 10.7453 37.2547 0 24 0C10.7453 0 0 10.7453 0 24C0 35.255 7.74912 44.6995 18.2026 47.2934V31.3344H13.2538V24H18.2026V20.8397C18.2026 12.671 21.8995 8.8848 29.9194 8.8848C31.44 8.8848 34.0637 9.18336 35.137 9.48096V16.129C34.5706 16.0694 33.5866 16.0397 32.3645 16.0397C28.4294 16.0397 26.9088 17.5306 26.9088 21.4061V24H34.7482L33.4013 31.3344H26.9088V47.8243C38.7926 46.3891 48.001 36.2707 48.001 24H48Z" fill="#0866FF" {
                }
                path d="M33.4003 31.3344L34.7472 24H26.9078V21.4061C26.9078 17.5306 28.4285 16.0397 32.3635 16.0397C33.5856 16.0397 34.5696 16.0694 35.136 16.129V9.48096C34.0627 9.1824 31.439 8.8848 29.9184 8.8848C21.8986 8.8848 18.2016 12.671 18.2016 20.8397V24H13.2528V31.3344H18.2016V47.2934C20.0582 47.7542 22.0003 48 23.999 48C24.983 48 25.9536 47.9395 26.9069 47.8243V31.3344H33.3994H33.4003Z" fill="white" {
                }
            }
            defs {
                clipPath id="clip0_17_24" {
                    rect width="48" height="48" fill="white" {
                    }
                }
            }
        }
    }
}

pub fn instagram_icon_color() -> Markup {
    html! {
        svg width="36px" height="36px" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" {
            circle cx="24" cy="24" r="20" fill="#C13584" {
            }
            path d="M24 14.1622C27.2041 14.1622 27.5837 14.1744 28.849 14.2321C30.019 14.2855 30.6544 14.481 31.0773 14.6453C31.6374 14.863 32.0371 15.123 32.457 15.5429C32.877 15.9629 33.137 16.3626 33.3547 16.9227C33.519 17.3456 33.7145 17.981 33.7679 19.1509C33.8256 20.4163 33.8378 20.7958 33.8378 23.9999C33.8378 27.2041 33.8256 27.5836 33.7679 28.849C33.7145 30.019 33.519 30.6543 33.3547 31.0772C33.137 31.6373 32.877 32.0371 32.4571 32.457C32.0371 32.8769 31.6374 33.1369 31.0773 33.3546C30.6544 33.519 30.019 33.7144 28.849 33.7678C27.5839 33.8255 27.2044 33.8378 24 33.8378C20.7956 33.8378 20.4162 33.8255 19.151 33.7678C17.981 33.7144 17.3456 33.519 16.9227 33.3546C16.3626 33.1369 15.9629 32.8769 15.543 32.457C15.1231 32.0371 14.863 31.6373 14.6453 31.0772C14.481 30.6543 14.2855 30.019 14.2321 28.849C14.1744 27.5836 14.1622 27.2041 14.1622 23.9999C14.1622 20.7958 14.1744 20.4163 14.2321 19.1509C14.2855 17.981 14.481 17.3456 14.6453 16.9227C14.863 16.3626 15.123 15.9629 15.543 15.543C15.9629 15.123 16.3626 14.863 16.9227 14.6453C17.3456 14.481 17.981 14.2855 19.151 14.2321C20.4163 14.1744 20.7959 14.1622 24 14.1622ZM24 12C20.741 12 20.3323 12.0138 19.0524 12.0722C17.7752 12.1305 16.9028 12.3333 16.1395 12.63C15.3504 12.9366 14.6812 13.3469 14.0141 14.0141C13.3469 14.6812 12.9366 15.3504 12.63 16.1395C12.3333 16.9028 12.1305 17.7751 12.0722 19.0524C12.0138 20.3323 12 20.741 12 23.9999C12 27.259 12.0138 27.6676 12.0722 28.9475C12.1305 30.2248 12.3333 31.0971 12.63 31.8604C12.9366 32.6495 13.3469 33.3187 14.0141 33.9859C14.6812 34.653 15.3504 35.0633 16.1395 35.3699C16.9028 35.6666 17.7752 35.8694 19.0524 35.9277C20.3323 35.9861 20.741 35.9999 24 35.9999C27.259 35.9999 27.6677 35.9861 28.9476 35.9277C30.2248 35.8694 31.0972 35.6666 31.8605 35.3699C32.6496 35.0633 33.3188 34.653 33.9859 33.9859C34.653 33.3187 35.0634 32.6495 35.37 31.8604C35.6667 31.0971 35.8695 30.2248 35.9278 28.9475C35.9862 27.6676 36 27.259 36 23.9999C36 20.741 35.9862 20.3323 35.9278 19.0524C35.8695 17.7751 35.6667 16.9028 35.37 16.1395C35.0634 15.3504 34.653 14.6812 33.9859 14.0141C33.3188 13.3469 32.6496 12.9366 31.8605 12.63C31.0972 12.3333 30.2248 12.1305 28.9476 12.0722C27.6677 12.0138 27.259 12 24 12Z" fill="white" {
            }
            path d="M24.0059 17.8433C20.6026 17.8433 17.8438 20.6021 17.8438 24.0054C17.8438 27.4087 20.6026 30.1675 24.0059 30.1675C27.4092 30.1675 30.1681 27.4087 30.1681 24.0054C30.1681 20.6021 27.4092 17.8433 24.0059 17.8433ZM24.0059 28.0054C21.7968 28.0054 20.0059 26.2145 20.0059 24.0054C20.0059 21.7963 21.7968 20.0054 24.0059 20.0054C26.2151 20.0054 28.0059 21.7963 28.0059 24.0054C28.0059 26.2145 26.2151 28.0054 24.0059 28.0054Z" fill="white" {
            }
            path d="M31.8507 17.5963C31.8507 18.3915 31.206 19.0363 30.4107 19.0363C29.6154 19.0363 28.9707 18.3915 28.9707 17.5963C28.9707 16.801 29.6154 16.1562 30.4107 16.1562C31.206 16.1562 31.8507 16.801 31.8507 17.5963Z" fill="white" {
            }
        }
    }
}

pub fn twitter_icon_color() -> Markup {
    html! {
        svg width="28px" height="28px" viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" {
            circle cx="512" cy="512" r="512" style="fill:#1da1f2" {
            }
            path d="M778 354.8c-18.8 8.3-38.9 13.9-60.1 16.5 21.6-13 38.2-33.5 46-57.9-20.2 11.8-42.7 20.4-66.5 25.2-19.1-20.4-46.2-33.2-76.4-33.2-57.8 0-104.7 46.9-104.7 104.6 0 8.3 1 16.3 2.7 23.9-87-4.1-164.2-45.9-215.8-109.1-9.1 15.4-14.2 33.2-14.2 52.7 0 36.4 18.5 68.4 46.6 87.2-17.2-.6-33.3-5.3-47.4-13.1v1.3c0 50.8 36 93.1 84 102.7-8.8 2.4-18.1 3.6-27.6 3.6-6.7 0-13.1-.6-19.5-1.8 13.4 41.6 52 71.9 98 72.7-35.7 28.1-81.1 44.8-129.8 44.8-8.3 0-16.6-.5-24.9-1.4 46.6 29.7 101.5 47 160.8 47C621.7 720.5 727 561 727 422.9c0-4.4 0-8.9-.3-13.4 20.4-14.7 38.3-33.2 52.3-54.2l-1-.5z" style="fill:#fff" {
            }
        }
    }
}

pub fn linkedin_icon_color() -> Markup {
    html! {
        svg width="38px" height="38px" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" {
            circle cx="24" cy="24" r="20" fill="#0077B5" {
            }
            path fill-rule="evenodd" clip-rule="evenodd" d="M18.7747 14.2839C18.7747 15.529 17.8267 16.5366 16.3442 16.5366C14.9194 16.5366 13.9713 15.529 14.0007 14.2839C13.9713 12.9783 14.9193 12 16.3726 12C17.8267 12 18.7463 12.9783 18.7747 14.2839ZM14.1199 32.8191V18.3162H18.6271V32.8181H14.1199V32.8191Z" fill="white" {
            }
            path fill-rule="evenodd" clip-rule="evenodd" d="M22.2393 22.9446C22.2393 21.1357 22.1797 19.5935 22.1201 18.3182H26.0351L26.2432 20.305H26.3322C26.9254 19.3854 28.4079 17.9927 30.8101 17.9927C33.7752 17.9927 35.9995 19.9502 35.9995 24.219V32.821H31.4922V24.7838C31.4922 22.9144 30.8404 21.6399 29.2093 21.6399C27.9633 21.6399 27.2224 22.4999 26.9263 23.3297C26.8071 23.6268 26.7484 24.0412 26.7484 24.4574V32.821H22.2411V22.9446H22.2393Z" fill="white" {
            }
        }
    }
}

pub fn youtube_icon_color() -> Markup {
    html! {
        svg width="30" height="30" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" {
            g clip-path="url(#clip0_17_47)" {
                path d="M47.044 12.3709C46.7726 11.3497 46.2378 10.4178 45.493 9.66822C44.7483 8.91869 43.8197 8.37791 42.8003 8.1C39.0476 7.09091 24.0476 7.09091 24.0476 7.09091C24.0476 7.09091 9.04761 7.09091 5.29488 8.1C4.27547 8.37791 3.34693 8.91869 2.60218 9.66822C1.85744 10.4178 1.32262 11.3497 1.05124 12.3709C0.0476075 16.14 0.0476074 24 0.0476074 24C0.0476074 24 0.0476075 31.86 1.05124 35.6291C1.32262 36.6503 1.85744 37.5822 2.60218 38.3318C3.34693 39.0813 4.27547 39.6221 5.29488 39.9C9.04761 40.9091 24.0476 40.9091 24.0476 40.9091C24.0476 40.9091 39.0476 40.9091 42.8003 39.9C43.8197 39.6221 44.7483 39.0813 45.493 38.3318C46.2378 37.5822 46.7726 36.6503 47.044 35.6291C48.0476 31.86 48.0476 24 48.0476 24C48.0476 24 48.0476 16.14 47.044 12.3709Z" fill="#FF0302" {
                }
                path d="M19.1385 31.1373V16.8628L31.684 24.0001L19.1385 31.1373Z" fill="#FEFEFE" {
                }
            }
            defs {
                clipPath id="clip0_17_47" {
                    rect width="48" height="48" fill="white" {
                    }
                }
            }
        }
    }
}

pub fn certificate_icon() -> Markup {
    html! {
        svg width="40px" height="40px" viewBox="0 0 24 24" version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" {
            title {
                "certificate_fill"
            }
            g id="页面-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd" {
                g id="System" transform="translate(-334.000000, -240.000000)" {
                    g id="certificate_fill" transform="translate(334.000000, 240.000000)" {
                        path id="MingCute" d="M24,0 L24,24 L0,24 L0,0 L24,0 Z M12.5934901,23.257841 L12.5819402,23.2595131 L12.5108777,23.2950439 L12.4918791,23.2987469 L12.4918791,23.2987469 L12.4767152,23.2950439 L12.4056548,23.2595131 C12.3958229,23.2563662 12.3870493,23.2590235 12.3821421,23.2649074 L12.3780323,23.275831 L12.360941,23.7031097 L12.3658947,23.7234994 L12.3769048,23.7357139 L12.4804777,23.8096931 L12.4953491,23.8136134 L12.4953491,23.8136134 L12.5071152,23.8096931 L12.6106902,23.7357139 L12.6232938,23.7196733 L12.6232938,23.7196733 L12.6266527,23.7031097 L12.609561,23.275831 C12.6075724,23.2657013 12.6010112,23.2592993 12.5934901,23.257841 L12.5934901,23.257841 Z M12.8583906,23.1452862 L12.8445485,23.1473072 L12.6598443,23.2396597 L12.6498822,23.2499052 L12.6498822,23.2499052 L12.6471943,23.2611114 L12.6650943,23.6906389 L12.6699349,23.7034178 L12.6699349,23.7034178 L12.678386,23.7104931 L12.8793402,23.8032389 C12.8914285,23.8068999 12.9022333,23.8029875 12.9078286,23.7952264 L12.9118235,23.7811639 L12.8776777,23.1665331 C12.8752882,23.1545897 12.8674102,23.1470016 12.8583906,23.1452862 L12.8583906,23.1452862 Z M12.1430473,23.1473072 C12.1332178,23.1423925 12.1221763,23.1452606 12.1156365,23.1525954 L12.1099173,23.1665331 L12.0757714,23.7811639 C12.0751323,23.7926639 12.0828099,23.8018602 12.0926481,23.8045676 L12.108256,23.8032389 L12.3092106,23.7104931 L12.3186497,23.7024347 L12.3186497,23.7024347 L12.3225043,23.6906389 L12.340401,23.2611114 L12.337245,23.2485176 L12.337245,23.2485176 L12.3277531,23.2396597 L12.1430473,23.1473072 Z" fill-rule="nonzero" {
                        }
                        path id="形状" d="M10.5857,2.10056 C11.3256895,1.36061789 12.5011493,1.32167357 13.2868927,1.98372704 L13.4141,2.10056 L15.3136,4.00005 L17.9999,4.00005 C19.0542909,4.00005 19.9180678,4.81592733 19.9944144,5.85078759 L19.9999,6.00005 L19.9999,8.68632 L21.8994,10.5858 C22.6393895,11.3257895 22.6783363,12.5012493 22.0162404,13.2870778 L21.8994,13.4143 L19.9999,15.3138 L19.9999,18.0001 C19.9999,19.0543955 19.18405,19.9182591 18.1491661,19.9946139 L17.9999,20.0001 L15.3136,20.0001 L13.4141,21.8995 C12.6742053,22.6394895 11.4987504,22.6784363 10.7129222,22.0163404 L10.5857,21.8995 L8.68622,20.0001 L5.99991,20.0001 C4.94554773,20.0001 4.08174483,19.1841589 4.00539573,18.1493537 L3.99991,18.0001 L3.99991,15.3137 L2.10043,13.4143 C1.36049737,12.6743105 1.32155355,11.4988507 1.98360703,10.7130222 L2.10044,10.5858 L3.99991,8.68636 L3.99991,6.00005 C3.99991,4.94568773 4.81578733,4.08188483 5.85064759,4.00553573 L5.99991,4.00005 L8.68622,4.00005 L10.5857,2.10056 Z M15.0794,8.98261 L10.8348,13.2271 L9.06704,11.4594 C8.67652,11.0689 8.04336,11.0689 7.65283,11.4594 C7.26231,11.8499 7.26231,12.4831 7.65283,12.8736 L10.057,15.2778 C10.4866,15.7073 11.1831,15.7073 11.6126,15.2778 L16.4936,10.3968 C16.8841,10.0063 16.8841,9.37314 16.4936,8.98261 C16.103,8.59209 15.4699,8.59209 15.0794,8.98261 Z" fill="#09244B" {
                        }
                    }
                }
            }
        }
    }
}
