<template>
  <div class="add-event">
    <div class="panel-header">
      <h3>添加新动作</h3>
      <span class="step-count">请填写事件信息</span>
    </div>

    <div class="form-container">
      <div class="form-group">
        <label>事件类型</label>
        <select v-model="form.type">
          <option value="mouse">鼠标事件</option>
          <option value="keyboard">键盘事件</option>
        </select>
      </div>

      <div class="form-group">
        <label>模式</label>
        <select v-model="form.mode">
          <option value="click">点击</option>
          <option value="press">长按</option>
        </select>
      </div>

      <div v-if="form.type === 'mouse'" class="form-group">
        <label>鼠标按键</label>
        <select v-model="form.button">
          <option value="left">左键</option>
          <option value="right">右键</option>
          <option value="middle">中键</option>
        </select>
      </div>

      <div v-if="form.type === 'keyboard'" class="form-group">
        <label>键盘按键</label>
        <input readonly @keydown="handleKeyDown" :placeholder=key_placeholder  />
      </div>

      <div v-if="form.mode === 'press'" class="form-group">
        <label>持续时间 (毫秒)</label>
        <input type="number" v-model.number="form.duration" placeholder="默认600ms" />
      </div>
    </div>

    <div class="action-buttons">
      <button class="ctrl-btn cancel" @click="emit('cancel')">取消</button>
      <button class="ctrl-btn save" @click="saveEvent">保存</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {reactive, ref} from 'vue'
import {km_event} from "../types/types.ts";
import {invoke} from "@tauri-apps/api/core";

let key_placeholder = ref<string>('例如：Enter, Space, A');

const emit = defineEmits<{
  (e: 'addEvent', event: km_event): void;
  (e: 'cancel'): void;
}>();

const form = reactive<km_event>({
  type: 'mouse',
  mode: 'click',
  duration: undefined,
  key: '',
  button: 'left'
});

const handleKeyDown = (event: KeyboardEvent) => {
  form.key = event.code;
  key_placeholder.value = event.code;
  console.log(`按下了code:${event.code}`);
  console.log(`按下了key:${event.key}`);
  invoke('code_to_vk', {code: event.code}).then((res) => {
    console.log('code_to_vk', res);
  });
}

const saveEvent = () => {
  // 构造最终事件对象
  const newEvent: km_event = {
    type: form.type,
    mode: form.mode,
  };
  if (form.mode === 'press') {
    newEvent.duration = form.duration && form.duration > 0 ? form.duration : 600;
  }
  if (form.type === 'keyboard') {
    newEvent.key = form.key?.trim() || '未命名按键';
  } else {
    newEvent.button = form.button;
  }
  emit('addEvent', newEvent);
};
</script>

<style scoped>
.add-event {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  border-bottom: 2px solid #eef2ff;
  padding-bottom: 0.6rem;
}

.panel-header h3 {
  font-weight: 600;
  font-size: 1.35rem;
  color: #0f172a;
}

.step-count {
  font-size: 0.8rem;
  background: #eef2ff;
  padding: 4px 10px;
  border-radius: 30px;
  color: #2c3e66;
}

.form-container {
  flex: 1;
  background: #fafcff;
  border-radius: 20px;
  border: 1px solid #ecf3fa;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.form-group label {
  font-size: 0.85rem;
  font-weight: 500;
  color: #2c3e66;
}

.form-group select,
.form-group input {
  padding: 8px 12px;
  border-radius: 30px;
  border: 1px solid #e2e8f0;
  background: white;
  font-size: 0.9rem;
  outline: none;
  transition: 0.2s;
}

.form-group select:focus,
.form-group input:focus {
  border-color: #2c3e66;
  box-shadow: 0 0 0 2px rgba(44, 62, 102, 0.2);
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.ctrl-btn {
  background: white;
  border: 1px solid #dee5ed;
  padding: 8px 24px;
  border-radius: 40px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  color: #2c3e66;
}

.ctrl-btn:hover {
  background: #eef2ff;
  border-color: #b9c8e5;
  transform: scale(0.97);
}

.ctrl-btn.save {
  background: #2c3e66;
  border-color: #1e2f41;
  color: white;
}

.ctrl-btn.save:hover {
  background: #1e2f41;
}
</style>