# 2025 Rust 学习

本人也是 Rust 新手，该项目目标为在闲暇之余将自己的 JS web 端 fileSys 项目使用 Rust WASM 重写，并使用 Leptos 作为前端框架。
挑战在 2025 年完成重写计划，边学边写。

选择 Rust 并非跟风，作为一个常年仅使用 JS 写代码的开发者，是非常需要再掌握一个系统级的编程语言，能够暴露细节而不是隐藏细节， 而 Rust 是正是一个非常好的选择。

## 相关学习资料
------------
### Books
- [Rust 官方教程](https://www.rust-lang.org/learn)
- [Rust 中文教程](https://course.rs/about-book.html)

### Videos
- [喜欢历史的程序君](https://www.youtube.com/watch?v=ZVIlcsYaDZY&list=PL2XM89iiOzkud-BMooV19IWyBtfMVVNJj)

### AI 辅助
- ChatGPT
- Gemini
- Cursor

## Daily
--------

### Day 01
- 项目搭建
- App 组件
- Breadcrumb 组件

### Day 02
- 组件引入 CSS
- 完善 Breadcrumb 组件

### Day 04
- 引入 tailwindcss
- 发送 http 请求，获取文件列表

### Day 05
- 封装/完善 request 请求工具

### Day 07
- 自定义 hook
- file_list 组件

### Day 09
- list 组件 & on_click 事件

### Day 10
- 完善 file_list 组件， 点击进入子目录
- 使用 context api 传递 path

### Day 11
- pushState 修改 url params
- use leptos-use

### Day 13
- learn leptos signal Getting and Setting 
- 完善组件样式

### Day 15
- add footer
- TODO: 子组件如何调用父组件的 callback 并且携带自定义参数，并且在一个 move || {} 中使用 ??

### Day 18
- 使用 leptos 中的 Callback 优化代码