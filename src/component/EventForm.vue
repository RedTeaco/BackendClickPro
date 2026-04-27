<template>
  <div class="fixed inset-0 bg-slate-900/20 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="bg-white rounded-xl shadow-lg w-110 max-h-[85vh] overflow-auto border border-slate-200">
      <div class="flex items-center justify-between px-5 py-4 border-b border-slate-100">
        <h3 class="text-base text-slate-700">{{ title }}</h3>
        <button @click="emit('cancel')" class="p-1.5 hover:bg-brand-stop/60 hover:text-brand-stop rounded-lg cursor-pointer">
          <LuX size="16" />
        </button>
      </div>

      <form @submit.prevent="handleSubmit" class="p-5 space-y-5">
        <!-- 分组名称（仅分组模式） -->
        <div v-if="isGroup">
          <label class="block text-sm text-slate-600 mb-2">分组名称</label>
          <input type="text" v-model="form.name" required class="w-full px-3 py-2 border rounded-lg" />
        </div>

        <!-- 非分组事件表单 -->
        <template v-else>
          <!-- 类型：鼠标/键盘 -->
          <div>
            <label class="block text-sm text-slate-600 mb-2">类型</label>
            <div class="flex gap-4 justify-center">
              <button type="button" @click="form.type = 'mouse'"
                      class="base-button p-4 border border-indigo-100"
                      :class="[form.type === 'mouse'
                      ? 'border-indigo-300 bg-indigo-50 text-indigo-700'
                      : 'unselected-button']"
              >
                <LuMouse size="16" />
                鼠标
              </button>
              <button type="button" @click="form.type = 'keyboard'"
                      class="base-button border border-indigo-100"
                      :class="[form.type === 'keyboard'
                      ? 'border-indigo-300 bg-indigo-50 text-indigo-700'
                      : 'unselected-button']">
                <LuKeyboard size="16" />
                键盘
              </button>
            </div>
          </div>

          <!-- 动作类型：点击/长按/滚轮(仅鼠标) -->
          <div>
            <label class="block text-sm text-slate-600 mb-2">动作类型</label>
            <div class="flex gap-2 justify-center">
              <button type="button" @click="form.actionType = 'click'"
                      class="base-button p-2 border borderr-sky-100 flex-1"
                      :class="[form.actionType === 'click'
                      ? 'border-sky-300 bg-sky-50 text-sky-700'
                      : 'unselected-button']"
              >
                点击
              </button>
              <button type="button" @click="form.actionType = 'longPress'"
                      class="base-button p-2 border borderr-sky-100 flex-1"
                      :class="[form.actionType === 'longPress'
                      ? 'border-sky-300 bg-sky-50 text-sky-700'
                      : 'unselected-button'
               ]"
              >
                长按
              </button>
              <button v-if="form.type === 'mouse'" type="button" @click="form.actionType = 'scroll'"
                      class="base-button p-2 border borderr-sky-100 flex-1"
                      :class="[form.actionType === 'scroll'
                      ? 'border-sky-300 bg-sky-50 text-sky-700'
                      : 'unselected-button'
               ]"
              >
                滚轮
              </button>
            </div>
          </div>

          <!-- 鼠标按键选择 -->
          <div v-if="form.type === 'mouse'">
            <label class="block text-sm text-slate-600 mb-2">鼠标按键</label>
            <div class="flex gap-2 justify-center" v-if="form.actionType !== 'scroll'">
              <button type="button" @click="form.mouseButton = 'left'"
                      class="base-button p-2 border border-violet-100 flex-1"
                      :class="[form.mouseButton === 'left' ? 'border-violet-300 bg-violet-50 text-violet-700' : 'unselected-button']"
              >左键</button>
              <button type="button" @click="form.mouseButton = 'middle'"
                      class="base-button p-2 border border-violet-100 flex-1"
                      :class="[form.mouseButton === 'middle' ? 'border-violet-300 bg-violet-50 text-violet-700' : 'unselected-button']"
              >中键</button>
              <button type="button" @click="form.mouseButton = 'right'"
                      class="base-button p-2 border border-violet-100 flex-1"
                      :class="[form.mouseButton === 'right' ? 'border-violet-300 bg-violet-50 text-violet-700' : 'unselected-button']"
              >右键</button>
            </div>
            <div class="flex gap-2 justify-center" v-else>
              <button v-if="form.actionType === 'scroll'" type="button" @click="form.mouseButton = 'wheel'"
                      class="base-button p-2 border border-violet-100 items-center justify-center"
                      :class="[form.actionType === 'scroll' ? 'border-violet-300 bg-violet-50 text-violet-700' : 'unselected-button']"
              >
                滚轮</button>
            </div>
          </div>

          <!-- 键盘按键 -->
          <div v-if="form.type === 'keyboard'">
            <label class="block text-sm text-slate-600 mb-2">按键</label>
            <button type="button" @click="listenForKey" class="w-full px-4 py-3 border-2 rounded-lg">
              {{ listening ? '按下任意键...' : (form.keyboardKey || '点击选择按键') }}
            </button>
          </div>

          <!-- 持续长按选项 -->
          <div v-if="form.actionType === 'longPress'" class="flex items-center gap-2 py-2 px-3 bg-slate-50 rounded-lg">
            <input type="checkbox" id="continuous" v-model="form.isContinuous" />
            <label for="continuous" class="text-sm text-slate-600 cursor-pointer">持续长按（直到手动停止）<small>期间请勿在窗口内使用键盘</small></label>
          </div>

          <!-- 非持续时显示的参数 -->
          <template v-if="!form.isContinuous">
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block text-sm text-slate-600 mb-2">间隔时间 (ms)</label>
                <input type="number" step="0.1" v-model="form.interval" class="w-full px-3 py-1.5 border rounded-lg" />
              </div>
              <div>
                <label class="block text-sm text-slate-600 mb-2">循环次数</label>
                <input type="number" v-model="form.loopCount" placeholder="<=0无限" class="w-full px-3 py-1.5 border rounded-lg" />
              </div>
            </div>
            <div v-if="form.actionType === 'longPress'">
              <label class="block text-sm text-slate-600 mb-2">持续时间 (ms)</label>
              <input type="number" step="0.1" v-model="form.duration" class="w-full px-3 py-1.5 border rounded-lg" />
            </div>
          </template>

          <!-- 滚轮特有：Delta -->
          <div v-if="form.actionType === 'scroll'">
            <label class="block text-sm text-slate-600 mb-2">滚动增量（正数向下/负数向上）</label>
            <input type="number" v-model="form.scrollDelta" class="w-full px-3 py-1.5 border rounded-lg" />
          </div>

          <!-- 坐标（鼠标事件） -->
          <div v-if="form.type === 'mouse'">
            <label class="block text-sm text-slate-600 mb-2">坐标X,Y</label>
            <div class="grid grid-cols-2 gap-2">
              <input type="number" v-model="form.coordinates.x" placeholder="X" class="w-full px-3 py-1.5 border rounded-lg" />
              <input type="number" v-model="form.coordinates.y" placeholder="Y" class="w-full px-3 py-1.5 border rounded-lg" />
            </div>
          </div>
        </template>

        <div class="flex gap-2 pt-3 border-t">
          <button type="button" @click="emit('cancel')" class="base-button flex-1 px-4 py-2.5 border hover:bg-slate-100 rounded-lg">取消</button>
          <button type="submit" class="base-button flex-1 px-4 py-2.5 bg-indigo-500 text-white hover:bg-indigo-400 rounded-lg">保存</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from 'vue';
