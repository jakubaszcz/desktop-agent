import Tab from "./header/Tab";
import { Tabs, Pages } from "../pages/Tabs.ts";

interface HeaderProps {
    OnClick: (page: Pages) => void;
    IsActive: Pages;
}

const Header = ({ OnClick, IsActive }: HeaderProps) => {
    return (
        <header>
            <div style={{
                padding: "20px",
                color: "white",
                borderBottom: "1px solid #333"
            }}>
                <h1 style={{ margin: 0, fontSize: "1.2rem" }}>Desktop Agent</h1>
            </div>

            <div>
                {
                    Tabs.map((tab, i) => (
                        <Tab
                            key={i}
                            text={tab.name}
                            isActive={IsActive === tab.page}
                            onClick={() => OnClick(tab.page)}
                        />
                    ))
                }
            </div>
        </header>
    );
};

export default Header;
