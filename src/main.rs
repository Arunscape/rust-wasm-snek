#![warn(clippy::all)]

mod canvas;
mod direction;
mod snek;

#[macro_use]
extern crate stdweb;

use canvas::Canvas;
use direction::Direction;
use snek::Snek;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);
    let snek = Rc::new(RefCell::new(Snek::new(20, 20)));
    snek.borrow().draw(&canvas);

    stdweb::web::document().add_event_listener({
        let snek = snek.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snek.borrow_mut().change_direction(Direction::Left),
                "ArrowRight" => snek.borrow_mut().change_direction(Direction::Right),
                "ArrowUp" => snek.borrow_mut().change_direction(Direction::Up),
                "ArrowDown" => snek.borrow_mut().change_direction(Direction::Down),
                _ => {}
            };
        }
    });

    fn game_loop(snek: Rc<RefCell<Snek>>, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(snek.clone(), canvas.clone(), time);
                snek.borrow_mut().update();
                snek.borrow().draw(&canvas);
            },
            time,
        )
    }

    game_loop(snek, Rc::new(canvas), 100);
    stdweb::event_loop();
}
