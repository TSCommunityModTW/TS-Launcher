import styles from "./HomeWidget.module.scss"

import home from "@/assets/icons/home.png";
//import home_selected from "@/assets/icons/home_selected.png"; s
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


type IProps = {
    playerName: string;
}
export default function HomeWidget(props: IProps) {

    const buttons = [
        {
            icon: skin,
            selected_icon: skin_selected,
        },
        {
            icon: inventory,
            selected_icon: inventory_selected,
        },
        {
            icon: friend,
            selected_icon: friend_selected,
        },
        {
            icon: world,
            selected_icon: world_selected,
        },
        {
            icon: money,
            selected_icon: money_selected,
        }
    ]

    return (
        <div className={styles.HomeWidgetContainer}>
            <div className={styles.header}>
                <div className={styles.titleavatar}>ICON HERE</div>玩家: <span className={styles.playerName}>{props.playerName}</span>你上次遊玩的內容
            </div>
            <div className={styles.menu}>
                <img className={styles.button} src={home} alt="Home" />
                <div className={styles.separator}></div>
                {
                    buttons.map((button) => {
                        return (
                                <img className={styles.button} src={button.icon} />
                        )
                    })
                }
            </div>
        </div>
    )
}