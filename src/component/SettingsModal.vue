<script setup lang="ts">
import {LuX} from 'vue-icons-plus/lu';
defineEmits<{ (e: 'close'): void }>();
</script>

<template>
  <div class="fixed inset-0 bg-slate-900/20 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="bg-white rounded-xl shadow-lg w-125 max-h-[80vh] overflow-auto border border-slate-200">
      <div class="flex items-center justify-between px-5 py-4 border-b border-slate-100">
        <h3 class="text-base text-slate-700">设置</h3>
        <button @click="$emit('close')" class="p-1.5 hover:bg-slate-100 rounded-lg">
          <LuX size="16" />
        </button>
        <button @click="$emit('close')" class="px-4 py-2 bg-indigo-500 text-white rounded-lg text-sm">确定</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>