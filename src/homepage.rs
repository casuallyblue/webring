use maud::{Markup, html, DOCTYPE};

pub trait Page {
    fn page(&self) -> Markup;
}

pub struct HomePage {
    
}

fn basicPage(head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                (head)
            }

            body {
                (body)                
            }
        }
    }
}

impl HomePage {
    fn head_contents(&self) -> Markup {
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

    fn navbar(&self) -> Markup {
        let divider = html!{span {" | "}};
        html! {
            nav .navbar {
                div .menu {
                    span { a href="/" {"Home"} }
                    (divider)
                    span { a href="https://git.casuallyblue.xyz" {"Git Server"} }
                }
            }
        }
    }

    fn header(&self) -> Markup {
        html! { header {
            h1 ."text-center" { "Home Page" }
            hr {}
            (self.navbar())
        }}
    }

    fn body(&self) -> Markup {
        html! {
            div ."key-container" {
                button ."text-center" #keys 
                    hx-trigger="click"
                    hx-get="/keys"
                    hx-swap="outerHTML"
                    hx-indicator="#keys-loading"
                    { "Get Keys" }

                p #"keys-loading" .htmx-indicator { "loading..." }
            }
        }
    }
}

impl Page for HomePage {
    fn page(&self) -> Markup {
        basicPage(self.head_contents(),
        html! {
            (self.header())
            (self.body())
        })
    }
}