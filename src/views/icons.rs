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
        svg class="lucide lucide-phone" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
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
