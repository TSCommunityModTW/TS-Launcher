import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { open } from "@tauri-apps/api/shell";

import styles from "./DeviceCode.module.scss";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import { useEffect, useState } from "react";
import Auth from "@/api/auth";

export default function DeviceCode() {

    const navigate = useNavigate();
    const { t } = useTranslation();
    const [deviceCode, setDeviceCode] = useState<string>();

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        try {

            const auth = new Auth();
            setDeviceCode(await auth.getDeviceCode());
            await auth.auth_verification_await();
            navigate("/login/link_success");

        } catch (err) {

            console.error(err);

            // navigate("/login/link_error");

        }
    }

    return (
        <div className={styles.deviceCodeContainer}>

            <Trail open={true}>

                <h1>{t("login.device_code.text_1")}</h1>
                <h2>{t("login.device_code.text_2")}</h2>

                <div className={styles.buttonContainer}>

                    <div className={styles.code}>
                        {deviceCode}
                    </div>

                    <ButtonFocus
                        content={t("login.device_code.text_3")}
                        themeColor="green"
                        onClick={() => {
                            open("https://www.microsoft.com/link");
                        }}
                    />

                </div>

            </Trail>

        </div>
    )
}