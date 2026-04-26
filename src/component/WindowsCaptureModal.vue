<template>
  <div class="fixed inset-0 bg-slate-900/20 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="bg-white rounded-xl shadow-lg w-125 max-h-[70vh] flex flex-col border border-slate-200">
<!--      头部-->
      <div class="flex items-center justify-between px-5 py-4 border-b border-slate-100">
      <h3 class="text-base text-slate-700">选择目标窗口</h3>
        <button @click="$emit('close')"
                class="p-1.5 hover:bg-slate-100 rounded-lg text-slate-400 hover:text-slate-600">
          <LuX size="4" />
        </button>
      </div>

<!--      搜索框-->
      <div class="px-4 py-3">
        <input
          v-model="searchTerm"
          type="text"
          placeholder="搜索窗口标题..."
          class="w-full px-3 py-2 border border-slate-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-indigo-200" />
      </div>

<!--      窗口列表-->
      <div class="flex-1 overflow-auto px-4 pb-4">
        <div v-if="store.availableWindows.length === 0"
             class="text-center text-slate-400 py-8 text-sm">
          正在加载窗口列表...
        </div>
        <div v-else-if="filteredWindows.length === 0 "
             class="text-center text-slate-400 py-8 text-sm">
          没有匹配的窗口
        </div>
        <div
          v-for="win in filteredWindows"
          :key="win.hwnd"
          @click="selectWindow(win.hwnd)"
          class="flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-colors hover:bg-slate-50 border border-transparent hover:border-slate-200"
          :class="{'bg-indigo-50 border-indigo-200':selectedHwnd === win.hwnd}"
          >
          <LuMonitor class="text-slate-400" size="4" />
          <div class="flex-1 min-w-0">
            <div class="text-sm text-slate-700 truncate">{{win.title}}</div>
          </div>
          <div v-if="selectedHwnd === win.hwnd" class="text-indigo-600">
            <LuCheck size="4" />
          </div>
        </div>
      </div>

<!--      底部按钮-->
      <div class="px-5 py-4 border-t border-slate-100 flex justify-between">
        <button
          @click="refreshList"
          class="px-3 py-2 text-sm border border-slate-200 rounded-lg hover:bg-slate-50 flex items-center gap-1.5 cursor-pointer">
          <LuRefreshCw size="3.5" />
          刷新
        </button>
        <div class="flex gap-2">
          <button
            @click="$emit('close')"
            class="px-4 py-2 border border-slate-200 text-slate-600 rounded-lg hover:bg-slate-50 text-sm cursor-pointer">
            取消
          </button>
          <button
            @click="confirmSelection"
            class="px-4 py-2 bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 text-sm cursor-pointer"
            :disabled="!selectedHwnd"
            >
            确定
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import {LuX, LuMonitor, LuCheck, LuRefreshCw} from 'vue-icons-plus/lu';
import {useEventStore} from '@/stores/app.ts';
import {computed, onMounted, ref} from "vue";
const store = useEventStore();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const searchTerm = ref('');
const selectedHwnd = ref<number | null>(store.selectedWindow?.hwnd?? null);

const filteredWindows = computed(() => {
  const term = searchTerm.value.trim().toLowerCase();
  if (!term) return store.availableWindows;
  return store.availableWindows.filter((win) => win.title.toLowerCase().includes(term));
});

async function refreshList() {
  await store.fetchWindows();
}

function selectWindow(hwnd: number) {
  selectedHwnd.value = hwnd;
}

function confirmSelection() {
  if (selectedHwnd.value) {
    store.selectWindow(selectedHwnd.value);
  }
  emit('close');
}
onMounted(() => {
    store.fetchWindows();
});
</script>