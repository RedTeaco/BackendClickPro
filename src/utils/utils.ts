import {km_event} from "../types/types.ts";

export const formatKmEvent = (event: km_event): string => {
    const translateButton = (btn: string): string => {
        const map: Record<string,string> = {
            left: "左键",
            right: "右键",
            middle: "中键",
            "mouse wheel up": "滚轮上",
            "mouse wheel down": "滚轮下",
        }
        return map[btn] || btn;
    };

    const formatDuration = (duration: number): string => {
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
    if (event.type === "keyboard") {
        const action = event.mode === "click" ? "点按" : "长按";
        let result = `键盘事件:${action} 按键:${event.key ?? "未知按键"}`;
        if (event.mode === "press" && event.duration !== undefined) {
            result += `持续时间:${formatDuration(event.duration)}`;
        }
        return result;
    } else {
        const action = event.mode === "click" ? "点击" : "长按";
        const buttonText = event.button ? translateButton(event.button) : "未知按键";
        let result = `鼠标事件:${action} 按键:${buttonText}`;
        if (event.mode === "press" && event.duration !== undefined) {
            result += `持续时间:${formatDuration(event.duration)}`;
        }
        return result;
    }
}