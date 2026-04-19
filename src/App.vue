<template>
  <div class="h-screen bg-gray-100 font-sans antialiased">
    <div class="flex flex-col bg-white h-full">
      <div class=""> <!-- header -->
        <!-- 软件标题 -->
        <div class="pl-2 pr-5 py-2 border-b border-gray-200 flex gap-4 items-center">
          <div class="w-12 h-12 flex items-center justify-center shadow-sm">
            <img src="./assets/app-icon.svg"  alt="Backend Clicker"/></div>
          <h1 class="text-2xl font-bold text-gray-800">后台键鼠</h1>
        </div>
      </div>

      <div class="flex h-full border-b border-b-gray-200 min-h-0 overflow-hidden">  <!-- 主要区域-->
        <!-- 左侧操作区 -->
        <aside class="w-50 bg-white border-r border-gray-200 flex flex-col h-full shrink-0 items-center">
          <!-- 主要操作按钮组 -->
          <div class="flex flex-col gap-6 custom-scroll px-4 py-8 justify-center w-full items-center">

            <button class="primary-button w-full"
                    @click="handleCapture" :disabled="loading">
              <span class="flex">
                <BiScreenshot />
                <span class="text-sm translate-y-0.5">捕获窗口</span>
              </span>
              <kbd class="shortcut">K</kbd>
            </button>
            <ListModal v-model:visible="showModal"
                       title="请选择窗口"
                       :options="windows"
                       @select="handleSelectWindow" />

            <button class="primary-button w-full"
                    @click="store.toggleRun"
                    :class="{ running: store.isRunning ,'stop-button': store.isRunning}">
            <span class="flex">
              <BiPlay v-if="!store.isRunning"/>
              <BiPause v-if="store.isRunning"/>
              <span class="text-sm translate-y-0.5">{{ store.isRunning ? '停止' : '启动' }}</span>
            </span>
              <kbd class="shortcut">F8</kbd>
            </button>



            <!-- 模式切换按钮组 -->
            <div class="flex w-full justify-between border-2 border-brand-tertiary rounded-theme">
              <button
                  class="accent-button-group-item flex-1"
                  :class="{ active: store.currentMode === 'sync', 'bg-brand-tertiary text-white': store.currentMode === 'sync'}"
                  @click="store.setMode('sync')"
                  :disabled="store.isRunning"
              >
            <span class="flex gap-2 px-1">
              <AiOutlineSync />
              <span class="text-sm translate-y-0.5">同步</span>
            </span>
              </button>
              <button
                  class="accent-button-group-item flex-1 items-end"
                  :class="{ active: store.currentMode === 'sequence', 'bg-brand-tertiary text-white': store.currentMode === 'sequence'}"
                  @click="store.setMode('sequence')"
                  :disabled="store.isRunning"
              >
            <span class="flex gap-2 px-1">
              <AiOutlineOrderedList />
              <span class="text-sm translate-y-0.5">序列</span>
            </span>
              </button>
            </div>

            <button class="secondary-button-outlined w-full" @click="router.push('/settings')" :disabled="store.isRunning">
            <span class="flex">
              <LuSettings />
              <span class="text-sm translate-y-0.5 items-center">设置</span>
            </span>
              <span class="text-sm font-light">settings</span>
            </button>
          </div>

          <!-- 运行状态 & 实时日志 -->
          <div class="flex flex-col gap-2 items-center w-full flex-1 min-h-0">
            <div class="card w-full whitespace-nowrap h-full">
              <div class="flex flex-col gap-1 min-w-max">
                <div v-for="(log, idx) in store.logs" :key="idx" class="text-[10px] w-full">
                  {{ log }}
                </div>
                <div v-if="store.logs.length === 0" class="text-[10px]">等待操作...</div>
              </div>

            </div>
          </div>
        </aside>

        <!-- 右侧区域：动态切换列表 / 添加页面 -->
        <main class="flex-1 bg-white flex flex-col min-w-0 h-full overflow-hidden">
          <router-view />
        </main>
      </div>

      <!-- 尾部 -->
      <div class="flex justify-between px-4">
        <div class="flex items-center gap-2 shrink-0">
            <span class="inline-block w-2.5 h-2.5 rounded-full bg-green-500 transition-shadow"
                  :class="{ active: store.isRunning, 'animate-shadow-pulse': store.isRunning }"></span>
          <span class="status-text">{{ store.isRunning ? '运行中' : '就绪' }}</span>
        </div>
        <!--显示当前选中的窗口-->
        <div class="">
          <MarqueeText v-if="store.selectedWindow" :text="`目标窗口: ${ store.selectedWindow.title }`"
                       wrapperClass="flex gap-2 rounded px-2 py-1"><BiTargetLock color="#d64545"/></MarqueeText>
          <MarqueeText v-else text="⚠️ 请先选择目标窗口"></MarqueeText>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
// ----------- 前端js --------------
// 引入Icon
import { BiScreenshot, BiPlay, BiPause, BiTargetLock } from "vue-icons-plus/bi";
import { LuSettings } from "vue-icons-plus/lu";
import {AiOutlineOrderedList, AiOutlineSync } from "vue-icons-plus/ai";

