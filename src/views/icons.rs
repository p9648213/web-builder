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

pub fn next_icon_large() -> Markup {
    html! {
        svg class="lucide-chevron-right lucide" stroke-linejoin="round" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke-width="2" width="36" height="36" stroke-linecap="round" {
            path d="m9 18 6-6-6-6" {}
        }
    }
}

pub fn previous_icon_large() -> Markup {
    html! {
        svg class="lucide-chevron-left lucide" stroke-width="2" width="36" viewBox="0 0 24 24" stroke-linecap="round" fill="none" stroke-linejoin="round" height="36" xmlns="http://www.w3.org/2000/svg" {
            path d="m15 18-6-6 6-6" {}
        }
    }
}
