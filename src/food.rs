use crate::snake::Snake;
use rand::Rng; // 导入随机数生成器
pub struct Food {
    pub position: (f64, f64, f64, f64),
}

impl Food {
    // 初始化食物位置
    pub fn new() -> Self {
        Food {
            position: (300.0, 300.0, 10.0, 10.0),
        }
    }

    pub fn update(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();
        // 随机生成食物位置
        loop {
            let x: f64 = rng.gen_range(0.0..=630.0); // 640 - 10
            let y: f64 = rng.gen_range(0.0..=470.0); // 480 - 10

            // 检查新生成的食物位置是否与蛇的任何部分重叠
            if !snake
                .segments
                .iter()
                .any(|&(seg_x, seg_y)| (seg_x - x).abs() < 10.0 && (seg_y - y).abs() < 10.0)
            {
                self.position = (x, y, 10.0, 10.0);
                break;
            }
        }
    }
}
