import styles from "./HomePageWidget.module.scss"
type IProps = {
    playerName: string;
}
export default function HomePageWidget(props:IProps){



    return (
        <div>
            <div>
                玩家: 玩家您可以在此處設定皮膚
            </div>
            <div>
                <div>Home</div>
                <div className={styles.separator}></div>
                <div>Skin</div>
                <div>Friend</div>
                <div>Other setting</div>
                <div>dotatate</div>
                <div>inventory</div>
            </div>
        </div>
    )
}