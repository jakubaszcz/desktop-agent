import "./App.css";

function App() {

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
                {/* This is where the sidebar will go */}
            </div>
            <div style={
                {
                    flex: 1,
                    background: "#f5f5f5"
                }}>
                {/* All the content will go here */}
            </div>
        </main>
    );
}

export default App;
