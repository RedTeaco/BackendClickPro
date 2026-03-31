<template>
  <div class="automation-dashboard">
    <div class="dashboard-card">
      <!-- 左侧操作区 -->
      <aside class="operation-panel">
        <!-- 软件标题 -->
        <div class="software-header">
          <div class="logo-icon">⚡</div>
          <h1 class="software-title">AutoFlow Studio</h1>
        </div>

        <!-- 主要操作按钮组 -->
        <div class="button-group">
          <button class="action-btn" @click="fetchNonMinimizedWindows" :disabled="isRunning">
            <span class="btn-text">捕获窗口</span>
            <kbd class="shortcut-key">Alt+C</kbd>
          </button>

          <button class="action-btn primary" @click="toggleRun" :class="{ running: isRunning }">
            <span class="btn-text">{{ isRunning ? '停止' : '启动' }}</span>
            <kbd class="shortcut-key">Ctrl+Shift+S</kbd>
          </button>

          <button class="action-btn" @click="openSettings" :disabled="isRunning">
            <span class="btn-text">设置</span>
            <span class="icon-settings">⚙️</span>
          </button>
        </div>

        <!-- 模式切换按钮组 -->
        <div class="mode-switch-group">
          <button
              class="mode-btn"
              :class="{ active: currentMode === 'sync' }"
              @click="setMode('sync')"
              :disabled="isRunning"
          >
            🔄 同步模式
          </button>
          <button
              class="mode-btn"
              :class="{ active: currentMode === 'sequence' }"
              @click="setMode('sequence')"
              :disabled="isRunning"
          >
            📋 序列模式
          </button>
        </div>

        <!-- 运行状态 & 实时日志 -->
        <div class="status-area">
          <div class="status-indicator">
            <span class="status-dot" :class="{ active: isRunning }"></span>
            <span class="status-text">{{ isRunning ? '运行中' : '就绪' }}</span>
          </div>
          <div class="log-container">
            <div v-for="(log, idx) in logs" :key="idx" class="log-line">{{ log }}</div>
            <div v-if="logs.length === 0" class="log-placeholder">等待操作...</div>
          </div>
        </div>
      </aside>

      <!-- 右侧操作列表区 -->
      <main class="actions-panel">
        <div class="panel-header">
          <h3>执行步骤列表</h3>
          <span class="step-count">{{ actionItems.length }} 个步骤</span>
        </div>

        <!-- 操作列表 -->
        <div class="action-list-container">
          <ul class="action-list">
            <li
                v-for="(item, index) in actionItems"
                :key="item.id"
                class="action-item"
                :class="{ selected: selectedIndex === index }"
                @click="selectItem(index)"
            >
              <span class="item-index">{{ index + 1 }}</span>
              <span class="item-content">{{ item.name }}</span>
              <button
                  class="item-delete"
                  @click.stop="deleteSingleItem(index)"
                  :disabled="isRunning"
                  title="删除此项"
              >✕</button>
            </li>
          </ul>
          <div v-if="actionItems.length === 0" class="empty-list">
            <span>📭 暂无步骤，点击上方“捕获窗口”或使用添加栏创建</span>
          </div>
        </div>

        <!-- 列表控制栏 (添加 / 删除 / 上下移动) -->
        <div class="control-bar">
          <div class="add-section">
            <input
                type="text"
                v-model="newActionName"
                placeholder="输入新步骤名称..."
                :disabled="isRunning"
                @keyup.enter="addAction"
            />
            <button class="ctrl-btn add-btn" @click="addAction" :disabled="isRunning">➕ 添加</button>
          </div>
          <div class="move-delete-group">
            <button class="ctrl-btn" @click="moveUp" :disabled="isRunning || selectedIndex === null || selectedIndex === 0">⬆ 上移</button>
            <button class="ctrl-btn" @click="moveDown" :disabled="isRunning || selectedIndex === null || selectedIndex === actionItems.length - 1">⬇ 下移</button>
            <button class="ctrl-btn delete-btn" @click="deleteSelected" :disabled="isRunning || selectedIndex === null">🗑 删除</button>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onUnmounted } from 'vue'
