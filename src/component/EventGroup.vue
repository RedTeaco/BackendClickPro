<template>
  <div :class="level > 0 ? 'ml-6 mt-1 pl-3 border-l-2 border-slate-200' : ''">
    <template v-for="(event, index) in events" :key="event.id">
      <DraggableEvent
          :event="event"
          :index="index"
          :level="level"
          :parent-id="parentId"
          :sequence-number="showSequence ? index + 1 : undefined"
          :is-expanded="expandedGroups.has(event.id)"
          :is-selected="selectedEventId === event.id"
          :show-sequence="showSequence"
          @toggle-expand="toggleExpand"
          @select="setSelected"
          @edit="emitEdit"
          @delete="emitDelete"
          @add-event="(pid) => emitAdd(false, pid)"
          @add-group="(pid) => emitAdd(true, pid)"
          @toggle-mode="(id) => emit('toggle-mode',id)"
          @move="handleMove"
      />

      <!-- 递归渲染子分组（当展开且有子项时） -->
      <EventGroup
          v-if="event.isGroup && expandedGroups.has(event.id) && event.children?.length"
          :events="event.children"
          :parent-id="event.id"
          :level="level + 1"
          :parent-mode="event.executionMode || 'sequence'"
          @edit="emitEdit"
          @delete="emitDelete"
          @add="emitAdd"
          @toggle-mode="(id) => emit('toggle-mode', id)"
          @move="handleMove"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue';
import DraggableEvent from './DraggableEvent.vue';
import type { EventTreeNode } from '@/types';

const props = defineProps<{
  events: EventTreeNode[];
  parentId?: string;
  level: number;
  parentMode?: 'sequence' | 'sync';
}>();

// 从父组件注入的状态（EventList 管理）
const expandedGroups = inject<Set<string>>('expandedGroups')!;
const selectedEventId = inject<string | null>('selectedEventId')!;

const emit = defineEmits<{
  (e: 'edit', event: EventTreeNode): void;
  (e: 'delete', id: string): void;
  (e: 'add', isGroup: boolean, parentId?: string): void;
  (e: 'toggle-mode', id: string): void;
  (e: 'move', dragId: string, hoverId: string, dragParentId?: string, hoverParentId?: string): void;
}>();

const showSequence = computed(() => props.parentMode === 'sequence');

function toggleExpand(id: string) {
  if (expandedGroups.has(id)) {
    expandedGroups.delete(id);
  } else {
    expandedGroups.add(id);
  }
}

function setSelected(id: string) {
  // 通过 inject 的引用直接修改
  // 注意：此为临时方案，实际应在父组件使用 provide 响应式状态
  (selectedEventId as any).value = id;
}

function emitEdit(event: EventTreeNode) {
  emit('edit', event);
}

function emitDelete(id: string) {
  emit('delete', id);
}

function emitAdd(isGroup: boolean, parentId?: string) {
  emit('add', isGroup, parentId);
}

function handleMove(dragId: string, hoverId: string, dragParentId?: string, hoverParentId?: string) {
  emit('move', dragId, hoverId, dragParentId, hoverParentId);
}
</script>