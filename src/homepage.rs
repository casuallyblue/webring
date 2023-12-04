use crate::page::Page;
use crate::page::{self, gen_js_includes, navbar};
use maud::{html, Markup, Render};

pub struct HomePage {
    pub state: std::sync::Arc<crate::AppState>,
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

type VecVec<T> = Vec<Vec<T>>;

impl Page for HomePage {
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
        let posts = self
            .state
            .sites
            .iter()
            .filter(|site| site.feed.is_some())
            .map(|site| {
                let feed_url = site.feed.clone().unwrap();
                let Ok(content) = reqwest::blocking::get(feed_url.as_str()) else {
                return None;
            };

                let Ok(content) = content.bytes() else {
                return None;
            };

                let Ok(channel) = rss::Channel::read_from(&content[..]) else {
                return None;
            };

                Some(channel)
            })
            .filter(|channel| channel.is_some())
            .map(|channel| channel.unwrap())
            .map(|feed| feed.items.clone())
            .flatten()
            .collect::<Vec<rss::Item>>();

        html! {
            (format!("{:?}", posts))
        }
    }
}
