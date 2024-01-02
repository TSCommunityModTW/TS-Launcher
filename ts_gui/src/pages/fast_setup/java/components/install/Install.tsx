import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Install.module.scss";

import Trail from "@/pages/components/trail/Trail";
import { useEffect, useState } from "react";
import Java from "@/invoke/java";
import { loading_listener } from "@/invoke/events";
import { useAppDispatch } from "@/hooks";
import { setCrashOpen } from "@/slices/stateSlice";

export default function Install() {

    const navigate = useNavigate();
    const dispatch = useAppDispatch();
    const { t } = useTranslation();
    const [progressBarText, setProgressBarText] = useState<string>("");
    const [progressBarPercentage, setProgressBarPercentage] = useState<number>(0);

    useEffect(() => {
        init();
    }, []);

    const init = async () => {

        const taskProgressMap = new Map();

        await loading_listener((payload: any) => {

            let fraction = Math.round(payload.fraction);
            let event = payload.event;

            taskProgressMap.set(event.version, fraction);
            let taskProgressArray = Array.from(taskProgressMap).map(([_name, value]) => (value));
            const totalProgress = taskProgressArray.reduce((sum, progress) => sum + progress, 0);
            const averageProgress = Math.round(totalProgress / 3);

            setProgressBarText(payload.message);
            setProgressBarPercentage(averageProgress);
        });

        Java.autoInstallJava()
            .then(() => {
                setTimeout(() => {
                    navigate("/java/install_success");
                }, 500);
            })
            .catch((err) => {
                dispatch(setCrashOpen({ state: true, errorMessage: err.message }));
                navigate("/java/setup");
            });
    }

    return (
        <div className={styles.installContainer}>

            <Trail open={true}>

                <h1>{t("java.install.text_1")}</h1>

                <div className={styles.versions}>
                    <ul>
                        <li>{t("java.install.text_2")}</li>
                        <li>{t("java.install.text_3")}</li>
                        <li>{t("java.install.text_4")}</li>
                    </ul>
                </div>

                <div className={styles.progressBarContainer}>

                    <div
                        className={styles.progressBar}
                        style={{ width: `${progressBarPercentage}%`}}
                    ></div>

                    <div className={styles.progressBarText}>
                        <h1>{progressBarText}</h1>
                        <h1>{progressBarPercentage}%</h1>
                    </div>

                </div>

            </Trail>

        </div>
    )
}