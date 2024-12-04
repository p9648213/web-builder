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
