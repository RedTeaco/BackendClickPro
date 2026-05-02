// types/event.ts
export interface WindowInfo {
    hwnd: number;
    title: string;
    process_id: number;
    is_minimized: boolean;
}

export type ShortcutConfig = {
    start_stop: string,
    capture_window: string
}

export type ExecutionNode =
    | { event: { data: InputEvent }}
    | { group: { mode: 'sync' | 'sequence'; children: ExecutionNode[] }}
    | { loop: { count: number | null; child: ExecutionNode }};

export type InputEvent =
    | {
    Mouse: {
        hwnd: number;
        btn: string;
        action: EventAction;
        x: number | null;
        y: number | null;
    };
}
    | {
    Keyboard: {
        hwnd: number;
        key: string;
        action: EventAction;
    };
};

export type EventAction =
    | { Click: { interval_ms: number; count: number | null }}
    | { Hold: { duration_ms: number; interval_ms: number; count: number | null; continuous?: boolean }}
    | { Scroll:{ delta: number; interval_ms: number; count: number | null }};

// 基础字段
interface EventTreeNodeBase {
    id: string;
    name: string;
    children?: EventTreeNode[];
}

// 分组节点（isGroup 必须为 true）
export interface GroupTreeNode extends EventTreeNodeBase {
    isGroup: true;
    executionMode: 'sequence' | 'sync';
    children: EventTreeNode[]; // 分组必须有 children 数组
}

// 事件节点（isGroup 可选或为 false）
export interface EventItemNode extends EventTreeNodeBase {
    isGroup?: false;
    type: 'mouse' | 'keyboard';
    actionType: 'click' | 'longPress' | 'scroll';
    mouseButton?: 'left' | 'middle' | 'right' | 'wheel';
    keyboardKey?: string;
    isContinuous?: boolean;
    interval?: number;
    duration?: number;
    loopCount?: number | null;
    coordinates?: { x: number; y: number };
    scrollDelta?: number;
}

export type EventTreeNode = GroupTreeNode | EventItemNode;