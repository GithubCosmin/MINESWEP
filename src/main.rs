//! The simplest example of a Dioxus app.
//!
//! In this example we:
//! - import a number of important items from the prelude (launch, Element, rsx, div, etc.)
//! - define a main function that calls the launch function with our app function
//! - define an app function that returns a div element with the text "Hello, world!"
//!
//! The `launch` function is the entry point for all Dioxus apps. It takes a function that returns an Element. This function
//! calls "launch" on the currently-configured renderer you have. So if the `web` feature is enabled, it will launch a web
//! app, and if the `desktop` feature is enabled, it will launch a desktop app.

use dioxus::prelude::*;
use dioxus_router::components;
use rand::Rng;

fn main() {
    dioxus::launch(app);
}

const BOARD_X : usize = 9;
const BOARD_Y : usize = 9;
const BOMB_COUNT: usize = 9;

fn app() -> Element {

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
                rsx! {
                    img {
                        src: "/assets/bmb.png", style: "height:100%; width: 100%",
                    }
                }
            } else {
                rsx! {
                    "{text}"
                }
            }}
        }
    }
}