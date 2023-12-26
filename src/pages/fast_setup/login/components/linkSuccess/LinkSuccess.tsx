import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./LinkSuccess.module.scss";

import Trail from "@/pages/components/trail/Trail";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import { useEffect, useState } from "react";
import Player from "@/api/player";
export default function LinkSuccess() {

    const navigate = useNavigate();
    const { t } = useTranslation();
    const [playerName, setPlayerName] = useState<string>("");

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        setPlayerName(await Player.get_name());
    };

    return (
        <div className={styles.linkSuccessContainer}>

            <Trail open={true}>

                <h1>{t("login.link_success.text_1")}</h1>
                <h2>{t("login.link_success.text_2")}</h2>

                <div className={styles.buttonContainer}>

                    <h1 className={styles.playerName}>
                        {playerName}
                    </h1>

                    <ButtonFocus
                        content={t("login.link_success.text_3")}
                        themeColor="green"
                        onClick={() => {
                            navigate("/java/setup");
                        }}
                    />

                </div>

            </Trail>

        </div>
    )
}