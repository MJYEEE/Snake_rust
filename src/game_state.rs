pub enum GameState {
    Running,
    Paused,
    GameOver,
}

impl GameState {
    // 初始化游戏状态为运行中
    pub fn new() -> Self {
        GameState::Running
    }

    // 切换为暂停状态
    pub fn pause(&mut self) {
        if let GameState::Running = self {
            *self = GameState::Paused;
        }
    }

    // 恢复游戏，切换为运行状态
    pub fn resume(&mut self) {
        if let GameState::Paused = self {
            *self = GameState::Running;
        }
    }

    // 设置游戏结束状态
    pub fn game_over(&mut self) {
        *self = GameState::GameOver;
    }

    // 检查是否游戏正在运行
    pub fn is_running(&self) -> bool {
        matches!(self, GameState::Running)
    }

    // 检查游戏是否暂停
    pub fn is_paused(&self) -> bool {
        matches!(self, GameState::Paused)
    }

    // 检查游戏是否结束
    pub fn is_game_over(&self) -> bool {
        matches!(self, GameState::GameOver)
    }
}
