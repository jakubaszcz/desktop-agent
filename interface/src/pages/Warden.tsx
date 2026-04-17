import TabContainer from "./TabContainer";

import { invoke } from "@tauri-apps/api/core"

const Warden = () => {

    const handleClick = async () => {
        console.log("clicked")
        await invoke("send_to_server", {
            msg: JSON.stringify({ type: "command", action: "warden:empty_trash" })
        })
    }

    return (
        <div style={{
            height: "100%",
            display: "flex",
            flexDirection: "column",
            boxSizing: "border-box"
        }}>
            <TabContainer text="Warden">
                <button onClick={handleClick}>Empty trash</button>
            </TabContainer>
        </div>
    )
}

export default Warden;