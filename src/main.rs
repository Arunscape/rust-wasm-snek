 #![warn(
     clippy::all)]

#[macro_use]
extern crate stdweb;



mod canvas;
use canvas::Canvas;

mod direction;

fn main(){
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);

    stdweb::event_loop();
}