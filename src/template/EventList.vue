<template>
  <div class="flex flex-col gap-2 h-full p-4">
    <div class="flex flex-nowrap gap-4 items-baseline justify-between">
      <h3 class="text-xl">执行步骤列表</h3>
      <span class="text-md">{{ store.actionItems.length }} 个步骤</span>
    </div>

    <div class="flex-1 min-h-0 overflow-y-auto">
      <ul class="flex flex-col gap-2">
        <li
            v-for="(item, index) in store.actionItems"
            :key="index"
            class="event-card-item"
            :class="{ selected: store.selectedIndex === index,
            'bg-brand-neutral/10 border-l-4 border-brand-primary':store.selectedIndex === index }"
            @click="store.selectItem(index)"
        >
          <span class="event-index">{{ index + 1 }}</span>
          <div class="flex flex-1 gap-4 items-center">
            <span v-if="item.type === 'mouse'" class="text-brand-primary"><LuMouse /></span>
            <span v-else class="text-brand-primary"><LuKeyboard /></span>
            <div class="flex flex-col flex-1 gap-1 text-black">
              <span>{{getEventType(item)}}</span>
              <div class="grid grid-cols-6 text-brand-neutral text-[0.7rem] items-center">
                <div class="col-span-1">
                  <div v-if="item.type === 'mouse'" class="detailed-content">
                    <BiTargetLock size="16"/>
                    <span class="translate-y-0.5">{{item.x ?? 0}}, {{item.y ?? 0}}</span>
                  </div>
                </div>
                <div class="col-span-2">
                  <div v-if="item.duration_ms" class="detailed-content">
                    <LuTimer size="16"/>
                    <span class="translate-y-0.5">持续 {{formatDuration(item.duration_ms)}}</span>
                  </div>
                  <div v-else class="detailed-content">
                    <LuMousePointerClick size="16"/>
                    <span class="translate-y-0.5">点击</span>
                  </div>
                </div>
                <div class="detailed-content col-span-2">
                  <LuHourglass size="16"/>
                  <span class="translate-y-0.5">间隔 {{formatDuration(item.interval_ms)}}</span>
                </div>
                <div class="detailed-content col-span-1">
                  <LuRefreshCw size="16"/>
                  <span v-if="item.count" class="translate-y-0.5">{{item.count}}次</span>
                  <span v-else class="translate-y-0.5">无限</span>
                </div>
              </div>
            </div>
          </div>
          <div class="flex items-center gap-1">
            <button
                :data-disabled="store.isRunning ? 'true' : undefined"
                class="action-icon-button hover:text-brand-primary data-disabled:hover:bg-transparent data-disabled:hover:text-current
data-disabled:opacity-40 data-disabled:cursor-not-allowed"
                title="修改">
              <LuPencil size="16"/>
            </button>
            <!-- 上移按钮 -->
            <button
                :data-disabled="(store.isRunning || index === 0) ? 'true' : undefined"
                class="action-icon-button
hover:text-brand-primary data-disabled:hover:bg-transparent data-disabled:hover:text-current data-disabled:opacity-40 data-disabled:cursor-not-allowed"
                @click.stop="store.moveUp(index)"
                title="上移"
            >
              <LuChevronUp size="20" />
            </button>
            <!-- 下移按钮 -->
            <button
                :data-disabled="(store.isRunning || index === store.actionItems.length - 1)?'true' : undefined"
                class="action-icon-button hover:text-brand-primary data-disabled:hover:bg-transparent data-disabled:hover:text-current data-disabled:opacity-40 data-disabled:cursor-not-allowed"
                @click.stop="store.moveDown(index)"
                title="下移"
            ><LuChevronDown size="20"/></button>
            <button
                :data-disabled="store.isRunning ? 'true' : undefined"
                class="action-icon-button hover:text-brand-stop
                hover:drop-shadow-xl hover:drop-shadow-brand-stop
data-disabled:hover:bg-transparent data-disabled:hover:text-current data-disabled:opacity-40 data-disabled:cursor-not-allowed"
                @click.stop="store.deleteActionItem(index)"
                title="删除此项"
            >
              <LuX size="20"/>
            </button>
          </div>
        </li>
      </ul>
      <div v-if="store.actionItems.length === 0" class="empty-list">
        <span>📭 暂无步骤，点击上方“添加”按钮创建新动作</span>
      </div>
    </div>

    <div class="w-full shrink-0">
        <button class="w-full justify-center border-dashed primary-button-outlined h-12"
                @click="router.push('/add')"
                :disabled="store.isRunning">
          <AiOutlinePlus />
        </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {AiOutlinePlus} from 'vue-icons-plus/ai';
import {BiTargetLock} from 'vue-icons-plus/bi';
import {LuKeyboard, LuMouse, LuTimer, LuHourglass, LuMousePointerClick, LuRefreshCw, LuPencil, LuChevronDown, LuChevronUp, LuX} from 'vue-icons-plus/lu';

import {useRouter} from 'vue-router';
import {useAppStore} from "../stores/app.ts";
import {formatDuration, getEventType} from "../utils/utils.ts";
// ... 类型定义和 formatKmEvent 函数保持不变（与之前相同）
const router = useRouter();
const store = useAppStore();
</script>