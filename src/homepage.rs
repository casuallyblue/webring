use crate::page::Page;
use crate::page::{self, gen_js_includes, navbar};
use maud::{html, Markup, Render};

pub struct HomePage {}

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

impl HomePage {
    fn head(&self) -> Markup {
        html! {
            title { "My Home Page"}
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
                target: "https://git.casuallyblue.dev",
                text: "Git Server",
            },
            Link {
                target: "/static/resume.pdf",
                text: "Resume",
            },
        ]);

        html! { header {
            h1 ."text-center" { "Home Page" }
            hr;
            (navbar)
            hr;
        }}
    }

    fn social_link(text: &str, uri: &str) -> Markup {
        html! {
            a href=(uri) {(text)}
        }
    }

    fn body(&self) -> Markup {
        html! {
            (self.header())
            div ."page-body" {
                h1 {"About Me"}
                p { "
                    Hi, I'm Sierra/casuallyblue, and I do pretty much anything tech related that I find interesting. 
                    I do a lot of low level development stuff as well as working on compilers and other dev tools. 
                    " }
            }

            div ."contact" {
                h3 {"On Other sites"}
                table {
                    tr {
                        th {"Service"}
                        th {"Username"}
                    }

                    tr {
                        td { "IRC"}
                        td { "casuallyblue" }
                    }

                    tr {
                        td {"Matrix"}
                        td {
                        (Self::social_link("@sierra@synapse.casuallyblue.dev", "https://matrix.to/#/@sierra:synapse.casuallyblue.dev"))
                        }
                    }

                    tr {
                        td {"Fediverse"}
                        td {
                            (Self::social_link("@sierra@social.casuallyblue.dev", "https://social.casuallyblue.dev/sierra"))
                        }
                    }

                    tr {
                        td {"Email"}
                        td {
                            (Self::social_link("sierra@casuallyblue.dev", "mailto:sierra@casuallyblue.dev"))
                        }
                    }

                    tr {
                        td { "Github"}
                        td {
                            (Self::social_link("casually-blue", "https://github.com/casually-blue"))
                        }
                    }

                    tr {
                        td {"Gitea"}
                        td {
                            (Self::social_link("casuallyblue", "https://git.casuallyblue.dev"))
                        }
                    }
                }
            }
            (self.footer())
        }
    }

    fn footer(&self) -> Markup {
        html! { footer {
                hr;
                p {"Built with nix/cargo"}
                p {"Source " a href="https://git.casuallyblue.dev/sierra/nix-flakes/site"{"here"}}
                div ."buttons-88x31" {
                    img src="/images/casually-blue.gif" alt="A gradient from purple to blue with a grey border and the username CasuallyBlue written on it";
                }
        }}
    }
}

impl Page for HomePage {
    fn render(&self) -> Markup {
        page::basic_page(self.head(), self.body())
    }
}
