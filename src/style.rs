use stylist::css;
use stylist::yew::Global;
use yew::{html, Html};

pub fn global_style() -> Html {
    html! {
        <Global css={css!(r#"
            body {
                background-color:black;
            }

            div {
                margin-top:35px;
                text-align:center;
                color:white;
                font-family:Cubano;
            }

            p {
                font-size:15em;
            }

            button {
                width:50%;
                height:500px;
                font-size:18em;
                font-family:Cubano;
                color:white;
                text-align:center;
                background-color:#555555;
                border-width:8px;
            }
    "#)} />
    }
}
