extern crate piston_window;

use piston_window::*;

struct Snake {
    segments: Vec<(f64, f64)>,
    direction: (f64, f64),
}

struct Food {
    position: (f64, f64),
}

impl Snake {
    fn new() -> Self {
        Snake {
            segments: vec![(100.0, 50.0), (90.0, 50.0)],
            direction: (1.0, 0.0),
        }
    }

    fn move_forward(&mut self) {
        let head = self.segments[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.segments.insert(0, new_head);
        self.segments.pop();
    }

    fn change_direction(&mut self, new_direction: (f64, f64)) {
        self.direction = new_direction;
    }

    fn grow(&mut self) {
        let head = self.segments[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.segments.insert(0, new_head);
    }
}

impl Food {
    fn new() -> Self {
        Food {
            position: (300.0, 300.0),
        }
    }
}


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake = Snake::new();
    let mut food = Food::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.change_direction((0.0, 1.0)),
                Key::Down => snake.change_direction((0.0, -1.0)),
                Key::Left => snake.change_direction((-1.0, 0.0)),
                Key::Right => snake.change_direction((1.0, 0.0)),
                _ => {}
            }
        }

        snake.move_forward();

        if snake.segments[0] == food.position {
            food.position = (rand::random::<f64>() * 640.0, rand::random::<f64>() * 480.0);
            snake.grow();
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for segment in &snake.segments {
                rectangle([1.0, 0.0, 0.0, 1.0], // Red color
                         [segment.0, segment.1, 10.0, 10.0], c.transform, g);
            }
            rectangle([0.0, 0.0, 1.0, 1.0], // Blue color
                     [food.position.0, food.position.1, 10.0, 10.0], c.transform, g);
        });
    }
}