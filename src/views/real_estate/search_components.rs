use maud::{html, Markup};

pub fn render_search_box() -> Markup {
    html! {
      div class="flex justify-center items-center mt-30" {
        div class="flex justify-center px-15 py-15 w-full max-w-360" {
          "Search Box"
        }
      }
    }
}
