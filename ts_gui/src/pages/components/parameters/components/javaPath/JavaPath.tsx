import { useTranslation } from "node_modules/react-i18next";
import { open } from "@tauri-apps/api/dialog";

import styles from "./JavaPath.module.scss";

import Checkbox from "@/pages/components/checkbox/Checkbox";
import Toggle from "@/pages/components/toggle/Toggle";
import InputIcon from "@/pages/components/inputIcon/InputIcon";

type IProps = {
    type: "instanceSetting" | "setting";
    checked?: boolean;
    value: string;
    toggle: boolean;
    pathChecking: boolean | undefined;
    onChangeJavaToggle?: (state: boolean) => void;
    onChangeJavaPath?: (value: string) => void;
    onChecked?: (state: boolean) => void;
    onClickAutoSearch?: () => void;
    onClickTest?: () => void;
    onClickManualSearched?: (path: string) => void;
}

export default function JavaPath(props: IProps) {

    const { t } = useTranslation();

    return (
        <div className={styles.javaPathDiv}>

            {
                props.type === "setting"
                    ?
                    <h1>{t("common.parameters.javaPath.type.setting.text")}</h1>
                    :
                    <div className={styles.titleDiv}>

                        <Checkbox
                            content={t("common.parameters.javaPath.type.instanceSetting.checkbox.text")} className={styles.checkbox}
                            checked={props.type === "instanceSetting" ? props.checked || false : false}
                            onClickChecked={(state) => {

                                if (props.onChecked === undefined) return;
                                props.onChecked(state);

                            }}
                        />

                        {
                            props.type === "instanceSetting" ? props.checked ? null : <h1>{t("common.parameters.javaPath.type.instanceSetting.text")}</h1> : null
                        }

                    </div>
            }
            <div
                className={styles.javaPathContainer}
                style={props.type === "instanceSetting" ? !props.checked ? { filter: "grayscale(1)", WebkitFilter: "grayscale(1)" } : undefined : undefined}
            >

                {
                    props.type === "instanceSetting" ? !props.checked ? <div className={styles.disabledDiv}></div> : null : null
                }

                <div className={styles.toggleBuiltInJavaDiv}>
                    <div className={styles.leftDiv}>
                        <h1>{t("common.parameters.javaPath.toggleBuiltInJava.title")}</h1>
                    </div>
                    <div className={styles.rightDiv}>
                        <Toggle className={styles.toggle} state={props.toggle} onChange={(state) => {
                            if (props.onChangeJavaToggle) props.onChangeJavaToggle(state);
                        }} />
                    </div>
                </div>

                <div
                    className={styles.inputTestJavaContainer}
                    style={props.toggle ? { filter: "grayscale(1)", WebkitFilter: "grayscale(1)" } : undefined}
                >

                    {
                        props.toggle ? <div className={styles.toggleInputPathDisabledDiv}></div> : null
                    }

                    <InputIcon className={styles.inputIcon} type="text" icon="java" value={props.value} onChange={(value) => {

                        if (props.onChangeJavaPath === undefined) return;
                        props.onChangeJavaPath(value);

                    }} />

                    <div className={styles.stateButtonDiv}>

                        <div className={styles.leftDiv}>
                            <h1>狀態:</h1>
                            {
                                props.pathChecking !== undefined ? props.pathChecking ? <h1>{t("common.parameters.javaPath.stateButtons.state.text_1")}</h1> : <h1>{t("common.parameters.javaPath.stateButtons.state.text_2")}</h1> : <h1>{t("common.parameters.javaPath.stateButtons.state.text_3")}</h1>
                            }
                        </div>
                        <div className={styles.rightDiv}>

                            <button className={styles.testButton} onClick={() => {

                                if (props.onClickTest) props.onClickTest();

                            }}>{t("common.parameters.javaPath.stateButtons.buttons.button_1.text")}</button>

                            {/* <button onClick={() => {

                            if (props.onClickAutoSearch) props.onClickAutoSearch();

                        }}>{t("common.parameters.javaPath.stateButtons.buttons.button_2.text")}</button> */}

                            <button
                                onClick={async () => {

                                    let filePath = await open({
                                        multiple: false
                                    });

                                    if (props.onClickManualSearched && filePath) props.onClickManualSearched(filePath as string);

                                }}>
                                {t("common.parameters.javaPath.stateButtons.buttons.button_3.text")}
                            </button>

                        </div>

                    </div>
                </div>

            </div>
        </div>
    );
}