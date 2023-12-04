use crate::page::Page;
use crate::page::{self, gen_js_includes, navbar};
use maud::{html, Markup, Render};

pub struct MembersPage {
    pub state: std::sync::Arc<crate::AppState>
}

struct Link<'a> {
    pub target: &'a str,
    pub text: &'a str,
}

impl<'a> Render for Link<'a> {
    fn render(&self) -> Markup {
        html! {
            a href=(self.target) {(self.text)}
        }
    }
}

impl Page for MembersPage {
    fn head(&self) -> Markup {
        html! {
            title { "Webring"}
            (gen_js_includes(vec!["/js/htmx.min.js", "/js/hyperscript.min.js"]))

            link rel="stylesheet" type="text/css" href="/css/main.css";
        }
    }

    fn header(&self) -> Markup {
        let navbar = navbar(vec![
            Link {
                target: "/",
                text: "Home",
            },
            Link {
                target: "/members",
                text: "Members",
            },
            Link {
                target: "/join",
                text: "Join",
            },
        ]);

        html! { 
            h1 ."text-center" { "Webring" }
            hr;
            (navbar)
            hr;
        }
    }

    fn footer(&self) -> Markup {
        html! { 
            hr;
            p {"Built with nix/cargo"}
            p {"Source " a href="https://git.casuallyblue.dev/flakes/webring"{"here"}}      
        }
    }

    fn content(&self) -> Markup {
       html!{} 
    }
}
