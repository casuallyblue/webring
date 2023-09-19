use maud::{html, Markup, DOCTYPE};

pub trait Page {
    fn render(&self) -> Markup;
}

pub fn basic_page(head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                (head)
            }

            body {
                div .wrapper {
                    (body)
                }
            }
        }
    }
}

fn flex_container(contents: Vec<Markup>) -> Markup {
    html! {
        div style="display: flex" {
            @for element in contents {
                (element)
            }
        }
    }
}
