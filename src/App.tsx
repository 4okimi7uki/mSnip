import { useRef } from "react";
import { snipAndDraw } from "./snip";

export default function App() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    const onClick = async () => {
        const canvas = canvasRef.current!;
        // まずは固定座標でテスト（後で選択UIに差し替え）
        await snipAndDraw(canvas, { x: 100, y: 100, w: 500, h: 300 });
    };

    return (
        <div style={{ padding: 24 }}>
            <div>
                <code>Debug tool: cmd + option + I</code>
            </div>
            <button onClick={onClick}>Snip!</button>
            <div>
                <canvas ref={canvasRef} style={{ border: "1px solid #ccc", marginTop: 12 }} />
            </div>
        </div>
    );
}
