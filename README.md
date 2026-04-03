# BongoCat AI

<p align="center">
  <b>一只真正住在桌面上的 Live2D AI 桌宠。</b><br>
  会跟着你的键盘、鼠标、手柄一起动，也能聊天、记忆，并慢慢变成更懂你的陪伴型助手。
</p>

<p align="center">
  <b>A Live2D AI desktop pet that actually lives on your desktop.</b><br>
  It reacts to your keyboard, mouse, and controller, can chat with you, remember things, and slowly grow into a companion that feels personal.
</p>

<p align="center">
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases"><img alt="Release" src="https://img.shields.io/github/v/release/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <a href="./LICENSE"><img alt="License" src="https://img.shields.io/github/license/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square">
  <img alt="Stack" src="https://img.shields.io/badge/Tauri%202-Vue%203-Rust-8b5cf6?style=flat-square">
</p>

<p align="center">
  <a href="#-快速开始">快速开始</a> ·
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases">下载发布版</a> ·
  <a href="#-核心亮点">核心亮点</a> ·
  <a href="#-自定义你的桌宠">自定义模型</a> ·
  <a href="./.github/CONTRIBUTING.md">参与贡献</a>
</p>

> 基于 [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat) 二次开发。它不想做一个冷冰冰的 AI 工具，而是想成为一只真正“住在桌面上”的猫：会动、会陪你、会记住你，也会在合适的时候和你说话。

---

## 让 AI 回到桌面，而不是只待在对话框里

大多数 AI 产品都停留在输入框里。

**BongoCat AI** 想做的是另一件事：
让 AI 变成一个可以长期陪伴你的桌面角色。

它既保留了原版 BongoCat 那种轻巧、可爱、即时反馈的桌宠体验，又继续往前走了一步：

- 它会随着你的 **键盘 / 鼠标 / 手柄输入实时互动**
- 它可以打开独立聊天窗口，变成你的 **陪伴型桌面助手**
- 它会把重要信息沉淀到 **本地 Markdown 记忆** 中，而不是只做一次性问答
- 它支持 **自定义模型、窗口行为、桌面交互方式**，更像“你的桌宠”，而不是一份模板产品

如果你想找的是一个既可爱、又能长期使用、还能持续成长的 AI 桌宠，这就是它想成为的样子。

---

## 核心亮点

### 像宠物，而不是像插件

- Live2D 桌宠渲染，透明背景、桌面置顶、轻量运行
- 根据键盘 / 鼠标 / 手柄输入做出实时联动，不只是静态摆件
- 支持窗口大小、透明度、圆角、位置、穿透等细节调节
- 支持托盘控制、显示/隐藏、快捷操作，更适合长期挂在桌面上

### 能聊，也能长期陪伴

- 独立聊天窗口，不破坏桌宠主界面的轻盈感
- 支持 provider / model / baseUrl / API Key 配置
- 回复风格更偏“陪伴感”，而不只是工具式答案输出
- 已具备基础流式显示与多轮上下文能力

### 记忆是可见的，不是黑盒的

- 使用本地 Markdown 作为记忆真相源
- `PERSONA.md`：桌宠人格设定
- `MEMORY.md`：长期稳定记忆
- `daily/YYYY-MM-DD.md`：当天上下文与临时事项
- 记忆文件可直接查看、编辑、审查，不做完全不可见的黑盒记忆

### 不只是默认猫猫，你可以把它养成自己的样子

- 支持导入自定义 Live2D 模型
- 预置 `standard` / `keyboard` / `gamepad` 模式
- 可接入社区模型生态与模型转换工具

### 真正适合日常使用的桌面应用底座

- 基于 **Tauri 2 + Vue 3 + TypeScript + Rust + Live2D**
- 原生桌面窗口、托盘、快捷键、启动项能力
- 支持 Windows、macOS、Linux(X11)
- 支持 GitHub Releases 分发

---

## 项目截图

| macOS                                                                                                     | Windows                                                                                                   | Linux                                                                                                       |
| --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-mac.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-win.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-linux.jpg) |

