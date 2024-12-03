
use dioxus::prelude::*;
use dioxus_router::components;
use rand::Rng;
use crate::mineswep::*;

pub fn app() -> Element {

    rsx! {
        for i in 0..9{
            for j in 0..9{       
                Square{
                    x:i,y:j, color: "blue", text: format!("{i}x{j}"), is_bomb: {rand::thread_rng().gen_bool(0.5)},
                }
            }
        }
    }
}
#[component]
fn Square(x:i32, y:i32, color: String, text: String, is_bomb: bool) -> Element {
    rsx! {
        div { 
            class:"cell",
            style:"
            position: absolute;
            width: 10vmin;
            height: 10vmin;
            top: {10*y}vmin;
            left: {10*x}vmin;
            background-color: {color};
            
            border: 1px solid black;
  text-align: center;
  justify-content: center;
  font-size: 180%;
            ",
            {if is_bomb {
                rsx!{
                    Bomb {}
                }
            } else {
                rsx! {
                    "{text}"
                }
            }}
        }
    }
}

#[component]
fn Bomb() -> Element {
    let mut img_src = use_signal(|| "/assets/bmb.png");    
    rsx! {
        img {
            src: "{img_src}", style: "height:100%; width: 100%",
            onclick: move |_| {img_src.set("/assets/bmb2.png")}
        }
    }
}