import {invoke} from "@tauri-apps/api/core";

// ---------- 状态定义 ----------
const isRunning = ref(false)           // 启动/停止状态
const currentMode = ref('sync')       // 'sync' 同步模式 / 'sequence' 序列模式
const actionItems = ref([])           // 操作列表 { id, name }
const selectedIndex = ref(null)       // 当前选中项的索引
const newActionName = ref('')          // 添加输入框的值

// 日志区域 (用于显示执行动作)
const logs = ref([])

// 执行器控制 (用于序列模式的异步中断)
let sequenceTimer = null
let isSequenceActive = false

// ---------- 辅助函数 ----------
// 添加一条日志 (自动保留最近20条)
const addLog = (message) => {
  logs.value.unshift(message)
  if (logs.value.length > 20) logs.value.pop()
}

// 模拟“执行一个操作” (这里仅展示日志)
const executeAction = async (item, index) => {
  addLog(`✅ 执行: ${item.name} (步骤 ${index + 1})`)
  // 模拟耗时操作，增加真实感 (用于序列模式延时)
  return new Promise(resolve => setTimeout(resolve, 600))
}

// ---------- 执行核心 (根据模式) ----------
// 停止当前所有执行
const stopExecution = () => {
  if (sequenceTimer) {
    clearTimeout(sequenceTimer)
    sequenceTimer = null
  }
  isSequenceActive = false
  addLog('⏹️ 执行已停止')
}

// 同步模式: 并发/顺序执行所有动作 (这里按顺序快速执行，因为是同步概念，展示同时触发效果)
const runSyncMode = async () => {
  addLog(`🔄 同步模式启动，共 ${actionItems.value.length} 个动作`)
  for (let i = 0; i < actionItems.value.length; i++) {
    // 如果在执行过程中被外部停止则中断
    if (!isRunning.value) break
    await executeAction(actionItems.value[i], i)
  }
  if (isRunning.value) {
    addLog('🏁 同步模式执行完毕')
    stopRunningState()   // 执行完毕自动转为停止状态
  } else {
    addLog('⚠️ 同步模式被中途停止')
  }
}

// 序列模式: 递归延时执行，可随时停止
const runSequenceMode = () => {
  let currentIndex = 0
  addLog(`📋 序列模式启动，逐项执行 (间隔0.6s)`)
  isSequenceActive = true

  const step = () => {
    if (!isRunning.value || !isSequenceActive || currentIndex >= actionItems.value.length) {
      if (currentIndex >= actionItems.value.length && isRunning.value) {
        addLog('🏁 序列模式执行完毕')
        stopRunningState()   // 完成后自动停止
      } else if (!isRunning.value) {
        addLog('⏸️ 序列模式已被停止')
      }
      return
    }
    const item = actionItems.value[currentIndex]
    addLog(`▶️ 执行: ${item.name} (${currentIndex+1}/${actionItems.value.length})`)
    // 模拟执行耗时，结束后继续下一步
    sequenceTimer = setTimeout(() => {
      currentIndex++
      step()
    }, 600)
  }
  step()
}

// 启动总控
const startRunning = () => {
  if (actionItems.value.length === 0) {
    addLog('⚠️ 无法启动: 操作列表为空，请先添加步骤')
    isRunning.value = false
    return
  }
  addLog(`🚀 启动执行，模式: ${currentMode.value === 'sync' ? '同步模式' : '序列模式'}`)
  if (currentMode.value === 'sync') {
    runSyncMode()
  } else {
    runSequenceMode()
  }
}

// 停止执行并重置运行状态
const stopRunningState = () => {
  if (isRunning.value) {
    isRunning.value = false
    stopExecution()
  }
}

// 切换启动/停止按钮
const toggleRun = () => {
  if (isRunning.value) {
    // 停止
    stopRunningState()
    addLog('⏸️ 用户手动停止执行')
  } else {
    // 启动
    isRunning.value = true
    startRunning()
  }
}

// 设置模式 (仅在停止状态下可修改)
const setMode = (mode) => {
  if (!isRunning.value) {
    currentMode.value = mode
    addLog(`⚙️ 切换至${mode === 'sync' ? '同步模式' : '序列模式'}`)
  }
}

