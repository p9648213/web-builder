use vy::PreEscaped;

pub fn render_main_app_header<'a>() -> PreEscaped<&'a str> {
    vy::lazy! {
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width,initial-scale=1" />
        <link rel="stylesheet" href="/assets/css/lib/tailwind.css" />
        <link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico" />
        <script src="/assets/js/lib/htmx.js" defer="defer"></script>
        <script src="/assets/js/main.js" defer="defer"></script>
    }
}
