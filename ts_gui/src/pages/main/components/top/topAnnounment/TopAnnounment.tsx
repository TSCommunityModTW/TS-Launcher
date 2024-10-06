import { useState } from "react";

import styles from "./TopAnnounment.module.scss"
import download from "@/assets/icons/download.png"

type Announment = {
    title: string;
    message: string;
}

type IProps = {
    needUpdate: boolean;
    announment: Array<Announment>
}

export default function TopAnnounment(props: IProps) {
    let AnnounmentTitle = null;


    const [isExpanded, setIsExpanded] = useState(false);

    const toggleExpand = () => {
        setIsExpanded(!isExpanded);
    };
    if (props.needUpdate) {
        AnnounmentTitle = "當前TS-Luncher版本有可用更新!!!";
    }
    else if (props.announment[0]) {
        AnnounmentTitle = props.announment[0].title;
    }
    else {
        AnnounmentTitle = "當前沒有公告。";
    }

    return (
        <div className={styles.WidgetContainer}>
            <div className={`${styles.announmentContainer} ${isExpanded ? styles.expanded : styles.notexpanded}`} onClick={toggleExpand}>
                <h1><span>公告：</span><span>{AnnounmentTitle}</span></h1>
                <div className={`${styles.messages} ${isExpanded ? styles.show : styles.hide}`}>
                    {props.announment.map((msg, index) => (
                        <p key={index}>{msg.message}</p>
                    ))}
                </div>

            </div>
            {props.needUpdate &&
                <div className={styles.downloadDiv}>
                    <img className={styles.downloadLogo} src={download}></img>
                </div>
            }

        </div>


    )

}