// ---------- 右侧列表操作 (运行时禁用) ----------
// 添加新动作
const addAction = () => {
  if (isRunning.value) return
  const name = newActionName.value.trim()
  if (!name) {
    addLog('⚠️ 步骤名称不能为空')
    return
  }
  const newItem = {
    id: Date.now() + Math.random(),
    name: name
  }
  actionItems.value.push(newItem)
  newActionName.value = ''
  addLog(`📌 添加步骤: ${name}`)
  // 如果之前没有选中项，可以默认选中新添加项（可选）
  if (actionItems.value.length === 1) selectedIndex.value = 0
}

// 删除单个项 (通过列表项上的删除按钮)
const deleteSingleItem = (index) => {
  if (isRunning.value) return
  if (index >= 0 && index < actionItems.value.length) {
    const removed = actionItems.value[index].name
    actionItems.value.splice(index, 1)
    addLog(`❌ 删除步骤: ${removed}`)
    // 调整选中索引
    if (actionItems.value.length === 0) {
      selectedIndex.value = null
    } else if (selectedIndex.value === index) {
      selectedIndex.value = index >= actionItems.value.length ? index - 1 : index
    } else if (selectedIndex.value > index) {
      selectedIndex.value--
    }
  }
}

// 删除当前选中的项
const deleteSelected = () => {
  if (isRunning.value || selectedIndex.value === null) return
  deleteSingleItem(selectedIndex.value)
}

// 上移选中项
const moveUp = () => {
  if (isRunning.value || selectedIndex.value === null || selectedIndex.value === 0) return
  const idx = selectedIndex.value
  const temp = actionItems.value[idx]
  actionItems.value[idx] = actionItems.value[idx - 1]
  actionItems.value[idx - 1] = temp
  selectedIndex.value = idx - 1
  addLog(`⬆ 上移步骤: ${temp.name}`)
}

// 下移选中项
const moveDown = () => {
  if (isRunning.value || selectedIndex.value === null || selectedIndex.value === actionItems.value.length - 1) return
  const idx = selectedIndex.value
  const temp = actionItems.value[idx]
  actionItems.value[idx] = actionItems.value[idx + 1]
  actionItems.value[idx + 1] = temp
  selectedIndex.value = idx + 1
  addLog(`⬇ 下移步骤: ${temp.name}`)
}

// 选中列表项
const selectItem = (index) => {
  if (isRunning.value) return
  selectedIndex.value = index
}

// 捕获窗口 (模拟添加窗口标题)
const captureWindow = () => {
  if (isRunning.value) {
    addLog('⛔ 运行时无法捕获窗口')
    return
  }
  // 模拟捕获窗口，弹出简易输入框让用户输入窗口名称 (更贴近真实捕获体验)
  const windowTitle = prompt('请输入捕获到的窗口标题/标识', '记事本 - 无标题')
  if (windowTitle && windowTitle.trim()) {
    const newStep = {
      id: Date.now() + Math.random(),
      name: `[窗口] ${windowTitle.trim()}`
    }
    actionItems.value.push(newStep)
    addLog(`🎯 捕获窗口: ${windowTitle.trim()} 已添加至步骤列表`)
    if (actionItems.value.length === 1) selectedIndex.value = 0
  } else if (windowTitle !== null) {
    addLog('⚠️ 捕获窗口名称无效，未添加')
  }
}

// 设置按钮 (模拟)
const openSettings = () => {
  addLog('⚙️ 设置面板 (演示版本，暂无详细配置)')
  alert('设置功能开发中\n快捷键绑定、主题切换等高级配置敬请期待~')
}

// 组件销毁前清理定时器
onUnmounted(() => {
  if (sequenceTimer) clearTimeout(sequenceTimer)
})

// 测试获取所有窗口
async function fetchNonMinimizedWindows() {
  try {
    const windows = await invoke('get_windows');
    console.log('非最小化窗口列表:', windows);

    // 处理窗口数据
    windows.forEach(win => {
      console.log(`窗口标题: ${win.title}, 窗口句柄: ${win.hwnd}`);
    });
  } catch (error) {
    console.error('获取窗口列表时出错:', error);
  }
}

