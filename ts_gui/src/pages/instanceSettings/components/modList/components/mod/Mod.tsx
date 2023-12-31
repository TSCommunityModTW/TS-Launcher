import React from "react";

import styles from "./Mod.module.scss";

import deleteForeverIcon from "@/assets/icons/delete-forever.png";
import Toggle from "@/pages/components/toggle/Toggle";

type IProps = {
    fileName: string;
    filePath: string;
    state: boolean;
    serverId: string;
    onDeleteClick: (fileName: string, filePath: string) => void;
}

export default function Mod(props: IProps) {

    const [state, setState] = React.useState(props.state);
    const [filePath, _setFilePath] = React.useState(props.filePath);

    return (
        <div className={styles.itemDiv}>

            <div className={styles.leftDiv}>
                <div className={styles.outerCircle}>
                    {
                        state ? <div className={styles.innerCircle}></div> : null
                    }
                </div>
                <h1>{props.fileName}</h1>
            </div>

            <div className={styles.rightDiv}>
                <Toggle className={styles.toggle} state={state} onChange={(state) => {

                    setState(state);
                    // setFilePath((filePath) => {
                    //     return window.electron.game.module.moduleEnableDisable(filePath, state, props.serverId);
                    // });

                }} />
                <img src={deleteForeverIcon} alt="delete-forever" onClick={() => props.onDeleteClick(props.fileName, filePath)} />
            </div>

        </div>
    )
}