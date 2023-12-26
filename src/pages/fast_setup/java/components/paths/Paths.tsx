import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

import styles from "./Paths.module.scss";

import FileIcon from "@/assets/icons/file.svg?react";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import Trail from "@/pages/components/trail/Trail";

export default function Paths() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    return (
        <div className={styles.pathsContainer}>

            <Trail open={true}>

                <h1 className={styles.title}>
                    {t("java.paths.text_1")}
                </h1>
                <h2 className={styles.subtitle}>
                    {t("java.paths.text_2")}
                </h2>

                <div className={styles.inputContainer}>

                    <span className={styles.text}>
                        Java 17
                    </span>

                    <div className={styles.inputText}>
                        <input type="text" />
                        <FileIcon className={styles.fileIcon} />
                    </div>

                </div>

                <div className={styles.inputContainer}>

                    <span className={styles.text}>
                        Java 16
                    </span>

                    <div className={styles.inputText}>
                        <input type="text" />
                        <FileIcon className={styles.fileIcon} />
                    </div>

                </div>

                <div className={styles.inputContainer}>

                    <span className={styles.text}>
                        Java 8
                    </span>

                    <div className={styles.inputText}>
                        <input type="text" />
                        <FileIcon className={styles.fileIcon} />
                    </div>

                </div>

                <ButtonFocus
                    content={t("java.paths.text_3")}
                    onClick={() => {
                        navigate("/setup_success")
                    }}
                />

            </Trail>

        </div>
    )
}