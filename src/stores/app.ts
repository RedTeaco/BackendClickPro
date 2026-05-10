import { defineStore } from 'pinia';
import {ref, computed, watch} from 'vue';
import { v4 as uuidv4 } from 'uuid';
import { getWindows, executePlan, stopExecution, saveRootGroups, loadRootGroups } from "@/services/tauri";
import type {
    WindowInfo,
    EventTreeNode,
    ExecutionNode,
    InputEvent,
    EventAction,
    GroupTreeNode,
    EventItemNode
} from '@/types';
import {listen} from "@tauri-apps/api/event";

function normalizeGroup(g:any): GroupTreeNode {
    return {
        id: (g.id as string) || uuidv4(),
        name: g.name || '未命名方案',
        isGroup: true,
        executionMode: g.executionMode === 'sync' ? 'sync' : 'sequence',
        children: Array.isArray(g.children) ? g.children.map(normalizeNode): [],
    };
}

function normalizeNode(node: any): EventTreeNode {
    if (node.isGroup) return normalizeGroup(node);
    return {
        id: node.id || uuidv4(),
        name: node.name || '未命名事件',
        type: node.type || 'mouse',
        actionType: node.actionType || 'click',
        mouseButton: node.mouseButton || 'left',
        keyboardKey: node.keyboardKey || '',
        isContinuous: node.isContinuous || false,
        interval: node.interval ?? 1,
        duration: node.duration ?? 1,
        loopCount: node.loopCount ?? null,
        coordinates: node.coordinates || {x: 0, y: 0},
        scrollDelta: node.scrollDelta ?? 0,
    };
}


