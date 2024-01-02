import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { v4 as uuidv4 } from "uuid";
import { useState } from "react";

import styles from "./Menu.module.scss";

import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";

type IProps = {
    instanceId: string;
    onClickMenuButton?: (menuType: number) => void;
}

enum IMenusType {
    Wire,
    Button,
    Title
}
interface IMenus {
    type: IMenusType,
    text: string,
    buttonId?: number,
}

export default function menu(props: IProps) {

    const instanceId = props.instanceId;
    const navigate = useNavigate();
    const { t } = useTranslation();
    const [menuId, setMenuId] = useState(0);

    const onClickMenuButton = (id: number) => {

        if (menuId === id) return;

        setMenuId(id);

        if (props.onClickMenuButton) {
            props.onClickMenuButton(id);
        }

        switch (id) {
            case 0: navigate(`/instanceSettings/${instanceId}/parameters`); break;
            case 1: navigate(`/instanceSettings/${instanceId}/mod_list`); break;
            case 2: navigate(`/instanceSettings/${instanceId}/resource_packs`); break;
            case 3: navigate(`/instanceSettings/${instanceId}/screenshot`); break;
        }
    }

    const menus: Array<IMenus> = [
        {
            type: IMenusType.Title,
            text: t("instanceSetting.menu.title_1.title")
        },
        {
            type: IMenusType.Button,
            buttonId: 0,
            text: t("instanceSetting.menu.title_1.subTitle_1")
        },
        {
            type: IMenusType.Button,
            buttonId: 1,
            text: t("instanceSetting.menu.title_1.subTitle_2")
        },
        {
            type: IMenusType.Button,
            buttonId: 2,
            text: t("instanceSetting.menu.title_1.subTitle_3")
        },
        {
            type: IMenusType.Wire,
            text: t("setting.menu.title_2.title")
        },
        {
            type: IMenusType.Title,
            text: t("instanceSetting.menu.title_2.subTitle_1")
        },
        {
            type: IMenusType.Button,
            buttonId: 3,
            text: t("instanceSetting.menu.title_2.subTitle_1")
        },
        {
            type: IMenusType.Wire,
            text: t("setting.menu.title_2.title")
        },
        {
            type: IMenusType.Title,
            text: t("instanceSetting.menu.title_4.title")
        },
    ]

    return (
        <div className={styles.menuDiv}>

            <div className={styles.menuContentDiv}>

                <div className={styles.menuContentButtonDiv}>

                    {
                        menus.map(menu => {

                            if (menu.type === IMenusType.Title) {
                                return <h1 key={uuidv4()} className={styles.menuTitle}>{menu.text}</h1>;
                            }

                            if (menu.type === IMenusType.Wire) {
                                return <div key={uuidv4()} className={styles.tr}></div>;
                            }

                            if (menu.type === IMenusType.Button) {
                                return (
                                    <div
                                        key={uuidv4()}
                                        className={`${styles.menuButton} ${menuId === menu.buttonId ? styles.menuButtonActive : null}`}
                                        onClick={() => onClickMenuButton(menu.buttonId ? menu.buttonId : 0)}
                                    >
                                        <h1 className={styles.menuButtonText}>{menu.text}</h1>
                                    </div>
                                );
                            }

                        })
                    }

                    {/* TODO: 待優化 */}
                    <div className={styles.functionButtonDiv}>
                        <ButtonFocus className={styles.functionButton} content={t("instanceSetting.menu.title_4.button_1") as string} />
                        <ButtonFocus className={styles.functionButton} content={t("instanceSetting.menu.title_4.button_2") as string} />
                    </div>

                </div>

            </div>

        </div>
    );
}