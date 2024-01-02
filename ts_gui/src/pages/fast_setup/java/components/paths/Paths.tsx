import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { open } from "@tauri-apps/api/dialog";
import { Dispatch, SetStateAction, useEffect, useRef, useState } from "react";

import styles from "./Paths.module.scss";

import FileIcon from "@/assets/icons/file.svg?react";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import Trail from "@/pages/components/trail/Trail";
import Store, { JavaPathVersion } from "@/invoke/store";
import { IStoreSettingsJava } from "@/interfaces/IStoreSettingsJava";

export default function Paths() {

    const navigate = useNavigate();
    const { t } = useTranslation();

    const [java17Path, setJava17Path] = useState<string>("");
    const [java16Path, setJava16Path] = useState<string>("");
    const [java8Path, setJava8Path] = useState<string>("");

    const storeSettingsJavaRef = useRef<IStoreSettingsJava>();

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        storeSettingsJavaRef.current = await Store.getSettingsJava("global");
    }

    const userSelectedFilePath = async (version: JavaPathVersion, setPath: Dispatch<SetStateAction<string>>) => {

        let filePath = await open({
            multiple: false
        });

        if (!filePath) return;

        filePath = filePath as string;

        setPath(filePath);

        switch (version) {
            case 0: {
                storeSettingsJavaRef.current!.java8_path = filePath;
                break;
            }
            case 1: {
                storeSettingsJavaRef.current!.java16_path = filePath;
                break;
            }
            case 2: {
                storeSettingsJavaRef.current!.java17_path = filePath;
                break;
            }
        }

        Store.setSettingsJava("global", storeSettingsJavaRef.current!);
    }

    const setGlobalAllJavaVersionPath = () => {
        storeSettingsJavaRef.current!.java8_path = java8Path;
        storeSettingsJavaRef.current!.java16_path = java16Path;
        storeSettingsJavaRef.current!.java17_path = java17Path;
        Store.setSettingsJava("global", storeSettingsJavaRef.current!);
    }

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
                        <input
                            type="text"
                            value={java17Path}
                            onChange={(event) => setJava17Path(event.target.value)}
                        />
                        <FileIcon
                            className={styles.fileIcon}
                            onClick={() => {
                                userSelectedFilePath(JavaPathVersion.Java17, setJava17Path);
                            }}
                        />
                    </div>

                </div>

                <div className={styles.inputContainer}>

                    <span className={styles.text}>
                        Java 16
                    </span>

                    <div className={styles.inputText}>
                        <input
                            type="text"
                            value={java16Path}
                            onChange={(event) => setJava16Path(event.target.value)}
                        />
                        <FileIcon
                            className={styles.fileIcon}
                            onClick={() => {
                                userSelectedFilePath(JavaPathVersion.Java16, setJava16Path);
                            }}
                        />
                    </div>

                </div>

                <div className={styles.inputContainer}>

                    <span className={styles.text}>
                        Java 8
                    </span>

                    <div className={styles.inputText}>
                        <input
                            type="text"
                            value={java8Path}
                            onChange={(event) => setJava8Path(event.target.value)}
                        />
                        <FileIcon
                            className={styles.fileIcon}
                            onClick={() => {
                                userSelectedFilePath(JavaPathVersion.Java8, setJava8Path);
                            }}
                        />

                    </div>

                </div>

                <ButtonFocus
                    content={t("java.paths.text_3")}
                    onClick={() => {
                        setGlobalAllJavaVersionPath();
                        Store.saveLauncherSettingsFile()
                        navigate("/setup_success");
                    }}
                />

            </Trail>

        </div>
    )
}