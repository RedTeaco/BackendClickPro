<template>
  <div class="flex flex-col h-full bg-slate-50/30">
    <!-- 默认分组头部（对应根组） -->
    <div class="px-4 py-3 border-b border-slate-200 bg-white">
      <div
          class="group flex items-center gap-2 px-3 py-2.5 rounded-lg transition-all cursor-pointer
          bg-linear-to-br from-indigo-50 to-indigo-100/50 border border-indigo-200 hover:shadow-sm"
          @click="toggleExpand(rootGroup.id)"
      >
        <button class="p-0.5 text-indigo-500">
          <component :is="isExpanded ? LuChevronDown : LuChevronRight" class="w-4 h-4" />
        </button>
        <LuFolderPlus class="w-4 h-4 text-indigo-600" />
        <div class="flex-1 text-sm text-indigo-900 font-medium">{{ rootGroup.name }}</div>

        <!-- 切换根组模式 -->
        <button
            @click.stop="store.toggleGroupMode(rootGroup.id)"
            :class="[
            'base-s-button border',
            rootGroup.executionMode === 'sequence'
            ? 'bg-indigo-50 border-indigo-200 text-indigo-600 hover:bg-indigo-100'
            : 'bg-amber-50 border-amber-200 text-amber-600 hover:bg-amber-100'
          ]"
        >
          <component :is="rootGroup.executionMode === 'sequence' ? LuListOrdered : LuShuffle" class="w-3 h-3" />
          <span>{{ rootGroup.executionMode === 'sequence' ? '序列' : '同步' }}</span>
        </button>

        <!-- 添加按钮 -->
        <div class="flex gap-1">
          <button @click.stop="emitAdd(true)" class="base-s-button bg-violet-50 text-violet-600 border border-violet-200 hover:bg-violet-100">
            <LuFolderPlus class="w-3.5 h-3.5 inline mr-1" />分组
          </button>
          <button @click.stop="emitAdd(false)" class="base-s-button bg-indigo-50 text-violet-600 border border-violet-200 hover:bg-indigo-100">
            <LuPlus class="w-3.5 h-3.5 inline mr-1" />事件
          </button>
        </div>
      </div>
    </div>

    <!-- 事件树内容 -->
    <div class="flex-1 overflow-auto p-4">
      <template v-if="isExpanded">
        <div v-if="!rootGroup.children?.length" class="text-center text-slate-400 py-12 text-sm">
          暂无事件，点击上方按钮添加
        </div>
        <EventGroup
            v-else
            :events="rootGroup.children"
            :level="0"
            :parent-mode="rootGroup.executionMode"
            @edit="handleEdit"
            @delete="handleDelete"
            @add="handleAdd"
            @toggle-mode="(id) => store.toggleGroupMode(id)"
            @move="handleMove"
        />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, provide, computed } from 'vue';
import { LuChevronRight, LuChevronDown, LuPlus, LuFolderPlus, LuListOrdered, LuShuffle } from 'vue-icons-plus/lu';
import { useEventStore } from '@/stores/app';
import EventGroup from './EventGroup.vue';
import type { EventTreeNode,GroupTreeNode } from '@/types';

const props = defineProps<{
  rootGroup: GroupTreeNode;
}>();

const emit = defineEmits<{
  (e: 'add', isGroup: boolean, parentId?: string): void;
  (e: 'edit', event: EventTreeNode): void;
}>();

const store = useEventStore();

// 局部展开状态（使用 Set）
const expandedGroups = ref<Set<string>>(new Set([props.rootGroup.id]));
const selectedEventId = ref<string | null>(null);

// 向下层组件提供状态
provide('expandedGroups', expandedGroups.value);
provide('selectedEventId', selectedEventId);

const isExpanded = computed(() => expandedGroups.value.has(props.rootGroup.id));

function toggleExpand(id: string) {
  if (expandedGroups.value.has(id)) {
    expandedGroups.value.delete(id);
  } else {
    expandedGroups.value.add(id);
  }
  // 触发响应更新
  expandedGroups.value = new Set(expandedGroups.value);
}

function emitAdd(isGroup: boolean, parentId?: string) {
  emit('add', isGroup, parentId);
}

function handleEdit(event: EventTreeNode) {
  emit('edit', event);
}

function handleDelete(id: string) {
  store.deleteEvent(id);
}

function handleAdd(isGroup: boolean, parentId?: string) {
  emit('add', isGroup, parentId);
}

function handleMove(dragId: string, hoverId: string, dragParentId?: string, hoverParentId?: string) {
  store.moveEvent(dragId, hoverId, dragParentId, hoverParentId);
}
</script>