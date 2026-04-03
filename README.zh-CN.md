# BongoCat AI

<p align="right">
  <a href="./README.md">English</a> | 简体中文
</p>

<p align="center">
  <b>一个本地优先、带透明 Markdown 记忆的 AI 桌面陪伴体。</b><br>
  它会跟着你的键盘、鼠标、手柄互动，支持聊天，并把持续记忆保存在用户可编辑的本地文件里。
</p>

<p align="center">
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases"><img alt="Release" src="https://img.shields.io/github/v/release/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <a href="./LICENSE"><img alt="License" src="https://img.shields.io/github/license/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square">
  <img alt="Stack" src="https://img.shields.io/badge/Tauri%202-Vue%203-Rust-8b5cf6?style=flat-square">
</p>

<p align="center">
  <a href="#快速开始">快速开始</a> ·
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases">下载</a> ·
  <a href="#核心亮点">亮点</a> ·
  <a href="#本地记忆">本地记忆</a> ·
  <a href="./.github/CONTRIBUTING.md">参与贡献</a>
</p>

> 本项目基于 [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat) 二次开发，目标是在原版桌宠基础上，继续加入聊天、记忆和更长期的陪伴体验。

---

## 为什么是 BongoCat AI

大多数 AI 产品仍然停留在输入框里。

**BongoCat AI** 想走另一条路：做一个本地优先、带持续记忆层的 AI 桌面陪伴体，让 AI 更贴近桌面，而不是只存在于网页对话框里。

它保留了原版 BongoCat 轻巧、可爱、即时反馈的体验，并继续扩展：

- 键盘、鼠标、手柄的实时互动
- 独立的陪伴式聊天窗口
- 可编辑的 Markdown 记忆与持续上下文
- 托盘、启动项、置顶、窗口行为等桌面原生能力

---

## 核心亮点

### 更像桌宠，而不是插件

- Live2D 桌宠渲染，支持透明背景
- 随键盘、鼠标、手柄输入实时联动
- 支持窗口大小、透明度、圆角、位置、穿透等设置
- 支持托盘控制，更适合长期挂在桌面上

### 加入 AI，但不变成普通聊天框

- 独立的陪伴式聊天窗口
- 支持 provider、model、base URL、API key 配置
- 支持流式显示和多轮上下文
- 整体更强调 companion UX，而不是纯工具式问答

### 记忆是可见的，不是黑盒的

- 记忆存放在用户可编辑的本地 Markdown 文件中
- `PERSONA.md`：人格和说话风格
- `MEMORY.md`：长期记忆与稳定偏好
- `daily/YYYY-MM-DD.md`：近期上下文与短期记录
- 方向上接近 OpenClaw 这类“透明、本地优先记忆”的思路，但当前实现仍以本仓库自己的轻量记忆层为主

### 真正面向桌面应用

- Tauri 2 + Vue 3 + TypeScript + Rust + Live2D
- 原生托盘、启动项、快捷键、窗口集成
- 支持 Windows、macOS、Linux(X11)
- 通过 GitHub Releases 分发

---

## 项目截图

| macOS                                                                                                     | Windows                                                                                                   | Linux                                                                                                       |
| --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-mac.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-win.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-linux.jpg) |

---

## 快速开始

1. 下载最新版本：
   - [GitHub Releases](https://github.com/SweetPomelo0/BongoCat-AI/releases)
2. 如果你需要按平台查看安装包名称：
   - [下载指南](./.github/DOWNLOAD_GUIDE.md)
3. 启动应用后，可以直接体验：
   - 桌宠窗口
   - 输入联动
   - 托盘控制
   - 聊天窗口和 AI 设置

如果你想更个性化一点，可以继续在设置中配置模型、AI provider 和 memory。

---

## 和原版 BongoCat 的区别

原版已经是一只很成熟的桌宠。

这个分支在它之上继续补了一层：

- 不只是动画联动，还有陪伴式聊天
- 不只是一次性回复，还有持续上下文和长期记忆
- 不只是黑盒状态，而是把记忆落到用户可见、可编辑的本地文件里

目标很简单：
把桌宠进一步做成一个本地优先的 AI 桌面陪伴层，而不只是一个主题化聊天 UI。

---

## 本地记忆

BongoCat AI 当前使用一套本地优先的 Markdown memory workspace，用来保存持续上下文，并保持记忆透明、可检查、可编辑。

当前记忆文件结构如下：

```text
<appData>/memory/
├── PERSONA.md
├── MEMORY.md
└── daily/
    └── YYYY-MM-DD.md
```

### 每个文件的作用

- `PERSONA.md`：人格、语气、角色设定
- `MEMORY.md`：长期事实和稳定偏好
- `daily/YYYY-MM-DD.md`：当天上下文和临时记录

### 设计原则

- **Markdown 就是真相源**
- **核心记忆尽量本地化、可见化**
- **用户可以检查和编辑系统记住的内容**

---

## 从源码运行

### 环境要求

- [Rust](https://v2.tauri.app/start/prerequisites/)
- [Node.js](https://nodejs.org/en/)
- [pnpm](https://pnpm.io/)

### 安装依赖

```bash
pnpm install
```

### 启动开发环境

```bash
pnpm tauri dev
```

### 构建应用

```bash
pnpm tauri build
```

---

## Roadmap

- [x] Live2D 桌宠与多输入联动
- [x] 聊天窗口和基础 AI 设置
- [x] 本地 Markdown 记忆 MVP
- [x] 设置页中的记忆文件入口
- [ ] 更稳健的结构化记忆提取
- [ ] 历史 daily notes 检索能力
- [ ] 情绪与主动行为系统
- [ ] 更丰富的环境感知

---

## 适合谁

- 想要真正能互动的桌宠的开发者和用户
- 喜欢可爱 UI，但又希望有实际 AI 能力的人
- 更偏好本地记忆，而不是完全黑盒状态的人
- 正在探索桌宠、Agent、陪伴式交互结合方向的构建者

---

## 相关项目

- [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat)

---

## 贡献

如果你也喜欢“桌宠 + AI + 本地记忆”这个方向，欢迎一起完善它。

- 阅读 [贡献指南](./.github/CONTRIBUTING.md)
- 提交 Issue 或 PR
- 分享模型、交互和陪伴体验的想法

---

## License

本项目基于上游 MIT 协议项目，当前许可证与上游保持兼容，详见 [LICENSE](./LICENSE)。
