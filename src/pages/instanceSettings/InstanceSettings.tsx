import React from "react";
import { useNavigate, useParams } from "react-router-dom";

import styles from "./InstanceSettings.module.scss";

import Menu from "./components/menu/Menu";
import ModList from "./components/modList/ModList";
import ResourcePacks from "./components/resourcePacks/ResourcePacks";
import Screenshot from "./components/screenshot/Screenshot";
import Parameters from "../components/parameters/Parameters";

export default function InstanceSettings() {

    let { serverId } = useParams<{ serverId: string }>();
    const { paramsMenuType } = useParams<{ paramsMenuType: string }>();
    const [menuType, setMenuType] = React.useState(Number(paramsMenuType));

    // TODO
    if (serverId === undefined) {
        serverId = "main-server";
    }

    const navigate = useNavigate();

    const instanceSettingComponent = [
        {
            id: 1,
            component: <Parameters checkbox={true} serverId={serverId} />
        },
        {
            id: 2,
            component: <ModList serverId={serverId} />
        },
        {
            id: 3,
            component: <ResourcePacks serverId={serverId} />
        },
        {
            id: 4,
            component: <Screenshot serverId={serverId} />
        },
        // {
        //     id: 5,
        //     component: <Flx serverId={serverId} />
        // }
    ]

    const backMain = () => {
        // window.electron.io.save();
        navigate("/main");
    }

    return (
        <div className={styles.instanceSettingDiv}>

            <div className={styles.backButtonBorderDiv}>
                <div className={styles.backButton} onClick={backMain}>
                    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none" /><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z" /></svg>
                </div>
            </div>

            <div className={styles.leftDiv}>
                <Menu menuType={menuType} onClickMenuButton={setMenuType} serverId={serverId} />
            </div>

            <div className={styles.rightDiv}>
                {
                    instanceSettingComponent.map((item) => (
                        <React.Fragment>
                            {
                                item.id === menuType ? item.component : null
                            }
                        </React.Fragment>
                    ))
                }
            </div>

        </div>
    );
}
