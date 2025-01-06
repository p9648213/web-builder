use maud::{html, Markup};

pub fn arrow_next_icon() -> Markup {
    html! {
        svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" {}
        }
    }
}

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

pub fn mail_icon() -> Markup {
    html! {
        svg class="lucide lucide-mail" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            rect width="20" height="16" x="2" y="4" rx="2" {
            }
            path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" {
            }
        }
    }
}

pub fn phone_icon() -> Markup {
    html! {
        svg class="lucide lucide-phone" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" {
            path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" {
            }
        }
    }
}

pub fn drop_down_icon() -> Markup {
    html! {
        svg class="lucide lucide-chevron-down" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            path d="m6 9 6 6 6-6" {
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
