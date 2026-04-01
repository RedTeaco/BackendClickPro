<script setup lang="ts">
import {watch} from "vue";
import {sWindow} from "../types/types.ts";

interface Props {
  visible: boolean;
  title?: string;
  options: sWindow[];
}
const props = withDefaults(defineProps<Props>(), {
  title: '窗口列表',
  options: () => []
});

const emit = defineEmits<{
(e: 'update:visible', value: boolean): void;
(e: 'select', item:sWindow): void;
}>();

watch(
    () => props.visible,
    (newVal) => {
      if (newVal) {
        document.body.style.overflow = 'hidden';
      } else {
        document.body.style.overflow = '';
      }
    }
);

const close = () => {
  emit('update:visible', false);
};

const selectOption = (item: sWindow) => {
  emit('select', item);
  close();
};
</script>

<template>
  <teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="close">
      <div class="modal-container">
        <div class="modal-header">
          <h3>{{title}}</h3>
          <button class="close-btn" @click="close">&times;</button>
        </div>
        <div class="modal-body">
          <div
          v-for="item in options"
          :key="item.hwnd"
          class="option-item"
          @click="selectOption(item)">{{item.title}}</div>
        </div>
      </div>
    </div>
  </teleport>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0,0,0,0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-container {
  background: white;
  border-radius: 8px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  overflow: hidden;
}

.modal-header{
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #eee;
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
}

.close-btn {
  border: none;
  background: none;
  font-size: 24px;
  cursor: pointer;
  line-height: 1;
}

.modal-body {
  padding: 0;
  max-height: 300px;
  overflow-y: auto;
}

.option-item {
  padding: 12px 16px;
  cursor: pointer;
  border-bottom: 1px solid #eee;
  transition: background-color 0.2s;
}

.option-item:hover {
  background-color: #f5f7fa;
}
</style>