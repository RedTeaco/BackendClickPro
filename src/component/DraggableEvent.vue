<template>
  <div
      :class="[
      'drag-wrapper transition-all',
      isDragOver ? 'ring-2 ring-indigo-300' : ''
    ]"
      @dragenter="onDragEnter"
      @dragover.prevent
      @drop.prevent="onDrop"
  >
    <!-- 分组节点 -->
    <template v-if="event.isGroup">
      <div
          :class="[
        'group flex items-center gap-2 px-3 py-2.5 rounded-lg transition-all cursor-pointer mb-2',
        isSelected
        ? 'bg-violet-50 border border-violet-200 shadow-sm'
        : 'bg-linear-to-br from-slate-50 to-slate-100/50 border border-slate-200 hover:border-violet-200 hover:shadow-sm'
      ]"
          @click="handleSelect"
      >
        <!-- 拖拽手柄（必须设置 draggable，dragstart 在此元素触发） -->
        <div
            class="cursor-grab active:cursor-grabbing text-slate-400 hover:text-slate-600"
            draggable="true"
            @dragstart="onDragStart"
            @dragend="onDragEnd"
        >
          <LuGripVertical class="w-4 h-4" />
        </div>

        <!-- 序号（序列模式显示） -->
        <div v-if="showSequence && sequenceNumber" class="flex items-center justify-center w-5 h-5 rounded-full bg-indigo-100 text-indigo-700 text-xs font-medium">
          {{ sequenceNumber }}
        </div>

        <!-- 展开/折叠 -->
        <button class="p-0.5 text-slate-500 hover:text-slate-700" @click.stop="$emit('toggle-expand', event.id)">
          <component :is="isExpanded ? LuChevronDown : LuChevronRight" class="w-4 h-4" />
        </button>

        <LuFolderPlus class="w-4 h-4 text-violet-500" />
        <div class="flex-1 text-sm text-slate-700 font-medium">{{ event.name }}</div>

        <!-- 切换模式按钮 -->
        <button
            @click.stop="$emit('toggle-mode', event.id)"
            :class="[
          'flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium transition-all',
          groupMode === 'sequence' ? 'bg-indigo-100 text-indigo-700' : 'bg-amber-100 text-amber-700'
        ]"
        >
          <component :is="groupMode === 'sequence' ? LuListOrdered : LuShuffle" class="w-3 h-3" />
          <span>{{ groupMode === 'sequence' ? '序列' : '同步' }}</span>
        </button>

        <!-- 操作按钮 -->
        <div class="flex gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
          <button @click.stop="$emit('add-event', event.id)" class="p-1.5 hover:bg-white/80 rounded" title="添加事件">
            <LuPlus class="w-3.5 h-3.5" />
          </button>
          <button @click.stop="$emit('add-group', event.id)" class="p-1.5 hover:bg-white/80 rounded" title="添加子分组">
            <LuFolderPlus class="w-3.5 h-3.5" />
          </button>
          <button @click.stop="$emit('edit', event)" class="p-1.5 hover:bg-white/80 rounded" title="编辑">
            <LuPenSquare class="w-3.5 h-3.5" />
          </button>
          <button @click.stop="$emit('delete', event.id)" class="p-1.5 hover:bg-rose-100 rounded" title="删除">
            <LuTrash2 class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </template>

    <!-- 普通事件节点 -->
    <template v-else>
      <div
          :class="[
        'group relative px-3 py-2.5 rounded-lg transition-all cursor-pointer mb-2',
        isSelected
        ? 'bg-indigo-50 border border-indigo-200 shadow-sm'
        : 'bg-white border border-slate-200 hover:border-slate-300 hover:shadow-sm'
      ]"
          @click="handleSelect"
      >
        <div class="flex items-start gap-2.5">
          <!-- 拖拽手柄 -->
          <div
              class="cursor-grab active:cursor-grabbing text-slate-400 hover:text-slate-600 mt-0.5"
              draggable="true"
              @dragstart="onDragStart"
              @dragend="onDragEnd"
          >
            <LuGripVertical class="w-4 h-4" />
          </div>

          <!-- 序号 -->
          <div v-if="showSequence && sequenceNumber" class="flex items-center justify-center w-5 h-5 rounded-full bg-indigo-100 text-indigo-700 text-xs font-medium mt-0.5">
            {{ sequenceNumber }}
          </div>

          <!-- 图标 -->
          <div :class="event.type === 'mouse' ? 'bg-sky-100 text-sky-600' : 'bg-purple-100 text-purple-600'" class="p-2 rounded-lg mt-0.5">
            <component :is="event.type === 'mouse' ? LuMouse : LuKeyboard" class="w-4 h-4" />
          </div>

          <!-- 详情 -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1.5">
            <span class="px-2 py-0.5 bg-slate-100 text-slate-700 rounded text-xs font-medium">
              {{ buttonText }}
            </span>
              <span class="text-xs text-slate-500">{{ actionText }}</span>
            </div>

            <div class="flex flex-wrap gap-2 text-xs">
              <div v-if="event.interval !== undefined" class="flex items-center gap-1 text-slate-600">
                <LuClock class="w-3 h-3 text-slate-400" />
                <span>间隔 {{ event.interval }}ms</span>
              </div>
              <div v-if="event.duration !== undefined" class="flex items-center gap-1 text-slate-600">
                <LuTimer class="w-3 h-3 text-slate-400" />
                <span>持续 {{ event.duration }}ms</span>
              </div>
              <div v-if="event.loopCount !== undefined && event.loopCount !== null" class="flex items-center gap-1 text-slate-600">
                <LuRepeat class="w-3 h-3 text-slate-400" />
                <span>循环 {{ event.loopCount }}次</span>
              </div>
              <div v-if="event.coordinates" class="flex items-center gap-1 text-slate-600">
                <LuMapPin class="w-3 h-3 text-slate-400" />
                <span>({{ event.coordinates.x }}, {{ event.coordinates.y }})</span>
              </div>
              <div v-if="event.scrollDelta !== undefined" class="flex items-center gap-1 text-slate-600">
                <LuMousePointer class="w-3 h-3 text-slate-400" />
                <span>Delta {{ event.scrollDelta }}</span>
              </div>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
            <button @click.stop="$emit('edit', event)" class="p-1.5 hover:bg-slate-100 rounded" title="编辑">
              <LuPenSquare class="w-3.5 h-3.5" />
            </button>
            <button @click.stop="$emit('delete', event.id)" class="p-1.5 hover:bg-rose-100 rounded" title="删除">
              <LuTrash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import {
  LuGripVertical, LuChevronRight, LuChevronDown, LuFolderPlus,
  LuPlus, LuPenSquare, LuTrash2, LuListOrdered, LuShuffle,
  LuMouse, LuKeyboard, LuClock, LuTimer, LuRepeat, LuMapPin, LuMousePointer
} from 'vue-icons-plus/lu';
import type { EventTreeNode } from '@/types';

