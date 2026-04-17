import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

const Dashboard = () => {

    const [usedMemory, setUsedMemory] = useState<number>(0);
    const [totalMemory, setTotalMemory] = useState<number>(0);

    useEffect(() => {
        let unlisten: (() => void) | null = null;

        listen<number>("memory-used", (event) => {
            setUsedMemory(event.payload);
        }).then((unlistenFn) => {
            unlisten = unlistenFn;
        });

        listen<number>("total-memory", (event) => {
            setTotalMemory(event.payload);
        }).then((unlistenFn) => {
            unlisten = unlistenFn;
        });

        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, []);

    return (
        <div>
            <p>Memory: {(usedMemory / 1024 ** 3).toFixed(2)} / {(totalMemory / 1024 ** 3).toFixed(2)} GiB</p>
        </div>
    )
}

export default Dashboard;