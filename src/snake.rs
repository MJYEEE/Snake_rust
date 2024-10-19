use crate::food::Food;

/*
蛇结构体
segments：存储蛇的身体部分，每个部分是一个二维坐标(f64, f64)
direction： 存储蛇的移动方向，也是一个二维坐标  (f64, f64)
length,width：蛇每个身体段的大小
speed：蛇的移动速度
*/
pub struct Snake {
    pub segments: Vec<(f64, f64)>,
    pub direction: (f64, f64),
    pub length: f64,
    pub width: f64,
    pub speed: f64,
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
    设置蛇的初始方向 direction 为向右移动，即 (10.0, 0.0)
    */
    pub fn new() -> Self {
        Snake {
            segments: vec![(100.0, 50.0), (90.0, 50.0)],
            direction: (1.0, 0.0),
            length: 10.0,
            width: 10.0,
            speed: 0.5,
        }
    }

    /*
    head：蛇的头位置segments[0]
    new_head：下一步蛇头的位置head+direction
    将新的蛇头位置插入到蛇身体的最前面
    移除蛇身体的最后一段，保持蛇的长度不变。
    */
    pub fn move_forward(&mut self) {
        let head = self.segments[0];
        let new_head = (
            head.0 + self.direction.0 * self.speed,
            head.1 + self.direction.1 * self.speed,
        );
        self.segments.insert(0, new_head);
        self.segments.pop();
    }

    /*
    刷新蛇的前进方向
    */
    pub fn change_direction(&mut self, new_direction: (f64, f64)) {
        self.direction = new_direction;
    }

    /*
    判断蛇是否与食物重叠
    */    
    pub fn check_collision_with_food(&self, food: &Food) -> bool {
        let head: (f64, f64) = self.segments[0];
        head.0 < food.position.0 + food.position.2
            && head.0 + self.length > food.position.0
            && head.1 < food.position.1 + food.position.3
            && head.1 + self.width > food.position.1
    }

    /*
    head：蛇的头位置segments[0]
    new_head：下一步蛇头的位置head+direction
    将新的蛇头位置插入到蛇身体的最前面
    但是不会移除蛇身体的最后一段，增长蛇的总体长度
    同时加快蛇的移动速度
    */
    pub fn grow(&mut self) {
        let head: (f64, f64) = self.segments[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.segments.insert(0, new_head);
        self.speed += 0.01;
    }
}