import {ref, onMounted, onUnmounted} from "vue";
import {useRouter} from "vue-router";
import {useAppStore} from "./stores/app.ts";
import ListModal from "./template/ListModal.vue";
import MarqueeText from "./component/MarqueeText.vue";
import type {sWindow} from './types/types.ts';


// ---------- 状态 ----------
const store = useAppStore();
const router = useRouter();

const showModal = ref(false)
const windows = ref<sWindow[]>([])
const loading = ref(false);

const handleCapture = async () => {
  loading.value = true;
  try {
    windows.value = await store.captureWindow()
    if (windows.value.length > 0 ){
      showModal.value = true
    }
  } catch (error) {
    store.addLog(`获取窗口列表失败:${error}`)
  } finally {
    loading.value = false;
  }
}

const handleSelectWindow = (win: sWindow) => {
  store.setSelectedWindow(win)
}

onMounted(async () => {
  await store.loadPersistedData()
  await store.setupListeners()
})

onUnmounted(() => {
  store.cleanupListeners()
})

// const isRunning = ref(false)
// const currentMode = ref('sync')
// const actionItems = ref<FormEvent[]>([])
// const selectedIndex = ref<number>()
// const showModal = ref(false)
// const windows = ref<sWindow[]>([])
// const loading = ref<boolean>(false)
// const logs = ref<string[]>([])
// const viewMode = ref<'list' | 'add'>('list')   // 右侧视图模式
// const selected_windows = ref<sWindow>()
//
// // ---------- 日志 ----------
// const addLog = async (message: string) => {
//   // 前端显示
//   logs.value.unshift(message)
//   if (logs.value.length > 20) logs.value.pop()
//   // 后端记录
//   try {
//     await invoke('log_message', {message})
//   } catch (err) {
//     console.error('Failed to write log:', err)
//   }
// }
//
// // ------- 持久化 ---------
// const loadPersistedData = async () => {
//   try {
//     // 加载事件列表
//     const storedEvents = await invoke<StoredEvent[]>('load_events')
//     console.log('Loaded events:', storedEvents)
//     actionItems.value = storedEvents.map((event) => storedToFormEvent(event))
//     // 加载快捷键
//     const shortcuts = await invoke<any>('load_shortcuts')
//     const saveMode = await invoke<string>('load_mode');
//     if (saveMode === 'sync' || saveMode === 'sequence') {
//       currentMode.value = saveMode;
//       await invoke('set_mode', {mode: saveMode});
//       addLog(`⚙️ 加载上次模式: ${saveMode === 'sync' ? '同步' : '序列'}`);
//     }
//     console.log('Loaded shortcuts:', shortcuts)
//   } catch (err) {
//     console.error('Failed to load persisted data:', err)
//   }
// }
//
// const saveEventsToPersist = async () => {
//   // 将当前actionItems 转换为后端InputEvent 格式 不包含hwnd并保存
//   const backendEvents: InputEvent[] = actionItems.value.map(item => toBackendEvent(item, 0)) // 保存时hwnd使用占位符
//   try {
//     await invoke('save_events', {events: backendEvents})
//   } catch (err) {
//     console.error('Failed to save events:', err)
//   }
// }
//
// // ------------- 执行相关 ---------------
// let unlistenCompletion: (() => void) | null = null
// let unlistenError: (() => void) | null = null
//
// onMounted(async () => {
//   await loadPersistedData()
//   // 监听执行完成事件
//   unlistenCompletion = await listen('execution-completed', () => {
//     isRunning.value = false
//     addLog('✅ 执行完成')
//   })
//   // 监听执行错误事件
//   unlistenError = await listen('execution-error', (event: any) => {
//     isRunning.value = false
//     addLog(`❌ 执行出错: ${event.payload}`)
//   })
// })
//
// onUnmounted(() => {
//   if (unlistenCompletion) unlistenCompletion()
//   if (unlistenError) unlistenError()
// })
//
// const startExecution = async () => {
//   if (!selected_windows.value) {
//     addLog('❌ 未选择目标窗口，请先捕获窗口')
//     isRunning.value = false
//     return
//   }
//   if (actionItems.value.length === 0) {
//     addLog('⚠️ 无动作可执行')
//     isRunning.value = false
//     return
//   }
//
//   const backendEvents = actionItems.value.map((item) =>
//       toBackendEvent(item, selected_windows.value?.hwnd!)
//   )
//     try {
//     await invoke('execute_events', {
//       events: backendEvents,
//       mode: currentMode.value,
//     })
//       addLog(`🚀 启动执行，模式: ${currentMode.value === 'sync' ? '同步' : '序列'}`)
//     } catch (err) {
//       addLog(`❌ 启动失败: ${err}`)
//       isRunning.value = false
//   }
// }
//
// const toggleRun = () => {
//   if (isRunning.value) {
//     invoke('stop_execution')
//         .then(() => addLog('⏸️ 用户停止执行'))
//         .catch(console.error)
//     isRunning.value = false
//   } else {
//     // 运行时检查窗口
//     if (!selected_windows.value) {
//       addLog('❌ 请先捕获并选择目标窗口')
//       alert('请先点击"捕获窗口"选择目标窗口')
//       return
//     }
//     if (actionItems.value.length === 0 ) {
//       addLog('⚠️ 请先添加执行步骤')
//       alert('请先添加执行步骤')
//       return
//     }
//     isRunning.value = true
//     startExecution()
//   }
// }
//
// const setMode = async (mode: string) => {
//   if (isRunning.value) return
//   try {
//     await invoke('set_mode', {mode})
//     currentMode.value = mode
//     await invoke('save_mode', {mode});
//     addLog(`⚙️ 切换至${mode === 'sync' ? '同步模式' : '序列模式'}`)
//   } catch (err) {
//     addLog(`切换模式失败: ${err}`)
//   }
// }
//
// // ---------- 动作列表操作 ----------
// // 新增动作（由 AddEvent 调用）
// const addActionItem = (newEvent: FormEvent) => {
//   actionItems.value.push(newEvent)
//   addLog(`📌 添加步骤: ${formatEvent(newEvent)}`)
//   if (actionItems.value.length === 1) selectedIndex.value = 0
//   // 添加后切回列表视图
//   viewMode.value = 'list'
//   saveEventsToPersist() // 持久化保存
// }
//
// const deleteSingleItem = (index: number) => {
//   if (isRunning.value) return
//   if (index >= 0 && index < actionItems.value.length) {
//     const removed = actionItems.value[index]
//     actionItems.value.splice(index, 1)
//     addLog(`❌ 删除步骤: ${formatEvent(removed)}`)
//     if (actionItems.value.length === 0) {
//       selectedIndex.value = undefined
//     } else if (selectedIndex.value === index) {
//       selectedIndex.value = index >= actionItems.value.length ? index - 1 : index
//     } else if (selectedIndex.value && selectedIndex.value > index) {
//       selectedIndex.value--
//     }
//     saveEventsToPersist() // 持久化保存
//   }
// }
//
// /**
//  * 移动指定索引的项向上一位
//  * @param index 要移动的项的索引
//  */
// const moveUp = (index: number) => {
//   if (isRunning.value) return;
//   if (index <= 0 || index >= actionItems.value.length) return;
//   const temp = actionItems.value[index];
//   actionItems.value[index] = actionItems.value[index - 1];
//   actionItems.value[index - 1] = temp;
//   // 如果移动的项是当前选中的项，更新选中索引
//   if (selectedIndex.value === index) {
//     selectedIndex.value = index - 1;
//   } else if (selectedIndex.value === index - 1){
//     selectedIndex.value = index;
//   }
//   addLog(`⬆ 上移步骤: ${formatEvent(temp)}`)
//   saveEventsToPersist()
// };
//
// /**
//  * 移动指定索引的项向下一位
//  * @param index 要移动的项的索引
//  */
// const moveDown = (index: number) => {
//   if (isRunning.value) return;
//   if (index < 0 || index >= actionItems.value.length - 1) return;
//   const temp = actionItems.value[index];
//   actionItems.value[index] = actionItems.value[index + 1];
//   actionItems.value[index + 1] = temp;
//   if (selectedIndex.value === index) {
//     selectedIndex.value = index + 1;
//   } else if (selectedIndex.value === index + 1){
//     selectedIndex.value = index;
//   }
//   addLog(`⬇ 下移步骤: ${formatEvent(temp)}`)
//   saveEventsToPersist()
// }
//
// const selectItem = (index: number) => {
//   if (isRunning.value) return
//   selectedIndex.value = index
// }
//
// // ---------- 其他功能 ----------
// const openSettings = () => {
//   addLog('⚙️ 设置面板 (演示版本，暂无详细配置)')
//   alert('设置功能开发中\n快捷键绑定、主题切换等高级配置敬请期待~')
// }
//
// const openModal = async () => {
//   windows.value = [];
//   loading.value = true;
//   try {
//     windows.value = await invoke<sWindow[]>('get_windows');
//     if (windows.value.length > 0 ){
//       showModal.value = true;
//     } else {
//       addLog('未找到非最小化窗口')
//     }
//   } catch (err) {
//     addLog(`获取窗口列表失败: ${err}`)
//   } finally {
//     loading.value = false
//   }
// }
//
// const handleSelect = (item: sWindow) => {
//   selected_windows.value = item;
//   addLog(`✅ 已选择目标窗口: ${selected_windows.value.title} (句柄: ${selected_windows.value.hwnd})`);
// }
</script>
<style scoped>
@keyframes shadow-pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(34, 197, 94, 0.5);
  }

  70% {
    box-shadow: 0 0 0 8px rgba(34, 197, 94, 0);
  }

  100% {
    box-shadow: 0 0 0 0 rgba(34, 197, 94, 0);
  }
}

.animate-shadow-pulse {
  animation: shadow-pulse 1.2s infinite ease-in-out;
}
</style>