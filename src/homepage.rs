use crate::page::Page;
use crate::page::{self, gen_js_includes, navbar};
use maud::{html, Markup};

pub struct HomePage {}

impl HomePage {
    fn head(&self) -> Markup {
        html! {
            (gen_js_includes(vec!["/js/htmx.min.js", "/js/hyperscript.min.js"]))

            link rel="stylesheet" type="text/css" href="/css/main.css" {}
        }
    }

    fn header(&self) -> Markup {
        let navbar = navbar(vec![
            html! {a href="/" {"Home"}},
            html! {a href="https://git.casuallyblue.dev" {"Git Server"}},
            html! {a href="/static/resume.pdf" {"Resume"}},
        ]);

        html! { header {
            h1 ."text-center" { "Home Page" }
            hr {}
            (navbar)
            hr {}
        }}
    }

    fn body(&self) -> Markup {
        html! {
            (self.header())
            div ."page-body" {
                p { "
                    
                    " }
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
        page::basic_page(self.head(), self.body())
    }
}
