use maud::{html, Markup};

pub fn arrow_next_icon() -> Markup {
    html! {
        svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" {}
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
        svg class="lucide lucide-bed" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
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
        svg class="lucide lucide-bath" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
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
        svg class="lucide lucide-table-2" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
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
