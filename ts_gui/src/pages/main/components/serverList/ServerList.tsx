import React from "react";

import styles from "./ServerList.module.scss";

import HomeIcon from "@/assets/icons/home.svg?react";
import gxs from "@/assets/images/servers/gxs.svg";
import nr from "@/assets/images/servers/nr.svg";
import ts from "@/assets/images/servers/ts.svg";
import muilties from "@/assets/images/servers/muilties.svg";

type IProps = {
    onChangeIndex?: (server: number) => void;
}

export default function ServerList(props: IProps) {

    const servers = [
        {
            id: 1,
            name: "GXS2.0 模組伺服器",
            icon: gxs
        },
        {
            id: 2,
            name: "沐緹斯伺服器",
            icon: muilties
        },
        {
            id: 3,
            name: "Nameless Realms 無名伺服器",
            icon: nr
        },
        {
            id: 4,
            name: "TS 模組伺服器",
            icon: ts
        }
    ]

    const [selectLocation, setSelectLocation] = React.useState<number>(1);

    return (
        <div className={styles.serverListContainer}>

            <div className={styles.homeButton} onClick={() => {
                setSelectLocation(0);
                if (props.onChangeIndex) props.onChangeIndex(0);
            }}>
                <HomeIcon
                        className={`${styles.homeImg} ${selectLocation === 0 ? styles.homeButtonFillA238FF : null}`}
                        // onClick={() => {
                        //     setSelectLocation(0);
                        //     if (props.onChangeIndex) props.onChangeIndex(0);
                        // }}
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