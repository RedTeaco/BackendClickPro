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
          <div>
            <button class="action-btn" @click="openModal" :disabled="loading">
              <span class="btn-text">捕获窗口</span>
              <kbd class="shortcut-key">K</kbd>
            </button>
            <ListModal v-model:visible="showModal"
                       title="请选择窗口"
                       :options="windows"
                       @select="handleSelect" />
          </div>

          <button class="action-btn primary" @click="toggleRun" :class="{ running: isRunning }">
            <span class="btn-text">{{ isRunning ? '停止' : '启动' }}</span>
            <kbd class="shortcut-key">F8</kbd>
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

<!--显示当前选中的窗口-->

                  <div v-if="selected_windows" class="selected-window-info">
                    📌 目标窗口: {{ selected_windows.title }}
                  </div>
                  <div v-else class="selected-winddow-info warning">
                    ⚠️ 请先选择目标窗口
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

      <!-- 右侧区域：动态切换列表 / 添加页面 -->
      <main class="actions-panel">
        <component
            :is="viewMode === 'list' ? EventList : AddEvent"
            :action-items="actionItems"
            :selected-index="selectedIndex"
            :is-running="isRunning"
            @select-item="selectItem"
            @delete-item="deleteSingleItem"
            @move-up="moveUp"
            @move-down="moveDown"
            @switch-to-add="viewMode = 'add'"
            @add-event="addActionItem"
            @cancel="viewMode = 'list'"
        />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import ListModal from "./template/ListModal.vue";
import EventList from "./template/EventList.vue";
import AddEvent from "./template/AddEvent.vue";

// ---------- 类型定义 ----------
import {FormEvent, InputEvent, sWindow} from "./types/types.ts";
import {formatEvent, toBackendEvent} from "./utils/utils.ts";
import {listen} from "@tauri-apps/api/event";

// ---------- 状态 ----------
const isRunning = ref(false)
const currentMode = ref('sync')
const actionItems = ref<FormEvent[]>([])
const selectedIndex = ref<number>()
const showModal = ref(false)
const windows = ref<sWindow[]>([])
const loading = ref<boolean>(false)
const logs = ref<string[]>([])
const viewMode = ref<'list' | 'add'>('list')   // 右侧视图模式
const selected_windows = ref<sWindow>()

// ---------- 日志 ----------
const addLog = async (message: string) => {
  // 前端显示
  logs.value.unshift(message)
  if (logs.value.length > 20) logs.value.pop()
  // 后端记录
  try {
    await invoke('log_message', {message})
  } catch (err) {
    console.error('Failed to write log:', err)
  }
}

// ------- 持久化 ---------
const loadPersistedData = async () => {
  try {
    // 加载事件列表
    const storedEvents = await invoke<StoredEvent[]>('load_events')
    actionItems.value = storedEvents.map((event) => storedToFormEvent(event))
    // 加载快捷键
    const shortcuts = await invoke<any>('load_shortcuts')
    console.log('Loadded shortcuts:', shortcuts)
  } catch (err) {
    console.error('Failed to load persisted data:', err)
  }
}

const saveEventsToPersist = async () => {
  // 将当前actionItems 转换为后端InputEvent 格式 不包含hwnd并保存
  const backendEvents: InputEvent[] = actionItems.value.map(item => toBackendEvent(item, 0)) // 保存时hwnd使用占位符
  try {
    await invoke('save_events', {events: backendEvents})
  } catch (err) {
    console.error('Failed to save events:', err)
  }
}

// ------------- 执行相关 ---------------
let unlistenCompletion: (() => void) | null = null
let unlistenError: (() => void) | null = null

onUnmounted(async () => {
  await loadPersistedData()
  // 监听执行完成事件
  unlistenCompletion = await listen('execution-completed', () => {
    isRunning.value = false
    addLog('✅ 执行完成')
  })
  // 监听执行错误事件
  unlistenError = await listen('execution-error', (event: any) => {
    isRunning.value = false
    addLog(`❌ 执行出错: ${event.payload}`)
  })
})

onUnmounted(() => {
  if (unlistenCompletion) unlistenCompletion()
  if (unlistenError) unlistenError()
})

const startExecution = async () => {
  if (!selected_windows.value) {
    addLog('❌ 未选择目标窗口，请先捕获窗口')
    isRunning.value = false
    return
  }
  if (actionItems.value.length === 0) {
    addLog('⚠️ 无动作可执行')
    isRunning.value = false
    return
  }

  const backendEvents = actionItems.value.map((item) =>
      toBackendEvent(item, selected_windows.value?.hwnd!)
  )
    try {
    await invoke('execute_events', {
      events: backendEvents,
      mode: currentMode.value,
    })
      addLog(`🚀 启动执行，模式: ${currentMode.value === 'sync' ? '同步' : '序列'}`)
    } catch (err) {
      addLog(`❌ 启动失败: ${err}`)
      isRunning.value = false
  }
}

