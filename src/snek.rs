use rand::seq::SliceRandom;
use rand::Rng;

use crate::canvas::Canvas;
use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snek {
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
}

impl Snek {
    pub fn new(width: u32, height: u32) -> Snek {
        let mut rng = rand::thread_rng();

        let head_x: u32 = f64::floor(rng.gen::<f64>() * f64::from(width)) as u32;
        let head_y: u32 = f64::floor(rng.gen::<f64>() * f64::from(width)) as u32;
        let food_x: u32 = f64::floor(rng.gen::<f64>() * f64::from(width)) as u32;
        let food_y: u32 = f64::floor(rng.gen::<f64>() * f64::from(height)) as u32;
        let head = Block(head_x, head_y);
        let tail = Vec::new();
        let food = Block(food_x, food_y);

        let last_direction = [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ]
        .choose(&mut rng)
        .unwrap();

        Snek {
            head,
            tail,
            food,
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: *last_direction,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.last_direction.opposite(direction) && self.direction.is_none() {
            self.direction = Some(direction);
        } else if self.direction.iter().any(|d| !d.opposite(direction)) {
            self.next_direction = Some(direction);
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            Direction::Down => Block((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
            Direction::Right => Block((self.head.0 + 1) % self.width, (self.head.1) % self.height),
        };

        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        if self.tail.contains(&new_head) {
            *self = Snek::new(self.width, self.height);
        }

        js! { console.log( "Snek's head: ", "X:", @{self.head.0}, "Y:", @{self.head.1} ) }
        self.head = new_head;
        if self.head == self.food {
            let mut food = self.food;
            while food == self.head || self.tail.contains(&food) {
                let mut rng = rand::thread_rng();
                let food_x: u32 = f64::floor(rng.gen::<f64>() * self.width as f64) as u32;
                let food_y: u32 = f64::floor(rng.gen::<f64>() * self.height as f64) as u32;

                food = Block(food_x, food_y);
            }
            self.food = food;
            last_end.map(|x| self.tail.push(x));
        }
        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "green");

        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "lightgreen");
        }

        canvas.draw(self.food.0, self.food.1, "red");
    }
}
