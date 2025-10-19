import { useState } from "react";
import { onSnip } from "./snip";

export default function App() {
    const [imgUrl, setImgUrl] = useState<string | null>(null);

    const handleSnap = async () => {
        const url = await onSnip();
        setImgUrl(url);
    };

    return (
        <div style={{ padding: 24 }}>
            <button onClick={handleSnap}>Rect Snip</button>
            {imgUrl && (
                <div style={{ marginTop: 8 }}>
                    <img src={imgUrl} style={{ maxWidth: 600, border: "1px solid #008479ff" }} />
                </div>
            )}
        </div>
    );
}
