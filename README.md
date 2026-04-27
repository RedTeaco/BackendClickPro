# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## // 逻辑梳理： 
前端表单：
- 事件：鼠标事件/键盘事件
- 动作：点击/长按
- - 点击 => {
间隔时间: ms
循环次数: 次 <=0 或不填则表示一直循环
}

- - 长按 => { 
是否持续: boolean 如果为true，则其他参数无需填写
持续时间: ms
间隔时间: ms 给默认值
循环次数: 次 <=0 或不填则表示一直循环
}

- key/button: 键盘/鼠标按键
- 鼠标事件=> x/y: 鼠标点击位置 //TODO 用户手动输入还是鼠标位置捕捉？暂时采用手动输入方式
默认值为0,0
- 鼠标事件=> delta: 滚轮滑动(前端额外设置button选项为滑轮,此时动作改为上划、下划)
- - 滑轮时,动作为持续时间、间隔时间、循环次数(与长按一致) // 对于滚轮，使用delta，为单次函数，应该如何编写后端函数?

### 分组模式
#### 模式1:同步模式
在该模式下-列表中的所有单个事件均同时执行
但是单个事件中可能有多个动作，例如点击循环3次的同时进行下一个事件


#### 模式2:序列模式
在该模式下-列表中的所有单个事件按顺序执行
但是单个事件中可能有多个动作，例如点击循环3次后进行下一个事件

# TODO
- [ ] 鼠标坐标捕捉
- [ ] 热键(快捷键)设置
- [ ] 管理员启动
- [ ] 启动加载问题pinia
- [ ] 清理代码，发布beta-1.0