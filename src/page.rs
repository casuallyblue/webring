use maud::{html, Markup, Render, DOCTYPE};

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

fn intersperse(elements: Vec<impl Render>, divider: impl Render) -> Markup {
    html! {
        @if let Some(element) = elements.first() {
            span {(element)}
        }
        @for element in elements.iter().skip(1) {
            (divider)
            span {(element)}
        }
    }
}

pub fn navbar(elements: Vec<impl Render>) -> Markup {
    html! {
        nav .navbar {
            div .menu {
                (intersperse(elements, html!{span {" | "}}))
            }
        }
    }
}

pub fn gen_js_includes(scripts: Vec<impl Render>) -> Markup {
    html! {
        @for script in scripts {
            script src=(script) {}
        }
    }
}

#[allow(unused)]
fn flex_container(contents: Vec<Markup>) -> Markup {
    html! {
        div style="display: flex" {
            @for element in contents {
                (element)
            }
        }
    }
}
