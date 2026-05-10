<template>
  <div class="fixed inset-0 bg-slate-900/20 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="bg-white rounded-xl shadow-lg w-125 max-h-[80vh] overflow-auto border border-slate-200">
      <div class="flex flex-col items-center justify-between px-5 py-4 border-b border-slate-100 gap-4">
        <div class="flex items-center justify-between w-full">
          <h3 class="text-base text-slate-700">设置</h3>
          <button @click="$emit('close')" class="p-1.5  rounded-lg hover:bg-brand-stop hover:text-white cursor-pointer">
            <LuX size="16" />
          </button>
        </div>
        <div class="flex flex-col justify-center gap-2 border border-gray-100 w-full">
          <h4>快捷键设置</h4>
          <div class="flex flex-nowrap items-center justify-between gap-2">
            <div class="flex flex-nowrap gap-2 items-center">
              <label>开始/停止</label>
              <input
                  type="text"
                  v-model="shortcutInput"
                  @focus="startRecording"
                  @blur="stopRecording"
                  @keydown="handleKeyDown"
                  placeholder="点击后按下快捷键"
                  class="border border-gray-300 rounded-lg px-2 py-1 w-25"
              />
            </div>
            <div class="self-end flex flex-nowrap gap-2 items-center">
              <button class="base-button border border-rose-200 bg-rose-50 hover:bg-rose-100" @click="resetShortcut">恢复默认</button>
              <button class="base-button border border-indigo-200 bg-indigo-50 hover:bg-indigo-100" @click="saveShortcut">保存</button>
            </div>
          </div>
        </div>
        <button @click="$emit('close')" class="base-button bg-indigo-500 hover:bg-indigo-400 text-white">确定</button>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import {LuX} from 'vue-icons-plus/lu';
import {useEventStore} from "@/stores/app.ts";
import {onMounted, onUnmounted, ref} from "vue";
import {register, unregister} from "@tauri-apps/plugin-global-shortcut";
import {loadShortcuts, saveShortcuts} from "@/services/tauri.ts";
defineEmits<{ (e: 'close'): void }>();

const store = useEventStore();
const shortcutInput = ref('');
const isRecording = ref(false);
let currentShortcut: string | null = null; // 当前生效的快捷键

let recordedKeys: Set<string> = new Set();

function recordShortcut(event: KeyboardEvent): void {
  event.preventDefault();
  // 添加修饰键
  if (event.ctrlKey) recordedKeys.add('Control');
  else recordedKeys.delete('Control');
  if (event.metaKey) recordedKeys.add('Command');
  else recordedKeys.delete('Command');
  if (event.shiftKey) recordedKeys.add('Shift');
  else recordedKeys.delete('Shift');
  if (event.altKey) recordedKeys.add('Alt');
  else recordedKeys.delete('Alt');

  // 获取主键（非修饰键）
  let mainKey = event.key;
  const ignoredKeys = ['Control', 'Command', 'Shift', 'Alt', 'Meta', 'Tab', 'CapsLock', 'Unidentified', 'Process'];
  if (mainKey && !ignoredKeys.includes(mainKey)) {
    recordedKeys.add(mainKey);
  } else {
    // 如果当前没有主键，移除之前可能记录的主键（因为用户可能先按了主键再按修饰键，实际上应该以最后按下的主键为准）
    // 这里简单策略：清除所有非修饰键，因为新按下的不是主键
    for (let k of recordedKeys) {
      if (!['Control', 'Command', 'Shift', 'Alt'].includes(k)) {
        recordedKeys.delete(k);
      }
    }
  }

  // 将 Set 转换为数组，并按固定顺序排序（修饰键在前，主键在后）
  const modifiers = ['Control', 'Command', 'Shift', 'Alt'];
  const keys: string[] = [];
  modifiers.forEach(mod => {
    if (recordedKeys.has(mod)) keys.push(mod);
  });
  const mainKeys = Array.from(recordedKeys).filter(k => !modifiers.includes(k));
  if (mainKeys.length > 0) {
    // 如果有多个主键（不太可能，但取最后一个）
    keys.push(mainKeys[mainKeys.length - 1]);
  }

  if (keys.length === 0) return;

  let shortcut = keys.join('+');
  // 标准化 CommandOrControl
  if (shortcut.includes('Command') || shortcut.includes('Control')) {
    shortcut = shortcut.replace(/Command|Control/g, 'CommandOrControl');
  }
  shortcutInput.value = shortcut;
}

// 结束录制的时机：例如用户按下 Enter 或失去焦点，或者延迟自动结束
function stopRecording() {
  isRecording.value = false;
  recordedKeys.clear();
}

// 在模板中，使用 @keydown 而不是 @keydown.enter 等，并监听 window 的 blur 事件
function startRecording() {
  isRecording.value = true;
  recordedKeys.clear();
  shortcutInput.value = '';
  // 设置全局事件，在组件卸载时清理
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('blur', stopRecording);
  // 可选：设置超时自动结束
  setTimeout(() => {
    if (isRecording.value) stopRecording();
  }, 5000);
}

function handleKeyDown(event: KeyboardEvent) {
  if (!isRecording.value) return;
  recordShortcut(event);
  // 当按下的键是 Enter 或 Escape 时立即结束录制
  if (event.key === 'Enter' || event.key === 'Escape') {
    stopRecording();
    if (event.key === 'Escape') shortcutInput.value = '';
    event.preventDefault();
  }
}

async function applyShortcut(shortcut: string) {
  try {
    if  (currentShortcut) {
      await unregister(currentShortcut);
    }
    await register(shortcut,event => {
      if (event.state === "Pressed"){
        store.toggleRun()
      }
    });
    currentShortcut = shortcut;
  } catch (err) {
    console.error("注册快捷键失败", err);
  }
}

async function saveShortcut() {
  const newShortcut = shortcutInput.value.trim();
  if (!newShortcut) return;
  await applyShortcut(newShortcut);
  await saveShortcuts({ start_stop: newShortcut,capture_window: 'K'});
}

function resetShortcut(){
  shortcutInput.value = 'F8';
  applyShortcut(shortcutInput.value);
}

async function loadShortcutSettings() {
  const config = await loadShortcuts();
  const saved = config.start_stop;
  if (saved) {
    currentShortcut = saved;
    shortcutInput.value = saved;
    await applyShortcut(saved);
  } else {
    shortcutInput.value = 'F8';
    await applyShortcut('F8');
  }
}

onMounted(() => {
  loadShortcutSettings();
})

// 在组件卸载时清理监听
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  window.removeEventListener('blur', stopRecording);
});

</script>
<style scoped>
</style>