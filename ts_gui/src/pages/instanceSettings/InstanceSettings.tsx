import { Outlet, useNavigate, useParams } from "react-router-dom";

import styles from "./InstanceSettings.module.scss";

import Menu from "./components/menu/Menu";
import Store from "@/invoke/store";

export default function InstanceSettings() {

    let { instanceId } = useParams<{ instanceId: string }>();

    if (instanceId === undefined) {
        return null;
    }

    const navigate = useNavigate();

    const backMain = () => {
        Store.saveLauncherSettingsFile();
        navigate("/main");
    }

    return (
        <div className={styles.instanceSettingDiv}>

            <div className={styles.backButtonBorderDiv}>
                <div className={styles.backButton} onClick={backMain}>
                    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none" /><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z" /></svg>
                </div>
            </div>

            <div className={styles.leftDiv}>
                <Menu instanceId={instanceId} />
            </div>

            <div className={styles.rightDiv}>
                <Outlet />
            </div>

        </div>
    );
}
