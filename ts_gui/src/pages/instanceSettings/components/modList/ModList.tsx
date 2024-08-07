import React from "react";
import { useTranslation } from "react-i18next";

import styles from "./ModList.module.scss";

import Mod from "./components/mod/Mod";
import AlertConfirm from "@/pages/components/alertConfirm/AlertConfirm";
import InputIcon from "@/pages/components/inputIcon/InputIcon";
import ButtonFocus from "@/pages/components/buttonFocus/ButtonFocus";
import Checkbox from "@/pages/components/checkbox/Checkbox";

import logger from "@/invoke/logger";


export default function ModList() {

    const { t } = useTranslation();


    const [disabledDiv, _setDisabledDiv] = React.useState<boolean>(false);
    const [searchValue, setSearchValue] = React.useState("");
    const [enableCheckbox, setEnableCheckbox] = React.useState(true);
    const [disableCheckbox, setDisableCheckbox] = React.useState(true);
    const hiddenFileInput = React.useRef<any>(null);
    const [hiddenAlertConfirm, setHiddenAlertConfirm] = React.useState(false);
    const [mods, _setMods] = React.useState<Array<{ fileName: string; filePath: string; state: boolean; hidden: boolean }>>();

    const [alertConfirmTitle, setAlertConfirmTitle] = React.useState("");
    const [alertConfirmDescription, setAlertConfirmDescription] = React.useState("");
    const [_deleteFilePath, setDeleteFilePath] = React.useState("");

    React.useEffect(() => {
        // const mods = window.electron.game.module.getModules(props.serverId);
        // if (mods.length > 0) setMods(window.electron.game.module.getModules(props.serverId));
    }, []);

    const handleChange = (event: any) => {
        for (let _file of event.target.files) {
            logger.logMessage("info",`[Mod List] handleChange - File Name: ${_file.name}`)
             //window.electron.game.module.copyModuleFile({ name: file.name, path: file.path }, props.serverId);
        }
        //setMods(window.electron.game.module.getModules(props.serverId));
    };

    const search = (_searchValue: string) => {

        if (mods === undefined) return;

        // const modules = window.electron.game.module.getModules(props.serverId);

        // if (search !== undefined && search.length !== 0) {
        //     for (let i = 0; i < modules.length; i++) {

        //         modules[i].hidden = true;

        //         if (modules[i].fileName.toLowerCase().indexOf(searchValue.toLowerCase()) === -1) {
        //             modules[i].hidden = false;
        //         }
        //     }
        // } else {
        //     for (let i = 0; i < modules.length; i++) {
        //         modules[i].hidden = true;
        //     }
        // }

        // setMods(modules);
    }

    const onFilterClick = (_enableCheckboxState: boolean, _disableCheckboxState: boolean) => {

        // let state = -1;

        // if (enableCheckboxState) state = 0;
        // if (disableCheckboxState) state = 1;
        // if (enableCheckboxState && disableCheckboxState) state = 2;

        // if (mods === undefined) return;

        // const modules = window.electron.game.module.getModules(props.serverId);

        // for (let i = 0; i < modules.length; i++) {

        //     modules[i].hidden = false;

        //     switch (state) {
        //         case 0:
        //             if (modules[i].state) modules[i].hidden = true;
        //             break;
        //         case 1:
        //             if (!modules[i].state) modules[i].hidden = true;
        //             break;
        //         case 2:
        //             modules[i].hidden = true;
        //             break;
        //     }
        // }

        // setMods(modules);
    }

    return (
        <div className={styles.modListDiv}>

            {
                disabledDiv
                    ?
                    <div className={styles.disabledDiv}>
                        <div className={styles.disabledBackgroundDiv}>
                            <h1 className={styles.disabledTitle}>{t("instanceSetting.components.modList.disabled.title")}</h1>
                        </div>
                    </div> : null
            }

            {
                hiddenAlertConfirm ? <AlertConfirm title={alertConfirmTitle} description={alertConfirmDescription} onCancelClick={() => setHiddenAlertConfirm(false)} onConfirmClick={() => {

                    // window.electron.game.module.moduleDelete(deleteFilePath);
                    setHiddenAlertConfirm(false);
                    // setMods(window.electron.game.module.getModules(props.serverId));

                }} /> : null
            }

            <h1 className={styles.headline}>{t("instanceSetting.menu.title_1.subTitle_2")}</h1>

            <div className={styles.searchButtonDiv}>

                <InputIcon className={styles.inputIcon} type="text" icon="search" value={searchValue} onChange={(value) => {

                    setSearchValue(value);
                    search(value);

                }} />
                <ButtonFocus content={t("instanceSetting.components.modList.searchButton.button_1.title") as string} className={styles.buttonFocus} onClick={() => hiddenFileInput.current.click()} />
                <input type="file" ref={hiddenFileInput} onChange={(event) => {
                    handleChange(event);
                    event.target.value = "";
                }} style={{ display: "none" }} multiple />

            </div>

            <div className={styles.textFilterDiv}>

                <div className={styles.leftDiv}>
                    <h1>{t("instanceSetting.components.modList.list.title_1")}</h1>
                    <h2>{t("instanceSetting.components.modList.list.title_2")}</h2>
                </div>

                <div className={styles.rightDiv}>
                    <h1 className={styles.text}>{t("instanceSetting.components.modList.list.filter.title")}</h1>
                    <Checkbox className={styles.checkbox} content={t("instanceSetting.components.modList.list.filter.checkbox_1.title")} checked={enableCheckbox} onClickChecked={(state) => {
                        onFilterClick(state, disableCheckbox);
                        setEnableCheckbox(state);
                    }} />
                    <Checkbox className={styles.checkbox} content={t("instanceSetting.components.modList.list.filter.checkbox_2.title")} checked={disableCheckbox} onClickChecked={(state) => {
                        onFilterClick(enableCheckbox, state);
                        setDisableCheckbox(state);
                    }} />
                </div>

            </div>

            <div className={styles.listDiv}>
                {
                    mods !== undefined
                        ?
                        mods.map((item) => (
                            <div>
                                {
                                    item.hidden ? <Mod fileName={item.fileName} filePath={item.filePath} state={item.state} serverId={""} onDeleteClick={(fileName, filePath) => {
                                        setAlertConfirmTitle(t("instanceSetting.components.modList.list.hiddenAlertConfirm.title"));
                                        setAlertConfirmDescription(`${t("instanceSetting.components.modList.list.hiddenAlertConfirm.descriptionsSplit.split_1")}: ${fileName} ${t("instanceSetting.components.modList.list.hiddenAlertConfirm.descriptionsSplit.split_1")}`);
                                        setDeleteFilePath(filePath);
                                        setHiddenAlertConfirm(true);
                                    }} /> : null
                                }
                            </div>
                        ))
                        :
                        <div className={styles.notModsDiv}>
                            <h1>{t("instanceSetting.components.modList.list.notMods.title")}</h1>
                        </div>
                }
            </div>

        </div>
    );
}

// function isGameStart(serverId: string): boolean {

//     const instance = window.electron.game.instance;
//     const state = instance.getState(serverId);

//     if (state === "onStandby" || state === "close" || state === "closeError" || state === "startError") {
//         return false;
//     }

//     return true;
// }