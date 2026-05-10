<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="close">
    <div class="bg-white rounded-lg shadow-xl w-96 max-w-full p-5">
      <h2 class="text-lg font-semibold mb-3">管理方案</h2>
      <div class="space-y-2 max-h-96 overflow-y-auto">
        <div v-for="scheme in store.rootGroups" :key="scheme.id"
             class="flex items-center gap-2 p-2 hover:bg-gray-50 rounded group"
            :class="{ 'bg-indigo-50': scheme.id === store.activeRootId}">
          <span class="flex-1 cursor-pointer" @click="switchToScheme(scheme.id)">{{scheme.name}}</span>
          <button @click="renameScheme(scheme)" class="text-gray-500 hover:text-indigo-600" title="重命名">
            <LuPencil size="16" />
          </button>
          <button @click="deleteScheme(scheme)" class="text-gray-500 hover:text-brand-stop" title="删除"
          :disabled="store.rootGroups.length <= 1">
            <LuTrash2 size="16" />
          </button>
        </div>
      </div>
      <div class="mt-4 flex justify-between">
        <button @click="createNewScheme" class="base-button bg-indigo-50 text-indigo-600">
          <LuPlus size="16" />
          新建方案
        </button>
        <button @click="close" class="base-button bg-slate-100">关闭
        </button>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import {useEventStore} from "@/stores/app.ts";
import {LuPencil, LuTrash2, LuPlus} from "vue-icons-plus/lu";

const emit = defineEmits(['close'])
const store = useEventStore();

function close() {
  emit('close');
}

function switchToScheme(id: string) {
  store.activeRootId = id;
}

function renameScheme(scheme: any) {
  const newName = prompt('重命名方案', scheme.name);
  if (newName && newName.trim()) {
    store.renameRootGroup(scheme.id, newName.trim());
  }
}

function deleteScheme(scheme: any) {
  if (store.rootGroups.length <= 1) {
    alert('至少保留一个方案');
    return;
  }
  if (confirm(`确定删除方案 ${scheme.name} 吗？`)) {
    store.deleteRootGroup(scheme.id);
  }
}

function createNewScheme() {
  let name = prompt('新建方案','新方案');
  if (name && name.trim()) {
    store.addRootGroup(name.trim());
  } else if (name !== null) {
    store.addRootGroup('未命名方案');
  }
}
</script>
<style scoped>

</style>