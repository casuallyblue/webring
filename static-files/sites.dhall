let Site = { name : Text, url : Text, author : Text, feed : Optional Text }

let sites
    : List Site
    = [ { name = "sierra"
        , url = "https://casuallyblue.dev/"
        , author = "Sierra <sierra@casuallyblue.dev>"
        , feed = Some "https://casuallyblue.dev/feed.xml"
        }
      , { name = "jadedctrl"
        , url = "https://xwx.moe/"
        , author = "jadedctrl <jadedctrl@posteo.net>"
        , feed = None Text
        }
      , { name = "gaffclant"
        , url = "https://gaffclant.dev"
        , author = "gaffclant <gaffclant@gaffclant.dev>"
        , feed = Some "https://gaffclant.dev/blog/feed.rss"
        }
      ]

in  sites
