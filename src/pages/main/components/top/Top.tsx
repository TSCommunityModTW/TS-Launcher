// import ReactSkinview3d from "react-skinview3d"
// import { WalkingAnimation } from "skinview3d";
import { useNavigate } from "react-router-dom";

import styles from "./Top.module.scss";

import ts_1 from "@/assets/images/logo/ts_1.png";
import settingLines from "@/assets/icons/settings.png";

type IProps = {
    userName: string;
    userUUID: string;
}

export default function Top(props: IProps) {

    const navigate = useNavigate();

    return (
        <div className={styles.topContainer}>

            <div className={styles.logoContainer}>

                <img src={ts_1} />

            </div>

            <div className={styles.playerSettingCloseContainer}>

                <div className={styles.playerContainer}>

                    <h1>{props.userName}</h1>
                    
                    <div className={styles.playerBodyImg} style={ props.userUUID.length > 0 ? { backgroundImage: `url(https://visage.surgeplay.com/bust/70/${props.userUUID}?y-40)` } : undefined }></div>

                    {/* <ReactSkinview3d
                        // skinUrl={`"https://crafatar.com/skins/"${props.userUUID}`}
                        skinUrl={skin_1}
                        height="500"
                        width="500"
                        onReady={({ viewer }) => {
                            const walkingAnimation = new WalkingAnimation();
                            walkingAnimation.headBobbing = false;
                            viewer.animation = walkingAnimation;
                            viewer.animation.speed = 0.5;
                        }}
                    /> */}

                </div>

                <div className={styles.buttonContainer}>

                    <div className={styles.settingButton}
                        onClick={() => {
                            navigate("/settings/general");
                        }}    
                    >
                        <img src={settingLines} />
                    </div>

                    <div className={styles.closeButton}
                        onClick={() => {
                            navigate("/welcome");
                        }}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" height="30px" viewBox="0 0 24 24" width="30px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none" /><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z" /></svg>
                    </div>

                </div>

            </div>

        </div>
    )
}