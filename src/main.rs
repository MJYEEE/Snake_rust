extern crate piston_window;

use food::Food;
use piston_window::*;
use snake::Snake;

mod food;
mod snake;

fn main() {
    /*
    创建一个名为 "Snake Game" 的窗口，大小为 640x480 像素。
    设置按下 ESC 键时退出程序。
    */
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    // 初始化蛇和食物
    let mut snake = Snake::new();
    let mut food = Food::new();
    // 主循环
    // 通过 window.next() 获取窗口事件，检查是否有键盘按键事件
    while let Some(event) = window.next() {
        // 检查是否有键盘按键事件，并根据按键方向改变蛇的方向。
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.change_direction((0.0, -1.0)),
                Key::Down => snake.change_direction((0.0, 1.0)),
                Key::Left => snake.change_direction((-1.0, 0.0)),
                Key::Right => snake.change_direction((1.0, 0.0)),
                _ => {}
            }
        }

        // 检查蛇的头部是否与食物位置重合，如果是，则重新生成食物位置并使蛇增长。
        if snake.check_collision_with_food(&food)
        {
            // 蛇头的上侧与食物的下侧相接触
            // 碰撞发生，生成新的食物位置并增长蛇
            food.update(&snake);
            snake.grow();
        } else {
            // 调用 snake.move_forward 方法使蛇向前移动。
            snake.move_forward();
        }
        // 绘制窗口
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            // 读取蛇的每一段身体
            for segment in &snake.segments {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // 红色
                    [segment.0, segment.1, snake.length, snake.width],
                    c.transform,
                    g,
                );
            }
            rectangle(
                [0.0, 0.0, 1.0, 1.0], // 蓝色
                [
                    food.position.0,
                    food.position.1,
                    food.position.2,
                    food.position.3,
                ],
                c.transform,
                g,
            );
        });
    }
}
