import React, { useEffect } from "react";
// import { useTranslation } from "react-i18next";

import styles from "./ButtonPlay.module.scss";
import playImg from "@/assets/icons/play.png";
import stopImg from "@/assets/icons/stop.png";
import supportImg from "@/assets/icons/support.png";
import Process from "@/invoke/process";
import { useAppDispatch } from "@/hooks";
import { setCrashOpen } from "@/slices/stateSlice";
import { loading_listener } from "@/invoke/events";

type IPlayState = "onStandby" | "validate" | "start" | "startError" | "close" | "closeError" | "validateFlx" | "stop" | "flxStop";
type IProps = {
    serverId: string;
    childrenServerId: string;
    onCrashClick?: (code: number, description: string) => void;
}

export default function ButtonPlay(props: IProps) {

    // const { t } = useTranslation();
    const dispatch = useAppDispatch();
    const [playState, _setPlayState] = React.useState<IPlayState>("onStandby");
    const [progressBar, _setProgressBar] = React.useState(100);
    const [playText, _setPlayText] = React.useState<string>("開始遊戲");
    const [playColor, _setPlayColor] = React.useState<"#0a9850" | "#3183E1" | "#ED4245">("#0a9850");
    const [playPadding, _setPlayIconPadding] = React.useState<15 | 30>(30);

    useEffect(() => {
        init();
    }, []);

    const init = async () => {

        // await loading_listener((payload: any) => {
        //     let fraction = Math.round(payload.fraction);
        // });

    }

    return (
        <div className={styles.buttonPlayDiv} style={{ padding: `0px ${playPadding}px` }}
            onClick={() => {
                Process.processMinecraftRun(props.serverId, props.childrenServerId)
                    .catch((err) => {
                        dispatch(setCrashOpen({ state: true, errorMessage: err.message }));
                    });
            }}
        >
            <div className={styles.playButtonBackground} style={{ width: `${progressBar}%`, backgroundColor: playColor }}></div>
            <h1>{playText}</h1>
            {
                playState === "validateFlx" || playState === "flxStop"
                    ?
                    <img style={{ right: `${playPadding}px` }} src={supportImg} />
                    :
                    playState === "onStandby" ? <img style={{ right: `${playPadding}px` }} src={playImg} /> : <img style={{ right: `${playPadding}px` }} src={stopImg} />
            }

        </div>
    );
}