// 测试获取当前窗口
async function fetchCurrentWindow() {
  try {
    const window = await invoke('get_current_window');
    console.log('当前窗口:', window);

    // 处理窗口数据
    console.log(`窗口标题: ${window.title}, 窗口句柄: ${window.handle}`);
  } catch (error) {
    console.error('获取失败:',error);
    }
}
</script>

<style scoped>
/* 全局重置与柔和基调 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.automation-dashboard {
  min-height: 100vh;
  background: linear-gradient(145deg, #f1f5f9 0%, #e6edf4 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  font-family: 'Inter', system-ui, -apple-system, 'Segoe UI', Roboto, Helvetica, sans-serif;
}

.dashboard-card {
  max-width: 1400px;
  width: 100%;
  height: 85vh;
  min-height: 620px;
  background: rgba(255,255,255,0.75);
  backdrop-filter: blur(2px);
  border-radius: 2rem;
  box-shadow: 0 25px 45px -12px rgba(0, 0, 0, 0.25), 0 2px 8px rgba(0,0,0,0.05);
  display: flex;
  overflow: hidden;
  transition: all 0.2s ease;
}

/* 左侧操作区 精致玻璃质感 */
.operation-panel {
  width: 320px;
  background: rgba(255,255,255,0.9);
  backdrop-filter: blur(4px);
  border-right: 1px solid rgba(0,0,0,0.05);
  padding: 1.8rem 1.4rem;
  display: flex;
  flex-direction: column;
  gap: 1.8rem;
}

.software-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 0.4rem;
}

.logo-icon {
  font-size: 2rem;
  background: #2c3e66;
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 18px;
  color: white;
  box-shadow: 0 8px 14px -8px rgba(0, 0, 0, 0.2);
}

.software-title {
  font-size: 1.6rem;
  font-weight: 600;
  background: linear-gradient(135deg, #1a2a3f, #2c3e66);
  background-clip: text;
  -webkit-background-clip: text;
  color: transparent;
  letter-spacing: -0.3px;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.action-btn {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  padding: 12px 18px;
  border-radius: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 0.95rem;
  font-weight: 500;
  color: #1e293b;
  transition: all 0.2s;
  cursor: pointer;
  box-shadow: 0 1px 2px rgba(0,0,0,0.02);
}

.action-btn:hover:not(:disabled) {
  background: #ffffff;
  border-color: #b9c7da;
  transform: translateY(-1px);
  box-shadow: 0 8px 18px -10px rgba(0, 0, 0, 0.1);
}

.action-btn.primary {
  background: #1e2f41;
  border-color: #2d3e54;
  color: white;
}

.action-btn.primary.running {
  background: #9b2c2c;
  border-color: #b13e3e;
}

.action-btn:disabled {
  opacity: 0.55;
  cursor: not-allowed;
  transform: none;
}

.shortcut-key {
  font-family: 'Menlo', monospace;
  background: rgba(0,0,0,0.05);
  padding: 4px 8px;
  border-radius: 20px;
  font-size: 0.7rem;
  letter-spacing: 0.3px;
  color: #475569;
}

.primary .shortcut-key {
  background: rgba(255,255,255,0.2);
  color: #e2e8f0;
}

.icon-settings {
  font-size: 1.1rem;
}

/* 模式切换按钮组 */
.mode-switch-group {
  display: flex;
  gap: 12px;
  background: #f1f5f9;
  padding: 6px;
  border-radius: 60px;
}

.mode-btn {
  flex: 1;
  background: transparent;
  border: none;
  padding: 8px 0;
  border-radius: 40px;
  font-weight: 500;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
  color: #475569;
}

.mode-btn.active {
  background: white;
  color: #1e2f41;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  font-weight: 600;
}

.mode-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 状态区域 + 日志 */
.status-area {
  margin-top: auto;
  background: #f1f4f9;
  border-radius: 24px;
  padding: 14px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px dashed #cbd5e1;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #94a3b8;
  transition: all 0.2s;
}

.status-dot.active {
  background: #3b9e3b;
  box-shadow: 0 0 0 2px #b9f6ca;
}

.status-text {
  font-size: 0.8rem;
  font-weight: 500;
  color: #334155;
}

.log-container {
  height: 130px;
  overflow-y: auto;
  font-size: 0.7rem;
  font-family: monospace;
  background: #ffffffb3;
  border-radius: 16px;
  padding: 8px 10px;
}

.log-line {
  padding: 4px 0;
  color: #2c3e66;
  border-bottom: 1px solid #eef2ff;
  font-size: 0.7rem;
  white-space: nowrap;
  overflow-x: hidden;
  text-overflow: ellipsis;
}

.log-placeholder {
  color: #94a3b8;
  text-align: center;
  font-style: italic;
  padding: 12px 0;
}

/* 右侧操作列表 现代简约 */
.actions-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  padding: 1.8rem 2rem;
  gap: 1.2rem;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  border-bottom: 2px solid #eef2ff;
  padding-bottom: 0.6rem;
}

.panel-header h3 {
  font-weight: 600;
  font-size: 1.35rem;
  color: #0f172a;
}

.step-count {
  font-size: 0.8rem;
  background: #eef2ff;
  padding: 4px 10px;
  border-radius: 30px;
  color: #2c3e66;
}

.action-list-container {
  flex: 1;
  overflow-y: auto;
  background: #fafcff;
  border-radius: 20px;
  border: 1px solid #ecf3fa;
  padding: 8px 4px;
}

.action-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.action-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  background: white;
  border-radius: 16px;
  transition: all 0.15s;
  cursor: pointer;
  border: 1px solid #eef2ff;
}