export const useEventStore = defineStore('event', () => {
    // ---------- 状态 ----------
    const availableWindows = ref<WindowInfo[]>([]);
    const selectedWindow = ref<WindowInfo | null>(null);
    const isRunning = ref(false);
    const loading = ref(false);

    // 根组列表（每个根组是一个顶级分组）
    const rootGroups = ref<EventTreeNode[]>([
        {
            id: 'default-root',
            name: '默认方案',
            isGroup: true,
            executionMode: 'sequence',
            children: [],
        },
    ]);
    const activeRootId = ref<string>(rootGroups.value[0].id);
    const activeRoot = computed<GroupTreeNode | undefined>(
        () => rootGroups.value.find(g => g.id === activeRootId.value) as GroupTreeNode | undefined
    );
    // 展开/折叠状态（用于UI）
    const expandedGroups = ref<Set<string>>(new Set(['default-root']));
    const selectedEventId = ref<string | null>(null);

    const totalLoopCount = ref<number | null>(null);

    // ---------- 窗口操作 ----------
    async function fetchWindows() {
        availableWindows.value = await getWindows();
    }

    function selectWindow(hwnd: number) {
        const win = availableWindows.value.find(w => w.hwnd === hwnd);
        if (win) selectedWindow.value = win;
    }

    // ---------- 根组管理 ----------
    function addRootGroup(name: string) {
        const newGroup: GroupTreeNode = {
            id: uuidv4(),
            name,
            isGroup: true,
            executionMode: 'sequence',
            children: [], // 必须初始化为空数组
        };
        rootGroups.value.push(newGroup);
        activeRootId.value = newGroup.id;
        expandedGroups.value = new Set([...expandedGroups.value, newGroup.id]);
    }

    function deleteRootGroup(id: string) {
        if (rootGroups.value.length <= 1) return;
        const idx = rootGroups.value.findIndex(g => g.id === id);
        if (idx !== -1) {
            rootGroups.value.splice(idx, 1);
            if (activeRootId.value === id) {
                activeRootId.value = rootGroups.value[0].id;
            }
        }
    }

    function renameRootGroup(id: string, name:string) {
        const group = rootGroups.value.find(g => g.id === id);
        if (group) group.name = name;
    }

    // ---------- 树操作（增删改移）针对 activeRoot ----------
    // 添加分组节点
    function addGroup(data: Partial<GroupTreeNode>, parentId?: string) {
        const root = activeRoot.value;
        if (!root) return;
        const newGroup: GroupTreeNode = {
            id: uuidv4(),
            name: data.name || '未命名分组',
            isGroup: true,
            executionMode: data.executionMode || 'sequence',
            children: data.children || [], // 确保 children 必填
        };
        insertNode(newGroup, parentId);
    }

    // 添加事件节点
    function addEventItem(data: Partial<EventItemNode>, parentId?: string) {
        const root = activeRoot.value;
        if (!root) return;
        const newEvent: EventItemNode = {
            id: uuidv4(),
            name: data.name || '未命名事件',
            type: data.type || 'mouse',
            actionType: data.actionType || 'click',
            mouseButton: data.mouseButton || 'left',
            keyboardKey: data.keyboardKey || '',
            isContinuous: data.isContinuous || false,
            interval: data.interval ?? 1,
            duration: data.duration ?? 1,
            loopCount: data.loopCount ?? null,
            coordinates: data.coordinates,
            scrollDelta: data.scrollDelta,
        };
        insertNode(newEvent, parentId);
    }

    function updateEvent(id: string, data: Partial<EventTreeNode>) {
        const root = activeRoot.value;
        if (!root) return;

        const updateNode = (nodes: EventTreeNode[]): boolean => {
            for (const node of nodes) {
                if (node.id === id) {
                    if (node.isGroup) {
                        // 分组分支：只允许更新 name 和 executionMode
                        if (data.name !== undefined) node.name = data.name;
                        const groupData = data as Partial<GroupTreeNode>;
                        if (groupData.executionMode) {
                            node.executionMode = groupData.executionMode;
                        }
                    } else {
                        // 事件分支：直接合并所有允许更新的字段
                        Object.assign(node, data);
                    }
                    return true;
                }
                if (node.children && updateNode(node.children)) return true;
            }
            return false;
        };

        updateNode(root.children); // root.children 已经是 EventTreeNode[]
    }

    // 共用插入逻辑
    // 插入节点到指定父节点下
    function insertNode(node: EventTreeNode, parentId?: string) {
        const root = activeRoot.value;
        if (!root) return;

        const rootChildren = root.children;
        if (!parentId) {
            rootChildren.push(node);
        } else {
            const addToParent = (nodes: EventTreeNode[]): boolean => {
                for (const n of nodes) {
                    if (n.id === parentId) {
                        if (!n.children) n.children = [];
                        n.children.push(node);
                        return true;
                    }
                    if (n.children && addToParent(n.children)) return true;
                }
                return false;
            };
            addToParent(rootChildren);
        }
    }

    function deleteEvent(id: string) {
        const root = activeRoot.value;
        if (!root) return;
        const deleteFrom = (nodes: EventTreeNode[]): EventTreeNode[] => {
            return nodes.filter(node => {
                if (node.id === id) return false;
                if (node.children) node.children = deleteFrom(node.children);
                return true;
            });
        };
        root.children = deleteFrom(root.children);
    }

    function moveEvent(dragId: string, hoverId: string, dragParentId?: string, hoverParentId?: string) {
        const root = activeRoot.value;
        if (!root) return;
        if (dragId === hoverId) return;

        let dragItem: EventTreeNode | null = null;
        // 从树中移除 dragId
        const removeFromTree = (nodes: EventTreeNode[]): EventTreeNode[] => {
            return nodes.filter(node => {
                if (node.id === dragId && !dragParentId) {
                    dragItem = node;
                    return false;
                }
                if (node.children) {
                    if (dragParentId && node.id === dragParentId) {
                        const idx = node.children.findIndex(c => c.id === dragId);
                        if (idx !== -1) {
                            dragItem = node.children[idx];
                            node.children = [...node.children.slice(0, idx), ...node.children.slice(idx + 1)];
                            return true; // 保留父节点
                        }
                    }
                    node.children = removeFromTree(node.children);
                }
                return true;
            });
        };

        let newChildren = removeFromTree(root.children || []);
        if (!dragItem) return;

        // 插入到 hoverId 位置（在 hoverParentId 下，位于 hoverId 之前或作为子项）
        const insertAt = (nodes: EventTreeNode[], targetId: string, item: EventTreeNode, targetParentId?: string): EventTreeNode[] => {
            if (!targetParentId) {
                const idx = nodes.findIndex(n => n.id === targetId);
                if (idx !== -1) {
                    const newNodes = [...nodes];
                    newNodes.splice(idx, 0, item);
                    return newNodes;
                }
                return nodes;
            }
            return nodes.map(node => {
                if (node.id === targetParentId && node.children) {
                    const idx = node.children.findIndex(c => c.id === targetId);
                    if (idx !== -1) {
                        const newChildren = [...node.children];
                        newChildren.splice(idx, 0, item);
                        return { ...node, children: newChildren };
                    }
                }
                if (node.children) {
                    return { ...node, children: insertAt(node.children, targetId, item, targetParentId) };
                }
                return node;
            });
        };

        root.children = insertAt(newChildren, hoverId, dragItem, hoverParentId);
    }

    function toggleGroupMode(id: string) {
        const root = activeRoot.value;
        if (!root) return;
        // 如果点击的是根组，直接修改
        if (root.id === id) {
            root.executionMode = root.executionMode === 'sequence' ? 'sync' : 'sequence';
            return;
        }
        // 递归查找分组
        const toggle = (nodes: EventTreeNode[]): boolean => {
            for (const node of nodes) {
                if (node.id === id && node.isGroup) {
                    node.executionMode = node.executionMode === 'sequence' ? 'sync' : 'sequence';
                    return true;
                }
                if (node.children && toggle(node.children)) return true;
            }
            return false;
        };
        toggle(activeRoot.value!.children || []);
    }

    // ---------- 展开折叠 ----------
    function toggleExpand(id: string) {
        const newSet = new Set(expandedGroups.value);
        if (newSet.has(id)) {
            newSet.delete(id);
        } else {
            newSet.add(id);
        }
        expandedGroups.value = newSet;
    }

    // ---------- 构建执行计划 ----------
    function buildExecutionPlan(): ExecutionNode | null {
        if (!activeRoot.value || !selectedWindow.value) return null;

        const hwnd = selectedWindow.value.hwnd;

        function convert(node: EventTreeNode): any {
            console.log(`node:${node}`)
            if (node.isGroup) {
                return {
                    group: {
                        mode: node.executionMode || 'sequence',
                        children: (node.children || []).map(convert),
                    },
                };
            }

            // 单个事件转换
            const intervalMs = (node.interval ?? 1);
            const count = (node.loopCount && node.loopCount > 0) ? node.loopCount : null;

            let action: EventAction;
            if (node.actionType === 'click') {
                action = { Click: {interval_ms: intervalMs, count: count }};
            } else if (node.actionType === 'longPress') {
                action = {
                    Hold: {
                    duration_ms: (node.duration ?? 1),
                    interval_ms: node.isContinuous ? 0 : intervalMs,
                    count: node.isContinuous ? null : count,
                    continuous: node.isContinuous || false,
                    }
                };
            } else { // scroll
                action = {
                    Scroll: {
                    delta: node.scrollDelta ?? 0,
                    interval_ms: intervalMs,
                    count: count,
                    }
                };
            }

            const inputEvent: InputEvent =
                node.type === 'mouse'
                    ? {
                        Mouse: {
                            hwnd,
                            btn: node.mouseButton ?? 'left',
                            action,
                            x: node.coordinates?.x ?? null,
                            y: node.coordinates?.y ?? null,
                        },
                    }
                    : {
                        Keyboard: {
                            hwnd,
                            key: node.keyboardKey ?? '',
                            action,
                        },
                    };

            return { event:{ data: inputEvent } };
        }

        const rootPlan = convert(activeRoot.value);
        if (totalLoopCount.value === 1) return rootPlan;
        return {loop: {count: totalLoopCount.value, child:rootPlan}};
    }

    // ---------- 执行控制 ----------
    async function run() {
        if (!selectedWindow.value) {
            console.warn("No window selected")
            return;
        }
        const plan = buildExecutionPlan();
        if (!plan) {
            console.warn("No plan built")
            return;
        }
        // 先设置运行状态
        isRunning.value = true;
        // 调用后端但不阻塞
        executePlan(plan).catch(err => {
            console.error("Execution start failed:", err);
            isRunning.value = false;
        });
    }

    async function stop() {
        await stopExecution();
        isRunning.value = false;
    }

    function toggleRun() {
        console.log("isRunning:",isRunning.value)
        isRunning.value ? stop() : run()
    }

    // ---------- 持久化 ----------
    async function save() {
        await saveRootGroups(rootGroups.value, totalLoopCount.value);
    }

    async function load() {
        loading.value = true;
        try {
            const groups = await loadRootGroups();
            if (groups.rootGroups && groups.rootGroups.length) {
                const normalized = groups.rootGroups.map(normalizeGroup);
                rootGroups.value = normalized;
                activeRootId.value = normalized[0].id;
                expandedGroups.value.clear();
                normalized.forEach(g => expandedGroups.value.add(g.id));
            }
            totalLoopCount.value = groups.totalLoopCount !== undefined ? groups.totalLoopCount : 1;
        }catch (err) {
            console.error('根组加载失败，使用默认方案',err);
        } finally {
            loading.value = false;
        }
    }

    function setTotalLoopCount(count: number | null) { totalLoopCount.value = count; }

    watch([rootGroups, totalLoopCount], () => {
        save();
    }, {deep:true});

    listen('execution-completed', () => {
        isRunning.value = false;
    });
    listen('execution-error', () => {
        isRunning.value = false;
    });

    return {
        // state
        availableWindows,
        selectedWindow,
        isRunning,
        loading,
        rootGroups,
        activeRootId,
        activeRoot,
        expandedGroups,
        selectedEventId,
        totalLoopCount,
        // actions
        fetchWindows,
        selectWindow,
        addRootGroup,
        deleteRootGroup,
        renameRootGroup,
        addGroup,
        addEventItem,
        updateEvent,
        deleteEvent,
        moveEvent,
        toggleGroupMode,
        toggleExpand,
        run,
        stop,
        toggleRun,
        save,
        load,
        setTotalLoopCount,
    };
});