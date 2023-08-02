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
                div .wrapper {
                    (body)                
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
            hr {}
        }}
    }

    fn keys_div(&self) -> Markup {
        html! {
            div ."key-container" {
                button ."text-center" #keys 
                    hx-trigger="click"
                    hx-get="/keys"
                    hx-swap="outerHTML"
                    hx-indicator="#keys-loading"
                    hx-target=".key-container"
                    { "Show SSH Pubkeys" }

                p #"keys-loading" .htmx-indicator { "loading..." }
            }        
        }
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
                (self.keys_div())    
        }}
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

impl Page for HomePage {
    fn page(&self) -> Markup {
        basicPage(
            self.head(),
            self.body()
        )
    }
}