import React from "react";

import styles from "./ServerList.module.scss";

import home from "@/assets/icons/home.png";
import server_1 from "@/assets/images/servers/server_1.png";
import server_2 from "@/assets/images/servers/server_2.png";
import server_3 from "@/assets/images/servers/server_3.png";
import server_4 from "@/assets/images/servers/server_4.png";

type IProps = {
    onChangeIndex?: (server: number) => void;
}

export default function ServerList(props: IProps) {

    const servers = [
        {
            id: 1,
            name: "GXS2.0 模組伺服器",
            icon: server_4
        },
        {
            id: 2,
            name: "沐緹斯伺服器",
            icon: server_1
        },
        {
            id: 3,
            name: "Nameless Realms 無名伺服器",
            icon: server_2
        },
        {
            id: 4,
            name: "TS 模組伺服器",
            icon: server_3
        }
    ]

    const [selectLocation, setSelectLocation] = React.useState<number>(1);

    return (
        <div className={styles.serverListContainer}>

            <div className={styles.homeButton}>
                {
                    selectLocation === 0
                    ?
                    <div className={styles.focus}></div>
                    :
                    null
                }
                <div className={styles.focusHover}></div>
                <img src={home}
                    onClick={() => {
                        setSelectLocation(0);
                        if (props.onChangeIndex) props.onChangeIndex(0);
                    }}
                />
            </div>

            <div className={styles.hr} />

            <div className={styles.listContainer}>

                {
                    servers.map((server) => {
                        return (
                            <div className={styles.listItem} key={server.id}>
                                {
                                    selectLocation === server.id
                                    ?
                                    <div className={styles.focus}></div>
                                    :
                                    null
                                }
                                <div className={styles.focusHover}></div>
                                <img src={server.icon}
                                    onClick={() => {
                                        setSelectLocation(server.id);
                                        if (props.onChangeIndex) props.onChangeIndex(server.id);
                                    }}
                                />
                            </div>
                        )
                    })
                }

            </div>

        </div>
    )
}