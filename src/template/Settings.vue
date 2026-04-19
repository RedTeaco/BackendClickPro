<template>
  <div class="p-6 h-full flex flex-col">
    <h2 class="text-xl font-bold mb-4">设置</h2>
    <div class="flex-1 space-y-6">
      <div class="setting-group">
        <h3 class="text-lg font-semibold mb-2">快捷键</h3>
        <p class="text-sm text-gray-500">feature</p>
      </div>
      <div class="setting-group">
        <h3 class="text-lg font-semibold mb-2">主题</h3>
        <p class="text-sm text-gray-500">feature</p>
      </div>
      <div class="setting-group">
        <h3 class="text-lg font-semibold mb-2">数据管理</h3>
        <button class="secondary-button-outlined" @click="clearAllData">清除所有保存数据</button>
      </div>
    </div>
    <div class="flex justify-end pt-4 border-t">
      <button class="primary-button" @click="router.push('/')">返回</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import {useRouter} from 'vue-router'
import {useAppStore} from "../stores/app.ts";
import {invoke} from '@tauri-apps/api/core'

const router = useRouter()
const store = useAppStore()

const clearAllData = async () => {
  if (confirm('确定要清除所有保存的事件和设置吗?')) {
    try {
      await invoke('clear_all_data')
      store.actionItems = []
      store.addLog('已清除所有本地数据')
    } catch (err) {
      store.addLog(`清除失败: ${err}`)
    }
  }
}
</script>
<style scoped>

</style>