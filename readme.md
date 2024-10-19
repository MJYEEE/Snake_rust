# Snake Game

## 目录
- [Snake Game](#snake-game)
  - [目录](#目录)
  - [简介](#简介)
  - [功能](#功能)
  - [依赖](#依赖)
  - [安装](#安装)
  - [运行](#运行)

## 简介
这是一个使用 Rust 和 Piston 库实现的经典贪吃蛇游戏。玩家可以通过键盘控制蛇的移动，蛇吃到食物后会增长，目标是尽可能让蛇变长而不碰到自己的身体或窗口边界。

## 功能

- 控制蛇的移动方向（上下左右）
- 吃掉食物以增长蛇的长度
- 碰撞检测（蛇头与食物的接触）
- 窗口显示游戏状态

## 依赖
- Rust 编译器 (`rustc`)
- Cargo 包管理器
- Piston 库及其相关依赖

## 安装
```bash
git clone https://github.com/MJYEEE/Snake_rust.git
cd Snake_rust
cargo build
```

## 运行
```bash
cargo run
```

