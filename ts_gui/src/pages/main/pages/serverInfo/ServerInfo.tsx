import { useLoaderData, useNavigate } from "react-router-dom";
import { v4 as uuidv4 } from "uuid";

import styles from "./ServerInfo.module.scss";

import Trail from "@/pages/components/trail/Trail";
import settingLines from "@/assets/icons/setting-lines.png";
import ButtonPlay from "../../components/buttonPlay/ButtonPlay";
import { ILauncherAssetsServer } from "@/interfaces/ILauncherAssetsServer";
import { useEffect, useState } from "react";
import { ILauncherAssetsServerChildren } from "@/interfaces/ILauncherAssetsServerChildren";
import ChildrenServer from "../../components/childrenServer/ChildrenServer";

export default function ServerInfo() {

    const navigate = useNavigate();

    const server = useLoaderData() as ILauncherAssetsServer;
    const childrenServers = server.children;

    const [titleTrail, setTitleTrail] = useState<boolean>(true);
    const [selectChildrenServerId, setSelectChildrenServerId] = useState<string>(childrenServers[0].id);
    const [barChildrenServers, setBarChildrenServers] = useState<any>();

    const childrenServer: (id: string) => ILauncherAssetsServerChildren | undefined = (id) => {
        return childrenServers.find(childrenServer => childrenServer.id === id);
    }

    useEffect(() => {

        // setTitleTrail(false);
        // setTimeout(() => {
        //     setTitleTrail(true);
        // }, 200)

        setSelectChildrenServerId(childrenServers[0].id);

    }, [server]);

    const getBarValue = (serverId: string): number => {
        if (barChildrenServers) {

            let bar = barChildrenServers.find((bar: { fraction: number, serverId: string }) => bar.serverId === serverId);

            if (bar) {
                return bar.fraction;
            }
        }

        return 0;
    }

    return (
        <div className={styles.serverInfoContainer}>

            <div className={styles.scroll}>
                <div className={styles.playServerGameContainer}>

                    <div className={styles.leftDiv}>
                        <Trail open={titleTrail}>
                            <h1 className={styles.serverNameH1}>{server.name}</h1>
                            <h2 className={styles.serverNameH2}>{childrenServer(selectChildrenServerId)?.name}</h2>
                            <div className={styles.buttonDiv}>
                                <div className={styles.settingButton} onClick={() => {
                                    navigate(`/instanceSettings/${selectChildrenServerId}/parameters`);
                                }}>
                                    <img src={settingLines} alt="setting-lines" />
                                </div>

                                <ButtonPlay
                                    serverId={server.id}
                                    childrenServerId={selectChildrenServerId}
                                />

                            </div>
                        </Trail>
                    </div>

                    <div className={styles.modpackContainer}>
                        <Trail open={true}>
                            <img src={server.imageUrl} />
                        </Trail>
                    </div>

                </div>

                <div className={styles.childServerList}>
                    {
                        childrenServers.map(item => {
                            return <ChildrenServer
                                key={uuidv4()}
                                serverId={item.id}
                                imageUrl={item.imageUrl}
                                name={item.name}
                            />
                        })
                    }
                </div>
            </div>

        </div>
    )
}