const toggleRun = () => {
  if (isRunning.value) {
    invoke('stop_execution')
        .then(() => addLog('⏸️ 用户停止执行'))
        .catch(console.error)
    isRunning.value = false
  } else {
    // 运行时检查窗口
    if (!selected_windows.value) {
      addLog('❌ 请先捕获并选择目标窗口')
      alert('请先点击"捕获窗口"选择目标窗口')
      return
    }
    if (actionItems.value.length === 0 ) {
      addLog('⚠️ 请先添加执行步骤')
      alert('请先添加执行步骤')
      return
    }
    isRunning.value = true
    startExecution()
  }
}

const setMode = async (mode: string) => {
  if (isRunning.value) return
  try {
    await invoke('set_mode', {mode})
    currentMode.value = mode
    addLog(`⚙️ 切换至${mode === 'sync' ? '同步模式' : '序列模式'}`)
  } catch (err) {
    addLog(`切换模式失败: ${err}`)
  }
}

// ---------- 动作列表操作 ----------
// 新增动作（由 AddEvent 调用）
const addActionItem = (newEvent: FormEvent) => {
  actionItems.value.push(newEvent)
  addLog(`📌 添加步骤: ${formatEvent(newEvent)}`)
  if (actionItems.value.length === 1) selectedIndex.value = 0
  // 添加后切回列表视图
  viewMode.value = 'list'
  saveEventsToPersist() // 持久化保存
}

const deleteSingleItem = (index: number) => {
  if (isRunning.value) return
  if (index >= 0 && index < actionItems.value.length) {
    const removed = actionItems.value[index]
    actionItems.value.splice(index, 1)
    addLog(`❌ 删除步骤: ${formatEvent(removed)}`)
    if (actionItems.value.length === 0) {
      selectedIndex.value = undefined
    } else if (selectedIndex.value === index) {
      selectedIndex.value = index >= actionItems.value.length ? index - 1 : index
    } else if (selectedIndex.value && selectedIndex.value > index) {
      selectedIndex.value--
    }
    saveEventsToPersist() // 持久化保存
  }
}

/**
 * 移动指定索引的项向上一位
 * @param index 要移动的项的索引
 */
const moveUp = (index: number) => {
  if (isRunning.value) return;
  if (index <= 0 || index >= actionItems.value.length) return;
  const temp = actionItems.value[index];
  actionItems.value[index] = actionItems.value[index - 1];
  actionItems.value[index - 1] = temp;
  // 如果移动的项是当前选中的项，更新选中索引
  if (selectedIndex.value === index) {
    selectedIndex.value = index - 1;
  } else if (selectedIndex.value === index - 1){
    selectedIndex.value = index;
  }
  addLog(`⬆ 上移步骤: ${formatEvent(temp)}`)
  saveEventsToPersist()
};

/**
 * 移动指定索引的项向下一位
 * @param index 要移动的项的索引
 */
const moveDown = (index: number) => {
  if (isRunning.value) return;
  if (index < 0 || index >= actionItems.value.length - 1) return;
  const temp = actionItems.value[index];
  actionItems.value[index] = actionItems.value[index + 1];
  actionItems.value[index + 1] = temp;
  if (selectedIndex.value === index) {
    selectedIndex.value = index + 1;
  } else if (selectedIndex.value === index + 1){
    selectedIndex.value = index;
  }
  addLog(`⬇ 下移步骤: ${formatEvent(temp)}`)
  saveEventsToPersist()
}

const selectItem = (index: number) => {
  if (isRunning.value) return
  selectedIndex.value = index
}

// ---------- 其他功能 ----------
const openSettings = () => {
  addLog('⚙️ 设置面板 (演示版本，暂无详细配置)')
  alert('设置功能开发中\n快捷键绑定、主题切换等高级配置敬请期待~')
}

const openModal = async () => {
  windows.value = [];
  loading.value = true;
  try {
    windows.value = await invoke<sWindow[]>('get_windows');
    if (windows.value.length > 0 ){
      showModal.value = true;
    } else {
      addLog('未找到非最小化窗口')
    }
  } catch (err) {
    addLog(`获取窗口列表失败: ${err}`)
  } finally {
    loading.value = false
  }
}

const handleSelect = (item: sWindow) => {
  selected_windows.value = item;
  addLog(`✅ 已选择目标窗口: ${selected_windows.value.title} (句柄: ${selected_windows.value.hwnd})`);
}
</script>

<style scoped>
/* 左侧面板样式及全局布局保持不变，右侧面板样式只保留外层容器 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.selected-window-info {
  background: #eef2ff;
  border-radius: 20px;
  padding: 8px 12px;
  font-size: 0.75rem;
  text-align: center;
  margin-top: 8px;
}
.selected-window-info.warning {
  background: #fee2e2;
  color: #b91c1c;
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

/* 右侧外层容器样式（保持背景和间距） */
.actions-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  padding: 1.8rem 2rem;
  gap: 1.2rem;
  overflow-y: auto;
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