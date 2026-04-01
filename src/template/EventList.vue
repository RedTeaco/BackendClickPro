<template>
  <div class="event-list">
    <div class="panel-header">
      <h3>执行步骤列表</h3>
      <span class="step-count">{{ actionItems.length }} 个步骤</span>
    </div>

    <div class="action-list-container">
      <ul class="action-list">
        <li
            v-for="(item, index) in actionItems"
            :key="index"
            class="action-item"
            :class="{ selected: selectedIndex === index }"
            @click="emit('selectItem',index)"
        >
          <span class="item-index">{{ index + 1 }}</span>
          <span class="item-content">{{ formatKmEvent(item) }}</span>
          <div class="item-actions">
            <!-- 上移按钮 -->
            <button
                class="action-icon"
                @click.stop="emit('moveUp', index)"
                :disabled="isRunning || index === 0"
                title="上移"
            >⬆</button>
            <!-- 下移按钮 -->
            <button
                class="action-icon"
                @click.stop="emit('moveDown', index)"
                :disabled="isRunning || index === actionItems.length - 1"
                title="下移"
            >⬇</button>
            <!-- 删除按钮（原 item-delete 样式微调） -->
            <button
                class="action-icon delete"
                @click.stop="emit('deleteItem', index)"
                :disabled="isRunning"
                title="删除此项"
            >✕</button>
          </div>
        </li>
      </ul>
      <div v-if="actionItems.length === 0" class="empty-list">
        <span>📭 暂无步骤，点击上方“添加”按钮创建新动作</span>
      </div>
    </div>

    <!-- 底部仅保留添加按钮，原来的 move-delete-group 已移除 -->
    <div class="control-bar">
      <div class="add-section">
        <button class="ctrl-btn add-btn" @click="emit('switchToAdd')" :disabled="isRunning">
          ➕ 添加
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import {km_event} from "../types/types.ts";
import {formatKmEvent} from "../utils/utils.ts";

// ... 类型定义和 formatKmEvent 函数保持不变（与之前相同）

const props = defineProps<{
  actionItems: km_event[];
  selectedIndex: number | null;
  isRunning: boolean;
}>();

const emit = defineEmits<{
  (e: 'selectItem', index: number): void;
  (e: 'deleteItem', index: number): void;
  (e: 'moveUp', index: number): void;   // 新增
  (e: 'moveDown', index: number): void; // 新增
  (e: 'switchToAdd'): void;
}>();

// ... formatKmEvent 函数不变
</script>

<style scoped>
/* 修改样式以适应新布局 */
.event-list {
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
  height: 100%;
}

/* 其他头部样式不变 */

.action-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  background: white;
  border-radius: 16px;
  transition: all 0.15s;
  cursor: pointer;
  border: 1px solid #eef2ff;
}

.action-item:hover {
  background: #f8fafc;
  border-color: #cbdff2;
}

.action-item.selected {
  background: #eef4ff;
  border-left: 4px solid #2c3e66;
  border-radius: 12px;
}

.item-index {
  font-weight: 600;
  width: 32px;
  color: #5b6e8c;
  font-size: 0.8rem;
}

.item-content {
  flex: 1;
  font-size: 0.9rem;
  font-weight: 500;
  color: #1e293b;
}

.item-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.action-icon {
  background: none;
  border: none;
  font-size: 1rem;
  cursor: pointer;
  color: #94a3b8;
  border-radius: 30px;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.1s;
}

.action-icon:hover:not(:disabled) {
  background: #eef2ff;
  color: #2c3e66;
}

.action-icon.delete:hover:not(:disabled) {
  background: #fee2e2;
  color: #b91c1c;
}

.action-icon:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* 底部控制栏只保留添加按钮，样式微调 */
.control-bar {
  background: #f9fbfd;
  padding: 12px 16px;
  border-radius: 40px;
  margin-top: 4px;
}

.add-section {
  display: flex;
  justify-content: flex-start;
}

.ctrl-btn {
  background: white;
  border: 1px solid #dee5ed;
  padding: 6px 16px;
  border-radius: 40px;
  font-size: 0.8rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  color: #2c3e66;
}

.ctrl-btn:hover:not(:disabled) {
  background: #eef2ff;
  border-color: #b9c8e5;
  transform: scale(0.97);
}

.ctrl-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.add-btn {
  background: #eef2ff;
  border-color: #cddef5;
  font-weight: 600;
}

/* 其余样式（滚动条、空状态等）保持不变 */
</style>