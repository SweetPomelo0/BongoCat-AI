# BongoCat AI

一个基于 [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat) 二次开发的 AI 桌宠项目。

目标不是把它做成一个冷冰冰的聊天工具，而是做成一只真正“住在桌面上”、会陪伴、会记忆、会主动关心用户的桌宠助手。

项目基于 **Tauri 2 + Vue 3 + TypeScript + Rust + Live2D**，并计划引入 **AI Agent 层 + OpenClaw 风格的 Markdown 记忆系统**，实现“越养越懂你”的使用体验。

---

## 项目愿景

BongoCat 原项目已经提供了成熟的桌宠壳能力：

- Live2D 动画渲染
- 键盘 / 鼠标 / 手柄交互捕获
- 透明置顶桌面窗口
- 跨平台轻量运行

本项目将在此基础上，逐步补上 AI 桌宠最关键的能力：

- **能聊天：** 支持自然语言对话与流式回复
- **会记忆：** 记住用户偏好、近期任务、聊天上下文
- **有情绪：** 根据时间、交互、工作状态切换情感状态
- **会感知：** 感知用户忙闲、活跃窗口、作息状态
- **会主动：** 在合适的时候提醒、关心、发起轻量互动
- **可透明编辑：** 记忆使用 Markdown 存储，用户可直接查看和修改

## 核心设计原则

### 1. 宠物感优先，不只是 AI 工具

它应该像一只长期陪伴用户的桌面宠物，而不是一个悬浮版聊天框。

### 2. Markdown 即真相

记忆以本地 Markdown 文件为准，用户可见、可查、可编辑，不做黑盒记忆。

### 3. 本地优先与隐私优先

敏感信息尽量只在本地处理；环境感知只保留必要摘要，不上传无关内容。

### 4. 先做 MVP，再逐步增强

先完成“能聊天 + 会记忆”的最小闭环，再迭代情绪、感知、主动行为等增强能力。

## 目标用户

- 长时间坐在电脑前的知识工作者
- 独立开发者 / 一人团队
- 喜欢桌宠文化、Agent 产品、可爱交互体验的极客用户

---

## 计划中的功能模块

### AI 对话

- 对话窗口
- 流式回复
- 多模型支持（DeepSeek / Claude / 自定义）
- 宠物人格设定

### 记忆系统

- `MEMORY.md` 长期记忆
- `daily/YYYY-MM-DD.md` 每日笔记
- `PERSONA.md` 人格设定
- 本地透明可编辑的 Markdown 记忆入口
- 保守的自动记忆提取 / 去重 / 写回
- 后续再扩展 SQLite FTS5 搜索索引

### 情感系统

- idle / happy / curious / sleepy / focused / worried 等状态
- 情感与 Live2D 动画联动
- 情感影响说话风格与主动行为频率

### 环境感知

- 键盘频率统计
- 活跃窗口标题检测
- 空闲时长 / 连续工作时长统计
- 深夜工作与作息状态识别

### 主动行为

- 长时间工作提醒
- 深夜关心提示
- 早安问候
- 与常用应用相关的轻量互动
- 临近 deadline 主动提醒

---

## 技术栈

### 当前基座

- **桌面框架：** Tauri 2
- **前端：** Vue 3 + TypeScript + Vite + Pinia
- **渲染：** Pixi.js + Live2D
- **后端：** Rust

### 计划新增

- **LLM 接入：** SiliconFlow / Claude / Custom Provider
- **本地记忆检索：** SQLite + FTS5
- **本地配置 / 凭据管理：** JSON + 系统 Keychain / 安全存储
- **环境感知：** 活跃窗口与输入事件统计

---

## 当前落地结构

```text
src/
├── pages/
│   ├── main/
│   ├── preference/
│   └── chat/
├── stores/
│   ├── ai.ts
│   └── chat.ts

src-tauri/
└── src/
    ├── ai.rs
    ├── memory.rs
    ├── core/
    └── utils/
```

## 记忆系统 MVP 结构

当前记忆系统已经按本地 Markdown 真相源落地：

```text
<appData>/memory/
├── PERSONA.md
├── MEMORY.md
└── daily/
    └── YYYY-MM-DD.md
```

说明：

- `PERSONA.md`：桌宠人格与说话风格
- `MEMORY.md`：长期稳定记忆
- `daily/YYYY-MM-DD.md`：当天上下文与临时事项
- 偏好设置页可直接打开记忆目录、Persona、MEMORY.md、今日日记
- API Key 在设置页以密码遮罩显示，不再明文暴露

---

## 开发计划

> 先完成基础可用版本，再逐步增加“更像宠物”的能力。

### Phase 0：项目接管与基线跑通

- [x] 从开源项目拉取代码并在本地运行
- [x] 梳理当前前端页面、窗口结构与状态管理方式
- [x] 梳理 Tauri 后端入口、命令注册与事件通信机制
- [x] 确认当前已改动文件与后续开发边界
- [x] 明确 README、目录规划、开发节奏

### Phase 1：对话入口与 UI 基础

