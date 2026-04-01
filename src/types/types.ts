export interface sWindow {
    hwnd: number;
    title: string;
    process_id: number;
    is_minimized: boolean;
}

export interface km_event {
    type: "keyboard" | "mouse";
    mode: "click" | "press";
    duration?: number;
    key?: string;
    button?: "left" | "right" | "middle" | string;
}