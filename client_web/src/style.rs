use stylist::css;
use stylist::yew::Global;
use yew::{html, Html};

pub fn global_style() -> Html {
    html! {
        <Global css={css!(r#"
        * {
            font-family:Cubano;
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        html {
            overflow-y: scroll;
            height: 100%;
            font: 100%/1.5 sans-serif;
            word-wrap: break-word;
            margin: 0 auto;
            padding: 1.5em;
        }

        @media (min-width: 768px) {
            html {
                font-size: 125%;
                max-width: 42em;
        } }

        h1, h2, h3, h4 {
            margin: 2.5rem 0 1.5rem 0;
            line-height: 1.25;
            color: #333;
        }

        a {
            color: #fa6432;
            text-decoration: none;
        }
        a:hover, a:focus, a:active {
            text-decoration: underline;
        }

        p {
            margin: 1em 0;
            line-height: 1.5;
        }
        p code {
            background-color: #eee;
            padding: 0.05em 0.2em;
            border: 1px solid #ccc;
        }

        ol, ul {
            margin: 1em;
        }
        ol li ol, ol li ul, ul li ol, ul li ul {
            margin: 0 2em;
        }
        ol li p, ul li p {
            margin: 0;
        }

        dl {
            font-family: monospace, monospace;
        }
        dl dt {
            font-weight: bold;
        }
        dl dd {
            margin: -1em 0 1em 1em;
        }

        img {
            max-width: 100%;
            display: block;
            margin: 0 auto;
            padding: 0.5em;
        }

        blockquote {
            padding-left: 1em;
            font-style: italic;
            border-left: solid 1px #fa6432;
        }

        table {
            font-size: 1rem;
            text-align: left;
            caption-side: bottom;
            margin-bottom: 2em;
        }
        table * {
            border: none;
        }
        table thead, table tr {
            display: table;
            table-layout: fixed;
            width: 100%;
        }
        table tr:nth-child(even) {
            background-color: rgba(200, 200, 200, 0.2);
        }
        table tbody {
            display: block;
            max-height: 70vh;
            overflow-y: auto;
        }
        table td, table th {
            padding: 0.25em;
        }

        table, .highlight > pre, pre.example {
            max-height: 70vh;
            margin: 1em 0;
            padding: 1em;
            overflow: auto;
            font-size: 0.85rem;
            font-family: monospace, monospace;
            border: 1px dashed rgba(250, 100, 50, 0.5);
        }

        .title {
            font-size: 2.5em;
        }

        .subtitle {
            font-weight: normal;
            font-size: 0.75em;
            color: #666;
        }

        .tags {
            margin-top: -1.5rem;
            padding-bottom: 1.5em;
        }
        .tags li {
            display: inline;
            margin-right: 0.5em;
        }

        figure {
            margin: 1em 0;
        }
        figure figcaption {
            font-family: monospace, monospace;
            font-size: 0.75em;
            text-align: center;
            color: grey;
        }

        .footnote-definition sup {
            margin-left: -1.5em;
            float: left;
        }

        .footnote-definition .footnote-body {
            margin: 1em 0;
            padding: 0 1em;
            border: 1px dashed rgba(250, 100, 50, 0.3);
            background-color: rgba(200, 200, 200, 0.2);
        }
        .footnote-definition .footnote-body p:only-child {
            margin: 0.2em 0;
        }

        header {
            display: flex;
            justify-content: space-between;
        }
        header nav {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        header a + a {
            margin-left: 1rem;
        }

        .posts {
            margin: 0;
            list-style: none;
        }
        .posts .post a {
            display: flex;
            padding: 0.5em 0;
            color: black;
        }
        .posts .post a:hover, .posts .post a:focus, .posts .post a:active {
            text-decoration: none;
            background: rgba(200, 200, 200, 0.2);
        }
        .posts .post date {
            font-family: monospace, monospace;
            font-size: 0.8rem;
            vertical-align: middle;
            padding-right: 2rem;
            color: grey;
        }       
    "#)} />
    }
}
