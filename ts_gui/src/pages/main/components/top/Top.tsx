// import ReactSkinview3d from "react-skinview3d"
// import { WalkingAnimation } from "skinview3d";
import { useNavigate } from "react-router-dom";

import styles from "./Top.module.scss";

import { useState } from "react";
import { useTranslation } from "react-i18next";

import ts_1 from "@/assets/images/logo/ts_1.png";
import settingLines from "@/assets/icons/settings.png";

import TopAnnounment from "./topAnnounment/TopAnnounment";




type IProps = {
    userName: string;
    userUUID: string;
}



export default function Top(props: IProps) {

    const navigate = useNavigate();
    const { t } = useTranslation();

    const [dropdownVisible, setDropdownVisible] = useState(false);
    const toggleDropdown = () => {
        setDropdownVisible(!dropdownVisible);
    };

    
    const handleSelect = (selection: string) => {
        switch (selection) {
            case 'select_1':
                navigate("/main/home");
                break;
            case 'select_2':
                navigate("/main/home");
                break;
            case 'select_3':
                navigate("/main/confirm_logout");
                break;
            case 'login':
                navigate("/login/account");
                break;
        }


    };
    
    return (
        <div className={styles.topContainer}>

            <div className={styles.logoContainer}>

                <img src={ts_1} />

            </div>

            <TopAnnounment needUpdate={false} announment={[]} />

            <div className={styles.playerSettingCloseContainer}>
                {props.userName && props.userUUID ?
                    <div className={styles.playerContainer} onClick={toggleDropdown}>
                        {dropdownVisible && (
                            <div className={styles.dropdownContent}>
                                <div className={styles.button}><p onClick={() => handleSelect('select_1')}>{t("main.components.topPlayer.playerMenu.select_1")}</p></div>
                                <div className={styles.button}><p onClick={() => handleSelect('select_2')}>{t("main.components.topPlayer.playerMenu.select_2")}</p></div>
                                <div className={styles.button}><p onClick={() => handleSelect('select_3')}>{t("main.components.topPlayer.playerMenu.select_3")}</p></div>
                            </div>
                        )}
                        <h1>{props.userName}</h1>

                        <div className={styles.playerBodyImg} style={props.userUUID.length > 0 ? { backgroundImage: `url(https://visage.surgeplay.com/bust/70/${props.userUUID}?y-40)` } : undefined}></div>



                    </div> :
                    <div className={styles.buttonContainer}>
                        <div className={styles.loginButton} onClick={() => handleSelect('login')}>{t("main.components.topPlayer.playerMenu.login")}</div>
                    </div>
                }
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