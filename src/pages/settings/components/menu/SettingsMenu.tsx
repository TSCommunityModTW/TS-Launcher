import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { v4 as uuidv4 } from "uuid";

import styles from "./SettingsMenu.module.scss";

type IProps = {
    onClickMenuButton?: (menuType: number) => void;
    onClickBackButton?: () => void;
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

export default function Menu(props: IProps) {

    const navigate = useNavigate();
    const { t } = useTranslation();
    const [menuId, setMenuId] = useState<number>(0);

    const onClickMenuButton = (id: number) => {

        if (menuId === id) return;

        setMenuId(id);

        if(props.onClickMenuButton) {
            props.onClickMenuButton(id);
        }

        switch(id) {
            case 0: navigate("/settings/general"); break;
            case 1: navigate("/settings/parameters"); break;
            case 2: navigate("/settings/language"); break;
            case 3: navigate("/settings/changelog"); break;
            case 4: navigate("/settings/information"); break;
        }
    }

    const menus: Array<IMenus> = [
        {
            type: IMenusType.Title,
            text: t("setting.menu.title_1.title")
        },
        {
            type: IMenusType.Button,
            buttonId: 0,
            text: t("setting.menu.title_1.subTitle_1")
        },
        {
            type: IMenusType.Button,
            buttonId: 1,
            text: t("setting.menu.title_1.subTitle_2")
        },
        {
            type: IMenusType.Button,
            buttonId: 2,
            text: t("setting.menu.title_1.subTitle_3")
        },
        {
            type: IMenusType.Wire,
            text: t("setting.menu.title_2.title")
        },
        {
            type: IMenusType.Title,
            text: t("setting.menu.title_2.title")
        },
        {
            type: IMenusType.Button,
            buttonId: 3,
            text: t("setting.menu.title_2.subTitle_1")
        },
        {
            type: IMenusType.Button,
            buttonId: 4,
            text: t("setting.menu.title_2.subTitle_2")
        }
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
                </div>

            </div>

        </div>
    );
}