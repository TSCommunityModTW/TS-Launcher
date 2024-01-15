import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";

import styles from "./Main.module.scss";

import ServerList from "./components/serverList/ServerList";
import Top from "./components/top/Top";
import Store from "@/invoke/store";


export default function Main() {

    const [playerName, setPlayerName] = useState<string>("");
    const [playerUUID, setPlayerUUID] = useState<string>("");

    useEffect(() => {
        init();
    }, []);

    const init = async () => {

        let profiles = await Store.getProfiles();

        setPlayerName(profiles.player.name);
        setPlayerUUID(profiles.player.uuid);

    }

    return (
        <div className={styles.box}>
            
            <div className={styles.mainContainer}>
                

                <Top
                    userName={playerName}
                    userUUID={playerUUID}
                />

                <div className={styles.container}>
                    <ServerList/>
                    <Outlet />
                </div>

            </div>
        </div>
    )
}