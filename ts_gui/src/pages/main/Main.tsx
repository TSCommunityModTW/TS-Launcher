import { useEffect } from "react";
import { Outlet, useLoaderData, useNavigate } from "react-router-dom";

import styles from "./Main.module.scss";

import ServerList from "./components/serverList/ServerList";
import Top from "./components/top/Top";
import { IMainLoader } from "@/loader";

import logger from "@/invoke/logger";

export default function Main() {

    const loaderData = useLoaderData() as IMainLoader;
    const navigate = useNavigate();

    useEffect(() => { init(); }, []);

    const init = async () => {

        navigate("/main/home");

    }
    logger.logMessage("debug","Main Page Loaded.");
    return (
        <div className={styles.transparentBackground}>

            <div className={styles.box}>

                <div className={styles.mainContainer}>

                    <Top
                        userName={loaderData.player.name}
                        userUUID={loaderData.player.uuid}
                    />

                    <div className={styles.container}>
                        <ServerList />
                        <Outlet />
                    </div>

                </div>

            </div>
            
        </div>
    )
}