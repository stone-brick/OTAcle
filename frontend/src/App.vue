<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const logs = ref<string[]>([]);
const actionInput = ref("");
const textInput = ref("");
const windowTitle = ref("");
const foundHwnd = ref<number | null>(null);
const status = ref<"ready" | "sending">("ready");

// 窗口查询模式
const searchMode = ref<"exe" | "class" | "pid" | "id" | "title">("title");

function buildWindowSpec(): string {
  const title = windowTitle.value.trim();
  if (!title) return "";
  switch (searchMode.value) {
    case "exe": return `ahk_exe ${title}`;
    case "class": return `ahk_class ${title}`;
    case "pid": return `ahk_pid ${title}`;
    case "id": return `ahk_id ${title}`;
    case "title": return title;
    default: return title;
  }
}

function addLog(msg: string) {
  const time = new Date().toLocaleTimeString();
  logs.value.push(`[${time}] ${msg}`);
}

async function sendAction(action: string, params: Record<string, unknown> = {}) {
  status.value = "sending";
  try {
    await invoke(action, params);
    addLog(`${action} 成功`);
  } catch (e) {
    addLog(`${action} 失败: ${e}`);
  }
  status.value = "ready";
}

async function handleSubmit() {
  if (!actionInput.value.trim()) return;
  addLog(`发送动作: ${actionInput.value}`);
  const spec = windowTitle.value.trim() || null;
  await sendAction("send_text", { text: actionInput.value, window: spec });
  actionInput.value = "";
}

async function sendKey(key: string) {
  addLog(`发送按键: ${key}`);
  const spec = windowTitle.value.trim() || null;
  await sendAction("send_key", { key, direction: "click", window: spec });
}

async function sendText() {
  if (!textInput.value.trim()) return;
  addLog(`发送文本: ${textInput.value}`);
  const spec = windowTitle.value.trim() || null;
  await sendAction("send_text", { text: textInput.value, window: spec });
  textInput.value = "";
}

async function findAndActivateWindow(): Promise<boolean> {
  const spec = buildWindowSpec();
  if (!spec) return false;

  // 1. 先查找窗口
  addLog(`查找窗口: ${spec}`);
  try {
    const hwnd = await invoke<number | null>("find_window_by_title", { title: spec });
    if (hwnd) {
      foundHwnd.value = hwnd;
      addLog(`找到窗口 HWND: ${hwnd} (0x${hwnd.toString(16)})`);
    } else {
      addLog(`未找到窗口`);
      return false;
    }
  } catch (e) {
    addLog(`查找失败: ${e}`);
    return false;
  }

  // 2. 激活窗口
  addLog(`激活窗口: ${spec}`);
  try {
    const success = await invoke<boolean>("activate_window_by_title", { title: spec });
    if (success) {
      addLog(`窗口激活成功`);
    } else {
      addLog(`窗口激活失败`);
    }
    return success;
  } catch (e) {
    addLog(`激活失败: ${e}`);
    return false;
  }
}
</script>

