import {defineStore} from "pinia";
import {FormEvent, InputEvent, StoredEvent, sWindow} from "../types/types.ts";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {formatEvent, storedToFormEvent, toBackendEvent} from "../utils/utils.ts";
import {listen} from "@tauri-apps/api/event";

export const useAppStore = defineStore('app', () => {
    const isRunning = ref(false);
    const currentMode = ref('sync');
    const actionItems = ref<FormEvent[]>([])
    const selectedIndex = ref<number | undefined>()
    const selectedWindow = ref<sWindow | undefined>()
    const logs = ref<string[]>([])

    // ----- 日志（前端显示+后端持久化）-----
    const addLog = async (message: string) => {
        logs.value.unshift(message)
        if (logs.value.length > 20) logs.value.pop()
        try {
            await invoke('log_message', { message })
        } catch (err) {
            console.error('Failed to write log:', err)
        }
    }

    // ----- 持久化加载 -----
    const loadPersistedData = async () => {
        try {
            const storedEvents = await invoke<StoredEvent[]>('load_events')
            actionItems.value = storedEvents.map((e) => storedToFormEvent(e))
            const mode = await invoke<string>('load_mode')
            if (mode === 'sync' || mode === 'sequence') {
                currentMode.value = mode
                await invoke('set_mode', { mode: mode })
                addLog(`⚙️ 加载上次模式: ${mode === 'sync' ? '同步' : '序列'}`)
            }
            await invoke('load_shortcuts') // 快捷键可后续处理
        } catch (err) {
            console.error('Failed to load persisted data:', err)
        }
    }

    // ----- 保存事件列表 -----
    const saveEventsToPersist = async () => {
        // 将当前actionItems 转换为后端InputEvent 格式 不包含hwnd并保存
        const backendEvents: InputEvent[] = actionItems.value.map(item => toBackendEvent(item, 0)) // 保存时hwnd使用占位符
        try {
            await invoke('save_events', {events: backendEvents})
        } catch (err) {
            console.error('Failed to save events:', err)
        }
    }

    // ----- 切换运行状态 -----
    const toggleRun = async () => {
        if (isRunning.value) {
            await invoke('stop_execution')
            addLog('⏸️ 用户停止执行')
            isRunning.value = false
            return
        }

        if (!selectedWindow.value) {
            addLog('❌ 请先捕获并选择目标窗口')
            alert('请先点击"捕获窗口"选择目标窗口')
            return
        }
        if (actionItems.value.length === 0) {
            addLog('⚠️ 请先添加执行步骤')
            alert('请先添加执行步骤')
            return
        }

        isRunning.value = true
        const backendEvents = actionItems.value.map((item) =>
            toBackendEvent(item, selectedWindow.value!.hwnd)
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

    // ----- 切换模式 -----
    const setMode = async (mode: string) => {
        if (isRunning.value) return
        try {
            await invoke('set_mode', { mode })
            currentMode.value = mode
            await invoke('save_mode', { mode })
            addLog(`⚙️ 切换至${mode === 'sync' ? '同步模式' : '序列模式'}`)
        } catch (err) {
            addLog(`切换模式失败: ${err}`)
        }
    }

    // ----- 动作列表操作（会触发保存）-----
    const addActionItem = (event: FormEvent) => {
        actionItems.value.push(event)
        addLog(`📌 添加步骤: ${formatEvent(event)}`)
        if (actionItems.value.length === 1) selectedIndex.value = 0
        saveEventsToPersist()
    }

    const deleteActionItem = (index: number) => {
        if (isRunning.value) return
        if (index < 0 || index >= actionItems.value.length) return
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
        saveEventsToPersist()
    }

    const moveUp = (index: number) => {
        if (isRunning.value || index <= 0) return
        const temp = actionItems.value[index]
        actionItems.value[index] = actionItems.value[index - 1]
        actionItems.value[index - 1] = temp
        if (selectedIndex.value === index) selectedIndex.value = index - 1
        else if (selectedIndex.value === index - 1) selectedIndex.value = index
        addLog(`⬆ 上移步骤: ${formatEvent(temp)}`)
        saveEventsToPersist()
    }

    const moveDown = (index: number) => {
        if (isRunning.value || index >= actionItems.value.length - 1) return
        const temp = actionItems.value[index]
        actionItems.value[index] = actionItems.value[index + 1]
        actionItems.value[index + 1] = temp
        if (selectedIndex.value === index) selectedIndex.value = index + 1
        else if (selectedIndex.value === index + 1) selectedIndex.value = index
        addLog(`⬇ 下移步骤: ${formatEvent(temp)}`)
        saveEventsToPersist()
    }

    const selectItem = (index: number) => {
        if (isRunning.value) return
        selectedIndex.value = index
    }

    // ----- 窗口捕获 -----
    const captureWindow = async () => {
        const windows = await invoke<sWindow[]>('get_windows')
        if (windows.length === 0) {
            addLog('未找到非最小化窗口')
            return []
        }
        return windows
    }

    const setSelectedWindow = (win: sWindow) => {
        selectedWindow.value = win
        addLog(`✅ 已选择目标窗口: ${win.title} (句柄: ${win.hwnd})`)
    }

    // ----- 事件监听（在 App.vue 挂载时调用）-----
    let unlistenCompletion: (() => void) | null = null
    let unlistenError: (() => void) | null = null

    const setupListeners = async () => {
        unlistenCompletion = await listen('execution-completed', () => {
            isRunning.value = false
            addLog('✅ 执行完成')
        })
        unlistenError = await listen('execution-error', (event: any) => {
            isRunning.value = false
            addLog(`❌ 执行出错: ${event.payload}`)
        })
    }

    const cleanupListeners = () => {
        unlistenCompletion?.()
        unlistenError?.()
    }

    return {
        // state
        isRunning,
        currentMode,
        actionItems,
        selectedIndex,
        selectedWindow,
        logs,
        // actions
        addLog,
        loadPersistedData,
        toggleRun,
        setMode,
        addActionItem,
        deleteActionItem,
        moveUp,
        moveDown,
        selectItem,
        captureWindow,
        setSelectedWindow,
        setupListeners,
        cleanupListeners,
    }
})