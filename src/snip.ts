import { convertFileSrc, invoke } from "@tauri-apps/api/core";
type Rect = { x: number; y: number; w: number; h: number };

export async function snipAndDraw(canvas: HTMLCanvasElement, rect: Rect) {
    const filePath = await invoke<string>("snip_to_file", rect);
    const url = convertFileSrc(filePath);

    const img = new Image();
    try {
        await new Promise<void>((resolve, reject) => {
            img.onload = () => resolve();
            img.onerror = reject;
            img.src = url;
        });

        const ctx = canvas.getContext("2d");
        canvas.width = img.width;
        canvas.height = img.height;
        ctx?.drawImage(img, 0, 0);
    } catch (error) {
        console.error(error);
    }
}
