import {FormEvent, InputEvent, StoredEvent} from "../types/types.ts";

export const formatDuration = (duration: number | undefined): string => {
    if (!duration) return '无限'
    if (duration < 0) return '0毫秒';
    if (duration < 1000) return `${duration}毫秒`;
    const totalSeconds = duration/1000;
    if (Number.isInteger(totalSeconds)){
        if (totalSeconds < 60) return `${totalSeconds}秒`
        if (totalSeconds >= 60) {
            const minutes = Math.floor(totalSeconds / 60);
            const seconds = Math.floor(totalSeconds % 60);
            return `${minutes}分钟${seconds}秒`;
        }
    }
    return `${totalSeconds.toFixed(2)}秒`
}
export const formatEvent = (event: FormEvent & {hwnd?: number}): string => {
    const parts: string[] = [];
    if (event.type === "mouse") {
        parts.push(`🐭 鼠标`);
        if (event.actionType === 'click') {
            parts.push(`点击 ${event.button || '?'}`);
        } else if (event.actionType === 'hold') {
            parts.push(`长按 ${event.button || '?'} ${event.duration_ms}ms`);
        } else if (event.actionType === 'scroll') {
            parts.push(`滚轮 Δ=${event.delta}`);
        }
        if (event.x !== undefined && event.y !== undefined) {
            parts.push(`(${event.x ?? 0}, ${event.y ?? 0})`);
        }
    } else {
        parts.push(`⌨️ 键盘`);
        if (event.actionType === 'click') {
            parts.push(`点击 ${event.key}`);
        } else if (event.actionType === 'hold') {
            parts.push(`长按 ${event.key} ${formatDuration(<number>event.duration_ms)}`);
        } else {
            parts.push(`不支持的动作`);
        }
    }
    // 循环信息
    const countText = (event.count === null || event.count <= 0) ? '∞' : event.count;
    parts.push(`间隔 ${formatDuration(event.interval_ms)} x ${countText}`);
    return parts.join(' · ');
};

export const getEventType = (event: FormEvent & {hwnd?: number}): string => {
    if (event.type === "mouse") {
        if (event.button === 'left') {
            return "左键"
        } else if (event.button === 'right') {
            return "右键"
        } else if (event.button === 'middle') {
            return "中键"
        } else {
            return "自定义按键"
        }
    }
    else if (event.type === "keyboard") {
        return event.key ? event.key : ''
    }
    if (event.actionType === "scroll") {
        return "滚轮"
    }

    return ''; // 均不匹配
}
// 将前端表单事件转换为后端InputEvent
export const toBackendEvent = (form:FormEvent, hwnd: number): InputEvent => {
    let action:any;
    if (form.actionType === 'click') {
       action = {
           Click: {
               interval_ms: form.interval_ms,
               count: form.count === null || form.count <= 0 ? null : form.count,
           }
       }
    } else if (form.actionType === 'hold') {
        action = {
            Hold: {
                duration_ms: form.duration_ms!,
                interval_ms: form.interval_ms,
                count: form.count === null || form.count <= 0 ? null : form.count,
            }
        };
    } else if (form.actionType === 'scroll') {
        action = {
            Scroll: {
                delta: form.delta!,
                interval_ms: form.interval_ms,
                count: form.count === null || form.count <= 0 ? null : form.count,
            }
        };
    } else {
        throw new Error(`Unknown action type: ${form.actionType}`);
    }

    if (form.type === 'mouse') {
        return {
            Mouse: {
                hwnd,
                btn: form.actionType === 'scroll' ? 'wheel' : (form.button || 'left'),
                action,
                x: form.x ?? null,
                y: form.y ?? null,
            }
        };
    } else {
        return {
            Keyboard: {
                hwnd,
                action,
                key: form.key!,
            }
        };
    }
}

export const storedToFormEvent = (stored: StoredEvent): FormEvent => {
    const form: FormEvent = {
        type: stored.event_type as 'mouse' | 'keyboard',
        actionType: stored.action_type as 'click' | 'hold' | 'scroll',
        interval_ms: stored.interval_ms,
        count: stored.count,
    };
    if (stored.event_type === 'mouse') {
        const btn = stored.button;
        if (btn === 'left' || btn === 'right' || btn === 'middle') {
            form.button = btn;
        }
        form.x = stored.x;
        form.y = stored.y;
        if (stored.action_type === 'hold') {
            form.duration_ms = stored.duration_ms ?? undefined;
        }
        if (stored.action_type === 'scroll') {
            form.delta = stored.delta ?? undefined;
        }
    } else {
        form.key = stored.key ?? undefined;
        if (stored.action_type === 'hold') {
            form.duration_ms = stored.duration_ms ?? undefined;
        }
    }
    return form;
}
