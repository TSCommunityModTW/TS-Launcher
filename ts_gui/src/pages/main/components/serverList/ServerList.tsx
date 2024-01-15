import { useState } from "react";
import { useLoaderData, useNavigate } from "react-router-dom";

import styles from "./ServerList.module.scss";

import HomeIcon from "@/assets/icons/home.svg?react";
import { IMainLoader } from "@/loader";

export default function ServerList() {

    const navigate = useNavigate();
    const { servers } = useLoaderData() as IMainLoader;

    const [selectLocation, setSelectLocation] = useState<string>("home");

    const onClickMenuButton = (serverId: string) => {
        navigate(`/main/${serverId}/server_info`);
    }

    return (
        <div className={styles.bgbox}>

                <div className={styles.serverListContainer}>

                    <div className={styles.homeButton} onClick={() => {
                        setSelectLocation("home");
                    }}>
                        <HomeIcon
                                className={`${styles.homeImg} ${selectLocation === "home" ? styles.homeButtonFillA238FF : null}`}
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
                                        <img 
                                            rel="preload"
                                            src={server.imageUrl}
                                            onClick={() => {
                                                setSelectLocation(server.id);
                                                onClickMenuButton(server.id);
                                            }}
                                        />
                                    </div>
                                )
                            })
                        }

                    </div>

                </div>
        </div>
    )
}