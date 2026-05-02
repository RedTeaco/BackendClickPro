import { invoke } from '@tauri-apps/api/core';
import type {WindowInfo, ExecutionNode, ShortcutConfig} from '@/types';

export const getWindows = () => invoke<WindowInfo[]>('get_windows');
export const executePlan = (plan: ExecutionNode) => invoke('execute_plan', { plan });
export const stopExecution = () => invoke('stop_execution');
// 持久化（保存/加载所有根组）
export const saveRootGroups = (groups: any[], totalLoopCount: number  | null) => invoke('save_root_groups', { groups, totalLoopCount });
export const loadRootGroups = () => invoke<{rootGroups: any[], totalLoopCount: number | null}>('load_root_groups');
// 持久化（保存/加载快捷键）
export const saveShortcuts = (shortcuts: any) => invoke('save_shortcuts', { shortcuts });
export const loadShortcuts = () => invoke<ShortcutConfig>('load_shortcuts');