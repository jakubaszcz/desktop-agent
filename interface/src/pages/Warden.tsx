import TabContainer from "./TabContainer";
import { listen } from "@tauri-apps/api/event";

import { invoke } from "@tauri-apps/api/core"

import { useState, useEffect } from "react";

interface Response {
    data: boolean
    status: string
}

const Warden = () => {

    const [isCommandEmptyTrashPressed, setCommandEmptyTrashState] = useState<Response>({data: false, status: ""});

    const handleClick = async () => {
        if (isCommandEmptyTrashPressed.data) return;
        setCommandEmptyTrashState({ data: true, status: "pending"});
        await invoke("send_to_server", {
            msg: JSON.stringify({ type: "command", action: "warden:empty_trash" })
        })
    }

    useEffect(() => {
        let unlisten: (() => void) | null = null;

        listen<Response>("response", (event) => {
            setCommandEmptyTrashState({ data: event.payload.data, status: event.payload.status});
            
            // Effacer le message après 3 secondes si ce n'est plus en "pending"
            if (event.payload.status !== "pending") {
                setTimeout(() => {
                    setCommandEmptyTrashState(prev => ({ ...prev, status: "" }));
                }, 3000);
            }
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
        <div style={{
            height: "100%",
            display: "flex",
            flexDirection: "column",
            boxSizing: "border-box"
        }}>
            <TabContainer text="Warden">
                <div style={{ padding: "20px", display: "flex", flexDirection: "column", gap: "10px" }}>
                    <button 
                        onClick={handleClick}
                        disabled={isCommandEmptyTrashPressed.data}
                        style={{
                            padding: "10px 20px",
                            backgroundColor: isCommandEmptyTrashPressed.data ? "#ccc" : "#007bff",
                            color: "white",
                            border: "none",
                            borderRadius: "5px",
                            cursor: isCommandEmptyTrashPressed.data ? "not-allowed" : "pointer",
                            fontSize: "16px",
                            fontWeight: "bold"
                        }}
                    >
                        {isCommandEmptyTrashPressed.data ? "Loading..." : "Empty trash"}
                    </button>
                    {isCommandEmptyTrashPressed.status && (
                        <p style={{
                            margin: 0,
                            padding: "10px",
                            borderRadius: "4px",
                            backgroundColor: isCommandEmptyTrashPressed.status === "pending" ? "#e9ecef" : 
                                             isCommandEmptyTrashPressed.status === "success" ? "#d4edda" : "#f8d7da",
                            color: isCommandEmptyTrashPressed.status === "pending" ? "#495057" :
                                   isCommandEmptyTrashPressed.status === "success" ? "#155724" : "#721c24",
                            border: `1px solid ${
                                isCommandEmptyTrashPressed.status === "pending" ? "#dee2e6" : 
                                isCommandEmptyTrashPressed.status === "success" ? "#c3e6cb" : "#f5c6cb"
                            }`
                        }}>
                            Statut : {isCommandEmptyTrashPressed.status}
                        </p>
                    )}
                </div>
            </TabContainer>
        </div>
    )
}

export default Warden;