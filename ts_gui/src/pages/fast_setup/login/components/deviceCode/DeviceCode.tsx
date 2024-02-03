import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { open } from "@tauri-apps/api/shell";
import { useEffect, useState } from "react";

import styles from "./DeviceCode.module.scss";

import { useAppDispatch } from "@/hooks";
import { setCrashOpen } from "@/slices/stateSlice";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import Auth from "@/invoke/auth";

export default function DeviceCode() {

    const { type } = useParams() as { type: "new" | "refresh" };
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { t } = useTranslation();
    const [deviceCode, setDeviceCode] = useState<string>("");

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        try {

            const auth = new Auth();
            setDeviceCode(await auth.getDeviceCode());

            if (await auth.auth_minecraft_await()) {
                navigate(`/login/link_success/${type}`);
            } else {
                navigate("/login/link_error");
            }

        } catch (err: any) {
            dispatch(setCrashOpen({ state: true, errorMessage: err.message }));
            navigate("/login/account");
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
                        disabled={ deviceCode.length > 0 ? false : true }
                        onClick={() => {
                            open("https://www.microsoft.com/link");
                        }}
                    />

                </div>

            </Trail>

        </div>
    )
}