import { LuX, LuMouse, LuKeyboard } from 'vue-icons-plus/lu';
import type { EventTreeNode } from '@/types';

const props = defineProps<{
  event: EventTreeNode | null;
  isGroup: boolean;
}>();
const emit = defineEmits<{
  (e: 'save', data: Partial<EventTreeNode>): void;
  (e: 'cancel'): void;
}>();

const title = computed(() => {
  const prefix = props.event ? '编辑' : '添加';
  return prefix + (props.isGroup ? '分组' : '事件');
});

const form = reactive({
  name: '',
  type: 'mouse' as 'mouse' | 'keyboard',
  actionType: 'click' as 'click' | 'longPress' | 'scroll',
  mouseButton: 'left' as 'left' | 'middle' | 'right' | 'wheel',
  keyboardKey: '',
  isContinuous: false,
  interval: 1.0,
  duration: 1.0,
  loopCount: null as number | null,
  coordinates: { x: 0, y: 0 },
  scrollDelta: 0,
});

const listening = ref(false);

// 初始化表单
watch(() => props.event, (ev) => {
  if (ev) {
    if (!(ev.isGroup)) {
      Object.assign(form, {
        name: ev.name || '',
        type: ev.type || 'mouse',
        actionType: ev.actionType || 'click',
        mouseButton: ev.mouseButton || 'left',
        keyboardKey: ev.keyboardKey || '',
        isContinuous: ev.isContinuous || false,
        interval: ev.interval ?? 1.0,
        duration: ev.duration ?? 1.0,
        loopCount: ev.loopCount ?? null,
        coordinates: ev.coordinates || {x: 0, y: 0},
        scrollDelta: ev.scrollDelta ?? 0,
      });
    }
  }
}, { immediate: true });

function listenForKey() {
  listening.value = true;
  const handler = (e: KeyboardEvent) => {
    e.preventDefault();
    form.keyboardKey = e.code;
    listening.value = false;
    window.removeEventListener('keydown', handler);
  };
  window.addEventListener('keydown', handler);
}

function handleSubmit() {
  let data: Partial<EventTreeNode> = {};

  if (props.isGroup) {
    data = { name: form.name, isGroup: true };
  } else {
    data = {
      type: form.type,
      actionType: form.actionType,
      mouseButton: form.type === 'mouse' ? form.mouseButton : undefined,
      keyboardKey: form.type === 'keyboard' ? form.keyboardKey : undefined,
      isContinuous: form.isContinuous,
      interval: form.isContinuous ? undefined : form.interval,
      duration: form.actionType === 'longPress' ? form.duration : undefined,
      loopCount: form.isContinuous ? null : (form.loopCount && form.loopCount > 0 ? form.loopCount : null),
      coordinates: form.type === 'mouse' ? { ...form.coordinates } : undefined,
      scrollDelta: form.actionType === 'scroll' ? form.scrollDelta : undefined,
    };
    // 自动生成名称
    data.name = generateName(data);
  }
  emit('save', data);
  emit('cancel');
}

function generateName(data: any): string {
  let name = data.type === 'mouse' ? '鼠标' : '键盘';
  if (data.actionType === 'click') name += '点击';
  else if (data.actionType === 'longPress') name += data.isContinuous ? '持续长按' : '长按';
  else name += `滚轮(${data.scrollDelta})`;
  return name;
}

</script>