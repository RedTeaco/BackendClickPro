import { invoke } from '@tauri-apps/api/core';
import type { WindowInfo, ExecutionNode } from '@/types';

export const getWindows = () => invoke<WindowInfo[]>('get_windows');
export const executePlan = (plan: ExecutionNode) => invoke('execute_plan', { plan });
export const stopExecution = () => invoke('stop_execution');
// 持久化（保存/加载所有根组）
export const saveRootGroups = (groups: any[]) => invoke('save_root_groups', { groups });
export const loadRootGroups = () => invoke<any[]>('load_root_groups');