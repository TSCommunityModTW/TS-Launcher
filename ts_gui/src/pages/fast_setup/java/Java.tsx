import { Outlet } from "react-router-dom";

import styles from "./Java.module.scss";

import javaImg from "@/assets/images/logo/java.svg";
import minecraftImg from "@/assets/images/logo/minecraft_1.svg";

export default function Java() {

    return (
        <div className={styles.javaContainer}>

            <div className={styles.leftContainer}>
                <div className={styles.javaIcon}>
                    <img className={styles.javaImg} src={javaImg} />
                    <img className={styles.minecraftImg} src={minecraftImg} />
                </div>
            </div>

            <div className={styles.rightContainer}>
                <Outlet />
            </div>

        </div>
    );
}