use maud::{html, Markup};
use crate::page;
use crate::page::Page;

pub struct HomePage {
    
}

fn elements() -> Vec<Markup> {
    vec![
  html!{a href="/" {"Home"}},
  html!{a href="https://git.casuallyblue.dev" {"Git Server"}},
  html!{a href="/static/resume.pdf" {"Resume"}}
    ]
}

fn navbar(elements: Vec<Markup>) -> Markup {
    let divider = html!{span {" | "}};
    html! {
            nav .navbar {
                div .menu {
                    @if let Some(element) = elements.first() {
                        span {(element)}
                    }
                    @for element in elements.iter().skip(1) {
                        (divider)
                        span {(element)}
                    }
                }
            }
        }
}

impl HomePage {
    fn head(&self) -> Markup {
        let js_includes = html! {
            script src="/js/htmx.min.js" {}
            script src="/js/hyperscript.min.js" {}
        };

        let stylesheets = html! {
            link rel="stylesheet" type="text/css" href="/css/main.css" {}
        };

        html! {
            (js_includes)  
            (stylesheets)
        }
    }



    fn header(&self) -> Markup {
        html! { header {
            h1 ."text-center" { "Home Page" }
            hr {}
            (navbar(elements()))
            hr {}
        }}
    }

    fn body(&self) -> Markup {
        html! {
            (self.header())
            div ."page-body" {
                p { "Hi, this is my site" }
            }
            (self.footer())
        }
    }

    fn footer(&self) -> Markup {
        html! { footer {
                hr {}
                p {"Built with nix/cargo"}
                p {"Source " a href="https://git.casuallyblue.dev/sierra/nix-flakes/site"{"here"}}
        }}
    }
}

impl Page for HomePage {
    fn render(&self) -> Markup {
        page::basic_page(
            self.head(),
            self.body()
        )
    }
}