.action-item:hover {
  background: #f8fafc;
  border-color: #cbdff2;
}

.action-item.selected {
  background: #eef4ff;
  border-left: 4px solid #2c3e66;
  border-radius: 12px;
}

.item-index {
  font-weight: 600;
  width: 32px;
  color: #5b6e8c;
  font-size: 0.8rem;
}

.item-content {
  flex: 1;
  font-size: 0.9rem;
  font-weight: 500;
  color: #1e293b;
}

.item-delete {
  background: none;
  border: none;
  font-size: 1.1rem;
  cursor: pointer;
  color: #94a3b8;
  border-radius: 30px;
  width: 26px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.1s;
}

.item-delete:hover:not(:disabled) {
  background: #fee2e2;
  color: #b91c1c;
}

.item-delete:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.empty-list {
  text-align: center;
  padding: 2rem;
  color: #94a3b8;
  font-size: 0.85rem;
}

/* 控制栏 */
.control-bar {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
  align-items: center;
  background: #f9fbfd;
  padding: 12px 16px;
  border-radius: 40px;
  margin-top: 4px;
}

.add-section {
  display: flex;
  gap: 8px;
  flex: 2;
  min-width: 180px;
}

.add-section input {
  flex: 1;
  padding: 8px 14px;
  border-radius: 40px;
  border: 1px solid #e2e8f0;
  background: white;
  font-size: 0.85rem;
  outline: none;
  transition: 0.2s;
}

.add-section input:focus {
  border-color: #2c3e66;
  box-shadow: 0 0 0 2px rgba(44, 62, 102, 0.2);
}

.move-delete-group {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.ctrl-btn {
  background: white;
  border: 1px solid #dee5ed;
  padding: 6px 16px;
  border-radius: 40px;
  font-size: 0.8rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  color: #2c3e66;
}

.ctrl-btn:hover:not(:disabled) {
  background: #eef2ff;
  border-color: #b9c8e5;
  transform: scale(0.97);
}

.ctrl-btn.delete-btn:hover:not(:disabled) {
  background: #ffefef;
  border-color: #f3c4c4;
  color: #b91c1c;
}

.ctrl-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.add-btn {
  background: #eef2ff;
  border-color: #cddef5;
  font-weight: 600;
}

/* 滚动条美观 */
::-webkit-scrollbar {
  width: 5px;
  height: 5px;
}

::-webkit-scrollbar-track {
  background: #eef2ff;
  border-radius: 10px;
}

::-webkit-scrollbar-thumb {
  background: #b9c8e5;
  border-radius: 10px;
}
</style>