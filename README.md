# cc-philo-talk

与哲学家对话、发展属于自己的哲学的桌面应用。

## 核心理念

知识不是概念的堆砌，而是在对话和碰撞中诞生的。

## 功能

### 哲学家档案
预置 9 位哲学家（苏格拉底、康德、黑格尔、克尔凯郭尔、马克思、尼采、海德格尔、福柯、德里达），每位含详细思想背景。可新增、编辑上下文——上下文越丰富，AI 扮演越准确。

### AI 对话
选择一位哲学家，AI 以其身份、语气和思维方式与你对话。可注入"我"的立场，让哲学家针对性回应。

### "我"
- **哲学宣言** — 写下你的核心立场和信念
- **思想上下文** — AI 对话时参考你的哲学背景
- **感悟** — 从对话中收藏或手动记录，逐步沉淀属于你的哲学

## 技术栈

| 层 | 技术 |
|---|---|
| 框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript |
| 样式 | Tailwind CSS v4 |
| 状态 | Pinia |
| AI | Claude API (Anthropic) |
| 构建 | Vite 6 + pnpm |

## 开发

```bash
pnpm install
pnpm tauri dev
```

## 配置 API Key

创建 `~/.cc-philo-talk/config.json`：

```json
{
  "apiKey": "sk-ant-your-key-here"
}
```
