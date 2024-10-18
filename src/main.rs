extern crate piston_window;

use piston_window::*;


/*
蛇结构体
segments：存储蛇的身体部分，每个部分是一个二维坐标(f64, f64)
direction： 存储蛇的移动方向，也是一个二维坐标  (f64, f64)
*/
struct Snake {
    segments: Vec<(f64, f64)>,
    direction: (f64, f64),
}

/*  
食物结构体
position： 存储食物的位置，前面两个变量表示食物的左上角坐标，后面两个变量表示食物的宽度和高度
*/
struct Food {
    position: (f64, f64, f64, f64),
}

/*
Snake的方法
move_forward： 移动蛇，将蛇头移动到下一个位置，并将蛇尾删除
change_direction： 更改蛇的方向
grow： 蛇长一节
*/
impl Snake {
    /* 
    初始化蛇的身体部分segments，包含两个初始位置 (100.0, 50.0) 和 (90.0, 50.0)
    设置蛇的初始方向 direction 为向右移动，即 (1.0, 0.0)
    */ 
    fn new() -> Self {
        Snake {
            segments: vec![(100.0, 50.0), (90.0, 50.0)],
            direction: (1.0, 0.0),
        }
    }

    /*
    head：蛇的头位置segments[0]
    new_head：下一步蛇头的位置head+direction
    将新的蛇头位置插入到蛇身体的最前面
    移除蛇身体的最后一段，保持蛇的长度不变。
    */
    fn move_forward(&mut self) {
        let head = self.segments[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.segments.insert(0, new_head);
        self.segments.pop();
    }

    /*
    刷新蛇的前进方向
    */
    fn change_direction(&mut self, new_direction: (f64, f64)) {
        self.direction = new_direction;
    }

    /*
    head：蛇的头位置segments[0]
    new_head：下一步蛇头的位置head+direction
    将新的蛇头位置插入到蛇身体的最前面
    但是不会移除蛇身体的最后一段，增长蛇的总体长度
    */
    fn grow(&mut self) {
        let head = self.segments[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.segments.insert(0, new_head);
    }
}

impl Food {
    /*
    初始化食物的位置，初始位置为 (300.0, 300.0)
    */
    fn new() -> Self {
        Food {
            position: (300.0, 300.0, 15.0, 15.0),
        }
    }
}


fn main() {
    // 初始化窗口
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    // 初始化蛇和食物
    let mut snake = Snake::new();
    let mut food = Food::new();
    // 主循环
    // 通过 window.next() 获取窗口事件，检查是否有键盘按键事件
    while let Some(event) = window.next() {
        // 
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.change_direction((0.0, -1.0)),
                Key::Down => snake.change_direction((0.0, 1.0)),
                Key::Left => snake.change_direction((-1.0, 0.0)),
                Key::Right => snake.change_direction((1.0, 0.0)),
                _ => {}
            }
        }
        // 调用 snake.move_forward 方法使蛇向前移动。
        snake.move_forward();
        // 检查蛇的头部是否与食物位置重合，如果是，则重新生成食物位置并使蛇增长。
        if  snake.segments[0].0 >= food.position.0 
            && snake.segments[0].0 <= (food.position.0 + food.position.2) 
            && snake.segments[0].1 >= food.position.1 
            && snake.segments[0].1 <= (food.position.1 + food.position.3) {
            food.position = (rand::random::<f64>() * 640.0, rand::random::<f64>() * 480.0, 15.0, 15.0);
            snake.grow();
        }
        // 绘制窗口
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