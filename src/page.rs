use maud::{html, Markup, Render, DOCTYPE};

pub trait Page {
    fn render(&self) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                (self.head())
            }

            body {
                header {
                    (self.header())
                }
                div .wrapper {
                    (self.content())
                }
                footer  {
                    (self.footer())
                }
            }
        }
    }
    }

    fn head(&self) -> Markup;

    fn header(&self) -> Markup;
    fn content(&self) -> Markup;
    fn footer(&self) -> Markup;
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
