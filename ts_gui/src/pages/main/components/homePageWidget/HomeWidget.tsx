import styles from "./HomeWidget.module.scss"
type IProps = {
    playerName: string;
}
export default function HomeWidget(props: IProps) {


    return (
        <div className={styles.HomeWidgetContainer}>
            <div className={styles.header}>
                <div className={styles.titleavatar}>ICON HERE</div>玩家: <div className={styles.playerName}>玩家</div>您可以在此處設定皮膚
            </div>
            <div className={styles.menu}>
                <div className={styles.button}>
                    <div className={styles.menuitem}>HOME</div>
                </div>
                <div className={styles.separator}></div>
                <div className={styles.button}>
                    <div className={styles.menuitem}>Skin</div>
                </div>
                <div className={styles.button}>
                    <div className={styles.menuitem}>Friend</div>
                </div>
                <div className={styles.button}>
                    <div className={styles.menuitem}>Other setting</div>
                </div>
                <div className={styles.button}>
                    <div className={styles.menuitem}>dotatate</div>
                </div>
                <div className={styles.button}>
                    <div className={styles.menuitem}>inventory</div>
                </div>
            </div>
        </div>
    )
}