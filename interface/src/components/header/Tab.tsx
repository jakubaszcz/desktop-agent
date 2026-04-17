interface TabProps {
    text: string;
    isActive: boolean;
    onClick: () => void;
}

const Tab = ({ text, isActive, onClick }: TabProps) => {
    return (
        <button
            onClick={onClick}
            style={{
                width: "100%",
                outline: "none",
                background: isActive ? "#2A2B2A" : "none",
                border: "none",
                padding: "20px",
                color: "white",
                borderBottom: "1px solid #2A2B2A",
                textAlign: "left"
            }}
        >
            <h1 style={{margin: 0, fontSize: "1rem"}}>{text}</h1>
        </button>
    )
}

export default Tab;