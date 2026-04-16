import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import Header from "./components/Header";
import { Pages } from "./pages/Tabs.ts";

function App() {

    const [activeTab, setActiveTab] = useState<Pages>(Pages.Dashboard);

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
        <main style={
            {
                display: "flex",
                height: "100vh",
            }}>
            <div style={
                {
                    width: "30%",
                    background: "#1e1e1e"
                }}>
                <Header OnClick={setActiveTab} IsActive={activeTab}  />
            </div>
            <div style={
                {
                    flex: 1,
                    background: "#f5f5f5",
                    padding: "20px"
                }}>
                {activeTab === Pages.Dashboard && (<p>Memory: {(usedMemory / 1024 ** 3).toFixed(2)} / {(totalMemory / 1024 ** 3).toFixed(2)} GiB</p>)}
                {activeTab === Pages.Warden && (<p>Warden Content</p>)}
                {/* All the content will go here */}
            </div>
        </main>
    );
}

export default App;