---

## 快速开始

### 1. 下载发布版

最直接的方式是前往 Releases：

- [下载最新版本](https://github.com/SweetPomelo0/BongoCat-AI/releases)

如果你想快速找到不同平台对应的安装包名称，可以继续看：

- [下载指南](./.github/DOWNLOAD_GUIDE.md)

### 2. 启动后，先感受它最有意思的部分

你第一次打开它时，就可以立即体验到：

- 桌宠主窗口与输入联动效果
- 托盘菜单控制
- 窗口位置、透明度、大小、穿透等行为设置
- 聊天窗口与 AI 设置页

### 3. 如果你想让它更像“你的桌宠”

接下来可以继续配置：

- Provider / Model / Base URL / API Key
- Memory 开关
- 记忆目录、`PERSONA.md`、`MEMORY.md`、今日日记入口
- 自定义模型导入

---

## 为什么它不只是普通的 BongoCat

原版 BongoCat 已经是一只非常成熟、非常讨喜的桌宠了：

- Live2D 动画渲染
- 键盘 / 鼠标 / 手柄事件捕获
- 透明置顶桌面窗口
- 多平台轻量运行

而这个分支继续补上的，是“长期陪伴”这一层：

- **不只是跟着你敲键盘，而是可以真正聊天**
- **不只是临时回应，而是能够逐步形成长期记忆**
- **不只是黑盒 Agent，而是尽量把记忆变成用户看得见、改得动、审查得到的 Markdown 文件**

它想做的，不是一层 AI 皮肤，也不是一个挂在桌面边缘的聊天框。

它想做的是：
**让一只桌宠，真的慢慢活进你的桌面日常里。**

---

## AI 与 Markdown 记忆

当前项目已经落地了本地 Markdown 记忆 MVP，结构如下：

```text
<appData>/memory/
├── PERSONA.md
├── MEMORY.md
└── daily/
    └── YYYY-MM-DD.md
```

说明：

- `PERSONA.md`：桌宠人格、语气与陪伴风格
- `MEMORY.md`：长期偏好、稳定信息与重要事实
- `daily/YYYY-MM-DD.md`：当天上下文、聊天记录摘要、临时事项

设计原则：

- **Markdown 即真相**：用户可见、可查、可直接编辑
- **本地优先**：核心桌宠体验与记忆文件落地尽量本地化
- **透明优先**：尽量避免“记住了什么你完全不知道”的黑盒体验

---

## 从源码运行

### 环境准备

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

## 当前路线图

> 优先做出“能聊 + 会记 + 有陪伴感”的 AI 桌宠最小闭环，再逐步增强情绪、感知与主动行为。

- [x] 桌宠渲染与多输入联动
- [x] 聊天窗口与基础 AI 配置
- [x] 本地 Markdown 记忆 MVP
- [x] 记忆文件透明入口与查看能力
- [ ] 更稳健的结构化记忆提取
- [ ] 历史记忆检索与召回
- [ ] 情绪状态与主动行为
- [ ] 更完整的环境感知

---

## 适合谁

这个项目特别适合：

- 长时间坐在电脑前的知识工作者
- 独立开发者 / 一人团队
- 喜欢桌宠文化、Agent 产品、可爱交互体验的用户
- 想把“桌宠 + AI + 本地记忆”结合起来的开发者

---

## 相关项目

- [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat)

---

## 社区与贡献

如果你也喜欢这种“桌宠 + AI + 本地记忆”的方向，欢迎一起把它打磨得更完整：

- 阅读 [贡献指南](./.github/CONTRIBUTING.md)
- 提交 Issue / PR
- 分享模型、交互想法、使用体验和改进建议

这个项目很适合持续迭代：
你给它一个想法，它就可能慢慢长成下一只更聪明、也更有灵魂的桌面宠物。

---

## License

本项目基于上游 MIT 项目二次开发，当前许可证与上游保持兼容，详见 [LICENSE](./LICENSE)。
