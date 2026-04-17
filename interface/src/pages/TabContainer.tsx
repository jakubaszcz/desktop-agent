import { ReactNode } from "react";

interface TabContainerProps {
    text: string;
    children?: ReactNode;
}

const TabContainer = ({ text, children }: TabContainerProps) => {
    return (
        <div style={{
            background: "#1e1e1e",
            padding: "20px",
            color: "white",
            flex: 1,
            borderRadius: "12px",
            boxShadow: "0 4px 15px rgba(0, 0, 0, 0.3)",
            border: "1px solid #333",
            margin: "20px",
            boxSizing: "border-box",
            overflow: "auto"
        }}>
            <h1 style={{ marginBottom: "20px", fontSize: "1.5rem" }}>
                {text}
            </h1>
            <div style={{ fontSize: "1rem", lineHeight: "1.5" }}>
                {children}
            </div>
        </div>
    )
}

export default TabContainer;