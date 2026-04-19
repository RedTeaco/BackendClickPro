<template>
  <div
      ref="wrapperRef"
      class="overflow-hidden whitespace-nowrap"
      :class="wrapperClass"
  >
    <div
        ref="textRef"
        class="flex gap-1 whitespace-nowrap marquee-text"
        :class="{ 'animate-marquee-alternate': shouldAnimate }"
        :style="animationStyle"
    >
      <slot></slot>
      {{ text }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'

const props = withDefaults(defineProps<{
  text: string
  speed?: number
  wrapperClass?: string
}>(), {
  speed: 30,
  wrapperClass: 'rounded px-2 py-1'
})

const wrapperRef = ref<HTMLElement>()
const textRef = ref<HTMLElement>()
const shouldAnimate = ref(false)
const animationStyle = ref<Record<string, string>>({})

const checkOverflow = () => {
  if (!wrapperRef.value || !textRef.value) return
  const wrapperWidth = wrapperRef.value.clientWidth
  const textWidth = textRef.value.scrollWidth

  if (textWidth > wrapperWidth) {
    shouldAnimate.value = true
    const distance = textWidth - wrapperWidth
    const duration = distance / props.speed
    animationStyle.value = {
      '--marquee-distance': `-${distance}px`,
      '--marquee-duration': `${duration}s`
    }
  } else {
    shouldAnimate.value = false
    animationStyle.value = {}
  }
}

const handleResize = () => nextTick(checkOverflow)

onMounted(() => {
  nextTick(checkOverflow)
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})

watch(() => props.text, () => nextTick(checkOverflow))
</script>

<style scoped>
/* 声明变量默认值，让 IDE 识别 */
.marquee-text {
  --marquee-distance: 0px;
  --marquee-duration: 0s;
}

@keyframes marquee-alternate {
  0% {
    transform: translateX(0);
  }
  100% {
    transform: translateX(var(--marquee-distance));
  }
}

.animate-marquee-alternate {
  animation: marquee-alternate var(--marquee-duration) linear infinite alternate;
}
</style>