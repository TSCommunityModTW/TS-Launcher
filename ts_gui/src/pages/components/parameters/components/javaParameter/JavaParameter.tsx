import { useTranslation } from "react-i18next";

import styles from "./JavaParameter.module.scss";

import Checkbox from "@/pages/components/checkbox/Checkbox";

type IProps = {
    type: "instanceSetting" | "setting";
    checked?: boolean;
    value: string;
    onChangeJavaParameter?: (value: string) => void;
    onChecked?: (state: boolean) => void;
}

export default function JavaParameter(props: IProps) {

    const { t } = useTranslation();

    return (
        <div className={styles.JavaParameterDiv}>

            {
                props.type === "setting"
                    ?
                    <h1>{t("common.parameters.javaParameter.type.setting.text")}</h1>
                    :
                    <div className={styles.titleDiv}>

                        <Checkbox content={t("common.parameters.javaParameter.type.instanceSetting.checkbox.text")} className={styles.checkbox} checked={props.type === "instanceSetting" ? props.checked || false : false} onClickChecked={(state) => {

                            if (props.onChecked === undefined) return;
                            props.onChecked(state);

                        }}
                        />

                        {
                            props.type === "instanceSetting" ? props.checked ? null : <h1>{t("common.parameters.javaParameter.type.instanceSetting.text")}</h1> : null
                        }

                    </div>
            }

            {/* <h1>參數</h1> */}
            <div className={styles.javaParameterContainer}>

                {
                    props.type === "instanceSetting" ? !props.checked ? <div className={styles.disabledDiv}></div> : null : null
                }

                <textarea
                    value={props.value}
                    onChange={(event) => {
                        if (props.onChangeJavaParameter) props.onChangeJavaParameter(event.target.value);
                    }}
                ></textarea>

            </div>

        </div>
    );
}