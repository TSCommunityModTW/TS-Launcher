import styles from "./HomeWidget.module.scss"

import home from "@/assets/icons/home.png";
import home_selected from "@/assets/icons/home_selected.png"; 
import skin from "@/assets/icons/shirt.png";
import skin_selected from "@/assets/icons/shirt_selected.png";
import inventory from "@/assets/icons/inventory.png";
import inventory_selected from "@/assets/icons/inventory_selected.png";
import friend from "@/assets/icons/friend.png";
import friend_selected from "@/assets/icons/friend_selected.png";
import world from "@/assets/icons/world.png"
import world_selected from "@/assets/icons/world_selected.png"
import money from "@/assets/icons/money.png"
import money_selected from "@/assets/icons/money_selected.png"
import { useState } from "react";

type IProps = {
    playerName: string;
    onButtonClick: (buttonKey: string) => void; 
}
export default function HomeWidget(props: IProps) {

    const [selectedButton, setSelectedButton] = useState("");
    const buttons = [
        {
            key: "skin",
            icon: skin,
            selected_icon: skin_selected,
        },
        {
            key: "inventory",
            icon: inventory,
            selected_icon: inventory_selected,
        },
        {
            key: "friend",
            icon: friend,
            selected_icon: friend_selected,
        },
        {
            key: "world",
            icon: world,
            selected_icon: world_selected,
        },
        {
            key: "money",
            icon: money,
            selected_icon: money_selected,
        }
    ]

    return (
        <div className={styles.HomeWidgetContainer}>
            <div className={styles.description}>
                <div className={styles.titleavatar}>ICON HERE</div>玩家: <span className={styles.playerName}>{props.playerName}</span>您上次遊玩的內容
            </div>
            <div className={styles.menu}>
                <div key="home" className={styles.widgetitem} onClick={() => setSelectedButton("home")}>
                    <img 
                    className={selectedButton==="home"?styles.button_selected:styles.button} 
                    src={selectedButton==="home"?home_selected:home} />
                </div>

                <div className={styles.separator}></div>
                {
                    buttons.map((button) => {
                        return (
                            <div key={button.key} className={styles.widgetitem} onClick={() => setSelectedButton(button.key)}>
                                <img 
                                className={selectedButton===button.key?styles.button_selected:styles.button} 
                                src={selectedButton===button.key?button.selected_icon:button.icon} />
                            </div>
                            
                        )
                        
                    })
                }
            </div>
        </div>
    )
}