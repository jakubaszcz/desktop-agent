import "./App.css";
import { useState } from "react";
import Header from "./components/Header";
import Dashboard from "./pages/Dashboard";
import Warden from "./pages/Warden";
import { Pages } from "./pages/Tabs.ts";

function App() {

    const [activeTab, setActiveTab] = useState<Pages>(Pages.Dashboard);

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
                    background: "#2A2B2A",
                    color: "white",
                    height: "100vh",
                    overflow: "hidden"
                }}>
                {activeTab === Pages.Dashboard && (<Dashboard text="Dashboard" />)}
                {activeTab === Pages.Warden && (<Warden />)}
                {/* All the content will go here */}
            </div>
        </main>
    );
}

export default App;