- [x] 新增聊天窗口路由 / 页面
- [x] 支持从宠物入口打开聊天窗口
- [x] 实现基础消息列表与输入框
- [ ] 支持对话窗口跟随宠物位置显示
- [ ] 实现基础消息动画与打字机效果

### Phase 2：LLM 接入

- [x] 在 Rust 侧新增 AI 模块目录与基础结构
- [x] 封装 provider / model / baseUrl / API Key 配置
- [x] 接入 SiliconFlow 对话 API
- [x] 实现伪流式输出并通过 Tauri event 推送到前端
- [x] 前端完成流式消息渲染
- [x] 预留 Claude / 自定义 provider 扩展接口

### Phase 3：记忆系统 MVP

- [x] 初始化本地 AI 工作目录（`MEMORY.md` / `PERSONA.md` / daily notes）
- [x] 实现对话前的记忆上下文加载
- [x] 实现对话后的记忆提取与写入
- [x] 区分长期记忆与每日记录
- [x] 增加基础去重逻辑
- [x] 增加记忆文件查看入口
- [x] 修复 Windows 下记忆目录/文件打开流程
- [ ] 增加更稳健的结构化提取策略
- [ ] 增加历史 daily notes 检索策略

### Phase 4：设置面板扩展

- [x] 在设置窗口新增 AI Tab
- [x] 支持 provider / model / API Key 配置
- [x] 支持记忆开关与本地文件打开入口
- [x] API Key 改为密码遮罩显示
- [ ] 支持人格设定内嵌编辑入口
- [ ] 支持环境感知开关
- [ ] 支持主动行为频率设置

### Phase 5：情感系统

- [ ] 建立本地情感状态机
- [ ] 接入时间、交互、工作时长等触发条件
- [ ] 将情感状态映射到 Live2D 动画表现
- [ ] 让情感状态影响对话语气
- [ ] 记录情感变化到每日笔记

### Phase 6：环境感知

- [ ] 统计键盘频率与空闲时长
- [ ] 检测活跃窗口标题
- [ ] 判断连续工作时长
- [ ] 输出给 LLM 的环境摘要上下文
- [ ] 确保感知逻辑默认本地处理、可关闭

### Phase 7：主动行为引擎

- [ ] 建立定时器与行为触发规则
- [ ] 实现休息提醒与深夜提醒
- [ ] 实现早安问候与轻量主动聊天
- [ ] 增加频率限制与免打扰控制
- [ ] 支持用户关闭主动行为

### Phase 8：记忆搜索与管理

- [ ] 建立 SQLite FTS5 索引
- [ ] 支持历史 daily notes 搜索
- [ ] 将搜索结果注入对话上下文
- [ ] 增加记忆管理窗口 / 文件夹打开入口
- [ ] 预留后续向量搜索扩展空间

### Phase 9：打磨与发布

- [ ] 完成基础功能联调
- [ ] 做一轮 Windows 侧完整测试
- [ ] 验证性能、内存占用与窗口体验
- [ ] 清理配置项与异常场景
- [ ] 准备可分发版本

---

## MVP 定义

第一阶段优先做出最小闭环：

- 可以打开聊天窗口
- 可以和宠物聊天
- 可以把对话写入本地记忆
- 用户可以直接查看和编辑记忆文件

也就是先把“**能聊 + 会记**”做出来，再继续增强“**有情绪 + 会主动 + 更懂你**”。

## 当前状态

当前项目已经完成：

- [x] 基于开源 BongoCat 项目完成拉取
- [x] 本地运行成功
- [x] 明确 AI 桌宠方向与总体技术方案
- [x] 输出首版 README 与开发计划
- [x] 梳理多窗口 / 路由 / 状态管理 / Tauri 命令结构
- [x] 完成聊天窗口 MVP（可打开窗口、发送消息、收到模型回复）
- [x] 完成多轮上下文与伪流式回复
- [x] 修复主窗口在多显示器/偏移坐标场景下显示不全的问题
- [x] 完成 Memory MVP：本地 Markdown 记忆、保守写回、设置页透明入口
- [x] 完成 AI 设置页：provider / model / baseUrl / API Key / memoryEnabled
- [x] 修复 Windows 下记忆目录/文件打开失败问题
- [x] API Key 改为密码遮罩显示
- [x] 通过 `cargo check` 与 `tsc --noEmit` 基础检查

下一步建议优先进入：

1. 把当前规则式记忆提取升级为更稳健的结构化提取
2. 增加历史 daily notes 检索策略，为后续召回打基础
3. 再推进情感系统、感知系统与主动行为能力

---

## 参考项目

- [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat)
- [openclaw/openclaw](https://github.com/openclaw/openclaw)
- [ChaozhongLiu/DyberPet](https://github.com/ChaozhongLiu/DyberPet)
- [fxy1699/PirPaw](https://github.com/fxy1699/PirPaw)
- [jihe520/Agentic-Desktop-Pet](https://github.com/jihe520/Agentic-Desktop-Pet)

## License

本项目当前基于上游 MIT 项目二次开发，后续许可证策略建议与上游保持兼容。
