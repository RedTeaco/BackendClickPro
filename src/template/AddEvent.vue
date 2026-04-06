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
        <label>动作</label>
        <select v-model="form.actionType">
          <option value="click">点击</option>
          <option value="hold">长按</option>
          <option v-if="form.type === 'mouse'" value="scroll">滚轮</option>
        </select>
      </div>

      <div v-if="form.type === 'mouse' && form.actionType !== 'scroll'" class="form-group">
        <label>鼠标按键</label>
        <select v-model="form.button">
          <option value="left">左键</option>
          <option value="right">右键</option>
          <option value="middle">中键</option>
        </select>
      </div>

      <div v-if="form.type === 'keyboard'" class="form-group">
        <label>键盘按键</label>
        <input readonly @keydown="handleKeyDown" :placeholder=keyPlaceholder  />
      </div>

      <!--鼠标坐标 可选-->
      <div v-if="form.type === 'mouse'" class="form-group row-group">
        <label>鼠标坐标(X,Y) 可选</label>
        <div style="display: flex; gap: 8px;">
          <input type="number" v-model.number="form.x" placeholder="X(默认0)" />
          <input type="number" v-model.number="form.y" placeholder="Y(默认0)" />
        </div>
        <small>留空表示使用窗口左上角(0,0)</small>
      </div>

<!--      长按特有-->
      <div v-if="form.actionType === 'hold'" class="form-group">
        <label>持续时间 (毫秒)</label>
        <input type="number" v-model.number="form.duration_ms" placeholder="默认10ms" />
      </div>
<!--      滚轮特有-->
      <div v-if="form.actionType === 'scroll'" class="form-group">
        <label>滚动量(delta)</label>
        <input type="number" v-model.number="form.delta" placeholder="正数向上,负数向下" />
      </div>

      <div class="form-group">
        <label>间隔时间(毫秒)</label>
        <input type="number" v-model.number="form.interval_ms" placeholder="两次循环之间的间隔"/>
      </div>

<!--      循环次数 不填或小于0表示无限-->
      <div class="form-group">
        <label>循环次数</label>
        <input type="number" v-model.number="form.count" placeholder="留空或 ≤0 表示无限循环"/>
        <small>输入正整数为有限次数，否则无限循环</small>
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
import {FormEvent} from "../types/types.ts";

let keyPlaceholder = ref<string>('例如：Enter, Space, A');

const emit = defineEmits<{
  (e: 'addEvent', event: FormEvent): void;
  (e: 'cancel'): void;
}>();

const form = reactive<FormEvent>({
  type: 'mouse',
  actionType: 'click',
  button: 'left',
  x: null,
  y: null,
  key: '',
  duration_ms: undefined, // 按键按下时间
  interval_ms: 100, // 两次循环之间的间隔
  count: null,
  delta: undefined,
});

const handleKeyDown = (event: KeyboardEvent) => {
  form.key = event.code;
  keyPlaceholder.value = event.code;
  event.preventDefault();
}

const saveEvent = () => {
  // 基础校验
  if (form.type === 'keyboard' && !form.key) {
    alert('请输入按键');
    return;
  }
  if (form.actionType === 'hold' && ((!form.duration_ms || form.duration_ms <= 0) && form.count)) { // 非无限循环时没填持续时间
    alert('长按处需要有效的持续时间(毫秒)')
    return;
  }
  if (form.actionType === 'scroll' && form.delta === undefined) {
    alert('需要填写滚轮滚动量');
    return;
  }

  // 处理count: 空字符串或 <=0 转为null
  let finalCount = form.count;
  if (finalCount !== null && (finalCount <= 0 || isNaN(finalCount))) {
    finalCount = null;
  }
  const newEvent: FormEvent = {
    ...form,
    count: finalCount,
    interval_ms: form.interval_ms || 0,
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