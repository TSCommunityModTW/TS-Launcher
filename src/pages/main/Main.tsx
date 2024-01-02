import { useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";

import styles from "./Main.module.scss";

import Trail from "../components/trail/Trail";
import ServerList from "./components/serverList/ServerList";
import Top from "./components/top/Top";
import settingLines from "@/assets/icons/setting-lines.png";
import ButtonPlay from "./components/buttonPlay/ButtonPlay";
import modpack from "@/assets/images/miscs/modpack.png";
import Store from "@/invoke/store";


export default function Main() {

    const navigate = useNavigate();
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

    const serverName = "無名伺服器";
    const title = "主服模組包伺服器";
    const serverId = "nr-server";

    const servers = [
        {
            id: 1,
            title: "主服務器",
            img: modpack
        },
        {
            id: 2,
            title: "次服務器",
            img: modpack
        },
        {
            id: 3,
            title: "TS-WAR",
            img: modpack
        },
        // {
        //     id: 1,
        //     title: "主服務器",
        //     img: modpack
        // },
        // {
        //     id: 2,
        //     title: "次服務器",
        //     img: modpack
        // },
        // {
        //     id: 3,
        //     title: "𝗧𝗦-𝗪𝗔𝗥",
        //     img: modpack
        // }
    ]

    return (
        <div className={styles.mainContainer}>

            <Top
                userName={playerName}
                userUUID={playerUUID}
            />

            <div className={styles.container}>

                <ServerList
                    onChangeIndex={(index) => {
                        console.log(index);
                    }}
                />

                <div className={styles.serverContainer}>

                    <div className={styles.scroll}>
                        <div className={styles.playServerGameContainer}>

                            <div className={styles.leftDiv}>
                                <Trail open={true}>
                                    <h1 className={styles.serverNameH1}>{serverName}</h1>
                                    <h2 className={styles.serverNameH2}>{title}</h2>
                                    <div className={styles.buttonDiv}>
                                        <div className={styles.settingButton} onClick={() => {
                                            navigate(`/instanceSettings/${serverId}/parameters`);
                                        }}>
                                            <img src={settingLines} alt="setting-lines" />
                                        </div>

                                        <ButtonPlay serverId={serverId} />

                                    </div>
                                </Trail>
                            </div>

                            <div className={styles.modpackContainer}>
                                <Trail open={true}>
                                    <img src={modpack} />    
                                </Trail>
                            </div>

                        </div>

                        <div className={styles.childServerList}>
                            {
                                servers.map(item => {
                                    return (
                                        <div key={item.id} className={styles.serverDiv} onClick={() => {

                                        }}>
                                            <div className={styles.serverBorderDiv}>
                                                <div>
                                                    <img src={item.img} />
                                                </div>
                                            </div>
                                            <div className={styles.titleContainer}>
                                                <div className={styles.leftTitleDiv}>
                                                    <h1>{item.title}</h1>
                                                </div>
                                                <div className={styles.rightTitleDiv}>
                                                    <div className={styles.serverState}></div>
                                                    <h1>在線人數: {5}</h1>
                                                </div>
                                            </div>
                                        </div>
                                    )
                                })
                            }
                        </div>
                    </div>

                </div>

            </div>

        </div>
    )
}