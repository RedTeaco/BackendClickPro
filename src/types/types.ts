export interface sWindow {
    hwnd: number;
    title: string;
    process_id: number;
    is_minimized: boolean;
}

// 事件动作类型(EventAction)
export type EventAction =
    | {type: 'Click'; interval_ms: number; count: number |  null } // count null = infinite
    | {type: 'Hold'; duration_ms: number; interval_ms: number; count: number |  null } // count null = infinite
    | {type: 'Scroll'; delta: number; interval_ms: number; count: number |  null } // count null = infinite 仅mouse_event

// 输入事件
export type InputEvent =
    | {
    Mouse: {
        hwnd: number;
        btn: string;
        action: EventAction;
        x: number | null; // null 后端默认0
        y: number | null; // null 后端默认0
        }
    }
    | {
    Key: {
        hwnd: number;
        key: string;
        action: EventAction; // 不支持Scroll
        };
    }

// 前端表单使用的临时事件(未指定hwnd，在添加时使用全局选中窗口填充)
export interface FormEvent {
    type: 'mouse' | 'keyboard';
    actionType: 'click' | 'hold' | 'scroll'; // scroll仅鼠标
    // 鼠标特有
    button?: 'left' | 'right' | 'middle';
    x?: number | null;
    y?: number | null;
    delta?: number; // scroll 必需
    // 键盘特有
    key?: string;
    // 动作参数
    duration_ms?:number; // 持续事件 hold必需
    interval_ms: number; // 间隔事件
    count: number | null; // 循环次数 null = infinite
}

export interface StoredEvent {
    event_type: string;
    action_type: string;
    button: string | null;
    key: string | null;
    x: number | null;
    y: number | null;
    delta: number | null;
    duration_ms: number | null;
    interval_ms: number;
    count: number | null;
}