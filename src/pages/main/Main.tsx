import styles from "./Main.module.scss";

import Trail from "../components/trail/Trail";
import ServerList from "./components/serverList/ServerList";
import Top from "./components/top/Top";
import settingLines from "@/assets/icons/setting-lines.png";
import ButtonPlay from "./components/buttonPlay/ButtonPlay";
import modpack from "@/assets/images/modpack.png";

export default function Main() {

    const serverName = "無名伺服器";
    const title = "主服模組包伺服器";
    const serverId = "";

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
                userName="yucheng_0918"
                userUUID="93ea0589ec754cad8619995164382e8d?"
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

                                        }}>
                                            <img src={settingLines} alt="setting-lines" />
                                        </div>

                                        <ButtonPlay serverId={serverId} />

                                    </div>
                                </Trail>
                            </div>

                            <div className={styles.modpackContainer}>
                                <img src={modpack} />
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