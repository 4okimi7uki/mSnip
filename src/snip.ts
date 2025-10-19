import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";

export async function onSnip(): Promise<string | null> {
    try {
        const filePath = await invoke<string>("screencap_interactive");
        const url = convertFileSrc(filePath);
        console.log(url);

        return url;
    } catch (e) {
        console.warn(e);
    }
    return null;
}
