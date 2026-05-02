<template>
  <div class="w-screen h-screen flex items-center justify-center bg-slatet-50">
    <div class="w-full h-full bg-white rounded-xl shadow-sm border border-slate-200 flex flex-col overflow-hidden">
      <div class="pl-2 pr-5 py-2 border-b bg-slate-50/50 border-slate-200 flex flex-col">
<!--        标题-->
        <div class="flex gap-4 items-center">
          <div class="w-10 h-10 flex items-center justify-center shadow-sm">
            <img src="./assets/app-icon.svg"  alt="Backend Clicker"/>
          </div>
          <h1 class="text-xl text-gray-800">后台键鼠</h1>
        </div>

        <div class="flex justify-between">
          <!--显示当前选中的窗口-->
          <div class="">
            <MarqueeText :text="`目标窗口: ${ store.selectedWindow ? store.selectedWindow.title : '未指定' }`"
                         wrapperClass="flex gap-2 rounded px-2 py-1">
            </MarqueeText>
          </div>
        </div>
      </div>
      <div class="flex gap-2 px-4 py-3 bg-white border-b border-slate-100">
        <button
        @click="store.toggleRun"
        class="base-button start-button"
        :class="{'start-button-running' : store.isRunning }"
        >
          <LuPlay v-if="!store.isRunning" size="16"/>
          <LuSquare v-if="store.isRunning" size="16"/>
          <span>{{ store.isRunning ? '停止' : '开始' }}</span>
        </button>
        <button @click="showWindowCapture=true"
                class="base-button bg-sky-50 text-sky-600 border border-sky-200 hover:bg-sky-100">
          <LuBoxSelect size="16" />
          捕捉窗口
        </button>
        <!-- 根组管理 -->
        <select v-model="store.activeRootId" class="px-3 py-1.5 border rounded-lg text-sm min-w-32">
          <option v-for="g in store.rootGroups" :key="g.id" :value="g.id">{{ g.name }}</option>
        </select>
        <button
            @click="store.addRootGroup('新方案')"
            class="base-button bg-indigo-50 text-indigo-600 hover:bg-indigo-100 border border-indigo-200"
        >
          <LuPlus size="16" />
          方案
        </button>

        <!-- 新增总循环次数 -->
        <div class="flex items-center gap-2 ml-2">
          <label class="text-sm text-gray-600">总循环:</label>
          <input
              type="number"
              v-model.number="loopCount"
              min="1"
              step="1"
              class="w-20 px-2 py-1 border rounded text-sm"
              :disabled="store.isRunning"
          />
          <button
              v-if="loopCount === null"
              @click="loopCount = 1"
              class="base-button text-xs border border-gray-200 hover:bg-gray-100"
          >取消无限</button>
          <button
              v-else
              @click="loopCount = null"
              class="base-button text-xs border border-gray-200 hover:bg-gray-100"
          >无限</button>
        </div>

        <button class="base-button ml-auto bg-slate-50 border border-slate-200 text-slate-600 hover:bg-slate-100"
                @click="showSettings = true"
                :disabled="store.isRunning"
        >
          <LuSettings size="16"/>
          <span>设置</span>
        </button>
      </div>
      <!-- 事件列表 -->
      <div class="flex-1 overflow-hidden">
        <EventList
            v-if="store.activeRoot"
            :root-group="store.activeRoot"
            @add="openForm"
            @edit="handleEditFromList"
        />
      </div>
      <!-- 运行状态条 -->
      <div v-if="store.isRunning" class="px-4 py-2.5 bg-emerald-50/50 border-t border-emerald-100 text-emerald-700 text-sm">
        <span class="inline-block w-1.5 h-1.5 bg-emerald-500 rounded-full mr-2 animate-pulse"></span>运行中
      </div>
    </div>

    <!-- 模态框 -->
    <EventForm
        v-if="formVisible"
        :event="editingEvent"
        :is-group="isGroupForm"
        @save="handleSave"
        @cancel="closeForm" />
    <SettingsModal
        v-if="showSettings"
        @close="showSettings = false" />
    <WindowsCaptureModal v-if="showWindowCapture" @close="showWindowCapture = false" />
  </div>
</template>

<script setup lang="ts">
// ----------- 前端js --------------
import {ref, onMounted, computed} from 'vue';
import { LuPlay,
  LuSquare,
  LuSettings,
  LuBoxSelect,
  LuPlus
} from 'vue-icons-plus/lu';
import { useEventStore } from '@/stores/app';
import EventList from '@/component/EventList.vue';
import EventForm from '@/component/EventForm.vue';
import SettingsModal from '@/component/SettingsModal.vue';
import type {EventItemNode, EventTreeNode} from '@/types';
import MarqueeText from "@/component/MarqueeText.vue";
import WindowsCaptureModal from "@/component/WindowsCaptureModal.vue";
import {loadShortcuts} from "@/services/tauri.ts";
import {register} from "@tauri-apps/plugin-global-shortcut";

const store = useEventStore();
const showWindowCapture = ref<boolean>(false);
const showSettings = ref(false);

const formVisible = ref(false);
const editingEvent = ref<EventTreeNode | null>(null);
const isGroupForm = ref(false);
const parentId = ref<string | undefined>();

const loopCount = computed({
  get: () => store.totalLoopCount,
  set: (val: number | null) => store.setTotalLoopCount(val),
});

function openForm(isGroup: boolean, pid?: string) {
  editingEvent.value = null;
  isGroupForm.value = isGroup;
  parentId.value = pid;
  formVisible.value = true;
}

function handleEditFromList(event: EventTreeNode) {
  editingEvent.value = event;
  isGroupForm.value = event.isGroup === true;
  parentId.value = undefined;
  formVisible.value = true;
}

function closeForm() {
  formVisible.value = false;
  editingEvent.value = null;
  parentId.value = undefined;
}

function handleSave(data: Partial<EventTreeNode>) {
  if (editingEvent.value) {
    store.updateEvent(editingEvent.value.id, data);
  } else {
    if (data.isGroup) {
      store.addGroup(data, parentId.value);
    } else {
      store.addEventItem(<EventItemNode>data, parentId.value);
    }
  }
  closeForm();
}

async function initGlobalShortcut() {
  try {
    const {start_stop} = await loadShortcuts();
    if (start_stop) {
      await register(start_stop, event => {
        if (event.state === 'Pressed') {
          store.toggleRun();
        }
      });
    }
  } catch (err) {
    console.error('初始化快捷键失败',err)
  }
}

onMounted(async () => {
  await store.load();      // 加载保存的根组
  await initGlobalShortcut();
});
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