<template>
  <main class="container">
    <h1>OTAcle - Act Module</h1>

    <!-- 状态显示 -->
    <div class="status-bar">
      <span class="status-indicator" :class="status"></span>
      <span>{{ status === "ready" ? "就绪" : "发送中..." }}</span>
    </div>

    <!-- 窗口控制 -->
    <div class="section">
      <h3>窗口控制</h3>
      <div class="window-control">
        <div class="window-row">
          <select v-model="searchMode" class="mode-select">
            <option value="title">窗口标题</option>
            <option value="exe">进程名</option>
            <option value="class">窗口类</option>
            <option value="pid">进程ID</option>
            <option value="id">HWND</option>
          </select>
          <input
            v-model="windowTitle"
            :placeholder="searchMode === 'exe' ? '进程名 (如 notepad.exe)' :
                          searchMode === 'class' ? '窗口类 (如 Notepad)' :
                          searchMode === 'pid' ? '进程ID' :
                          searchMode === 'id' ? 'HWND值' : '窗口标题'"
            :disabled="status === 'sending'"
            class="window-input"
          />
        </div>
        <div class="btn-row">
          <button @click="findAndActivateWindow" :disabled="status === 'sending'" class="activate-btn">
            激活窗口
          </button>
        </div>
        <div v-if="foundHwnd !== null" class="hwnd-display">
          HWND: {{ foundHwnd }} (0x{{ foundHwnd.toString(16) }})
        </div>
      </div>
    </div>

    <!-- 命令输入 -->
    <div class="section">
      <h3>动作输入</h3>
      <form @submit.prevent="handleSubmit" class="action-form">
        <input
          v-model="actionInput"
          placeholder="输入动作指令..."
          :disabled="status === 'sending'"
        />
        <button type="submit" :disabled="status === 'sending'">发送</button>
      </form>
    </div>

    <!-- 快捷动作 -->
    <div class="section">
      <h3>快捷动作</h3>
      <div class="action-grid">
        <div class="action-group">
          <span class="group-label">方向键</span>
          <div class="btn-row">
            <button @click="sendKey('up')" :disabled="status === 'sending'">↑</button>
          </div>
          <div class="btn-row">
            <button @click="sendKey('left')" :disabled="status === 'sending'">←</button>
            <button @click="sendKey('down')" :disabled="status === 'sending'">↓</button>
            <button @click="sendKey('right')" :disabled="status === 'sending'">→</button>
          </div>
        </div>

        <div class="action-group">
          <span class="group-label">动作键</span>
          <div class="btn-row">
            <button @click="sendKey('space')" :disabled="status === 'sending'">Space</button>
            <button @click="sendKey('return')" :disabled="status === 'sending'">Enter</button>
          </div>
          <div class="btn-row">
            <button @click="sendKey('escape')" :disabled="status === 'sending'">Esc</button>
          </div>
        </div>

        <div class="action-group">
          <span class="group-label">文本</span>
          <div class="btn-row">
            <input v-model="textInput" placeholder="输入文本..." :disabled="status === 'sending'" />
            <button @click="sendText" :disabled="status === 'sending'">发送</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 日志 -->
    <div class="section log-section">
      <h3>日志</h3>
      <div class="log-box">
        <p v-for="(log, i) in logs" :key="i" class="log-line">{{ log }}</p>
        <p v-if="logs.length === 0" class="log-empty">暂无日志</p>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  margin: 0;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

h1 {
  text-align: center;
  margin: 0;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px;
  background: var(--status-bg);
  border-radius: 6px;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #ccc;
}

.status-indicator.ready {
  background: #4caf50;
}

.status-indicator.sending {
  background: #ff9800;
}

.section {
  padding: 15px;
  background: var(--card-bg);
  border-radius: 8px;
}

.section h3 {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.action-form {
  display: flex;
  gap: 8px;
}

.action-form input {
  flex: 1;
}

.action-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.action-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.group-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.btn-row {
  display: flex;
  gap: 4px;
}

.btn-row button {
  padding: 6px 12px;
  font-size: 13px;
}

.btn-row input {
  padding: 6px 8px;
  font-size: 13px;
  width: 120px;
}

.log-section {
  flex: 1;
}

.log-box {
  height: 150px;
  overflow-y: auto;
  background: var(--log-bg);
  border-radius: 4px;
  padding: 8px;
  font-family: monospace;
  font-size: 12px;
}

.log-line {
  margin: 2px 0;
}

.log-empty {
  color: var(--text-secondary);
  font-style: italic;
}

.window-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.window-input {
  width: 100%;
}

.hwnd-display {
  font-family: monospace;
  font-size: 13px;
  color: var(--log-text);
  background: var(--log-bg);
  padding: 6px 10px;
  border-radius: 4px;
}

.window-row {
  display: flex;
  gap: 8px;
}

.mode-select {
  width: 140px;
  padding: 8px 12px;
  font-size: 14px;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: var(--card-bg);
  color: var(--text-primary);
}

.activate-btn {
  background: #396cd8;
  color: white;
  border-color: #396cd8;
}

.activate-btn:hover:not(:disabled) {
  background: #2a52b0;
}

button {
  cursor: pointer;
}

button:disabled,
input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.5;

  --bg: #f6f6f6;
  --card-bg: #ffffff;
  --status-bg: #f0f0f0;
  --log-bg: #f3f3f3;
  --log-text: #d4d4d4;
  --text-primary: #0f0f0f;
  --text-secondary: #666;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #2f2f2f;
    --card-bg: #3a3a3a;
    --status-bg: #2a2a2a;
    --log-bg: #0d0d0d;
    --log-text: #d4d4d4;
    --text-primary: #f6f6f6;
    --text-secondary: #999;
  }
}

body {
  margin: 0;
  background: var(--bg);
  color: var(--text-primary);
}

input,
button {
  border-radius: 6px;
  border: 1px solid #ccc;
  padding: 8px 12px;
  font-size: 14px;
  font-family: inherit;
  background: var(--card-bg);
  color: var(--text-primary);
  transition: border-color 0.2s;
}

button:hover:not(:disabled) {
  border-color: #396cd8;
}

button:active:not(:disabled) {
  background: #e8e8e8;
}

input:focus,
button:focus {
  outline: none;
  border-color: #396cd8;
}

* {
  box-sizing: border-box;
}
</style>