const props = defineProps<{
  event: EventTreeNode;
  index: number;
  level: number;
  parentId?: string;
  sequenceNumber?: number;
  isExpanded: boolean;
  isSelected: boolean;
  showSequence?: boolean;
}>();

const emit = defineEmits<{
  (e: 'toggle-expand', id: string): void;
  (e: 'select', id: string): void;
  (e: 'edit', event: EventTreeNode): void;
  (e: 'delete', id: string): void;
  (e: 'add-event', parentId?: string): void;
  (e: 'add-group', parentId?: string): void;
  (e: 'toggle-mode', id: string): void;
  (e: 'move', dragId: string, hoverId: string, dragParentId?: string, hoverParentId?: string): void;
}>();

const isDragOver = ref(false);

const groupMode = computed(() =>
    props.event.isGroup ? props.event.executionMode : undefined
);

const buttonText = computed(() => {
  const ev = props.event;
  if (ev.isGroup) return;
  if (ev.type === 'mouse') {
    if (ev.actionType === 'scroll') return '滚轮';
    return ev.mouseButton === 'left' ? '左键' : ev.mouseButton === 'middle' ? '中键' : '右键';
  }
  return ev.keyboardKey || '未设置';
});

const actionText = computed(() => {
  const ev = props.event;
  if (ev.isGroup) return;
  if (ev.actionType === 'click') return '点击';
  if (ev.actionType === 'longPress') return ev.isContinuous ? '持续长按' : '长按';
  return '滚动';
});

// 拖拽开始：存储拖拽数据
function onDragStart(e: DragEvent) {
  if (!e.dataTransfer) return;
  e.dataTransfer.effectAllowed = 'move';
  e.dataTransfer.setData('text/plain', JSON.stringify({
    id: props.event.id,
    parentId: props.parentId,
  }));
}

function onDragEnd() {
  isDragOver.value = false;
}

function onDragEnter(e: DragEvent) {
  if (!e.dataTransfer) return;
  e.preventDefault();
  isDragOver.value = true;
}

function onDrop(e: DragEvent) {
  isDragOver.value = false;
  const dragData = e.dataTransfer?.getData('text/plain');
  if (!dragData) return;

  try {
    const { id: dragId, parentId: dragParentId } = JSON.parse(dragData);
    // 不能拖到自己身上
    if (dragId === props.event.id) return;
    emit('move', dragId, props.event.id, dragParentId, props.parentId);
  } catch (err) {
    console.error('拖拽数据解析失败', err);
  }
}

function handleSelect() {
  emit('select', props.event.id);
}
</script>