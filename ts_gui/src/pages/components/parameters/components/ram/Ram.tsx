import { useTranslation } from "node_modules/react-i18next";

import styles from "./Ram.module.scss";

import Checkbox from "@/pages/components/checkbox/Checkbox";
import Slider from "@/pages/components/slider/Slider";
import { useEffect, useState } from "react";
import System from "@/invoke/system";

type IProps = {
    type: "instanceSetting" | "setting";
    ramChecked?: boolean;
    ramMax: number,
    ramMin: number
    onRamMaxChange?: (value: number) => void;
    onRamMinChange?: (value: number) => void;
    onRamChecked?: (state: boolean) => void;
}

export default function Ram(props: IProps) {

    const { t } = useTranslation();
    const [ramTotal, setRamTotal] = useState(0);
    const [ramFree, setRamFree] = useState(0);

    const setRamMax = (value: number) => {
        if (props.onRamMaxChange === undefined) return;
        props.onRamMaxChange(value);
    }

    const setRamMin = (value: number) => {
        if (props.onRamMinChange === undefined) return;
        props.onRamMinChange(value);
    }

    useEffect(() => {

        init();
        let intervalRamFreeId = setInterval(intervalSetRamFree, 1000);

        return () => {
            clearInterval(intervalRamFreeId);
        }

    }, [])

    const init = async () => {
        setRamTotal(Math.round(await System.getMaxMemorySize() / 1024));
        intervalSetRamFree();
    }

    const intervalSetRamFree = async () => {
        setRamFree(Math.round(await System.getFreeMemorySize() / 1024));
    }

    return (
        <div className={styles.ramDiv}>

            {
                props.type === "setting"
                    ?
                    <h1>{t("common.parameters.ram.type.setting.text")}</h1>
                    :
                    <div className={styles.titleDiv}>

                        <Checkbox
                            content={t("common.parameters.ram.type.instanceSetting.checkbox.text")} className={styles.checkbox}
                            checked={props.type === "instanceSetting" ? props.ramChecked || false : false}
                            onClickChecked={(state) => {
                                if (props.onRamChecked) props.onRamChecked(state);
                            }}
                        />

                        {
                            props.type === "instanceSetting" ? props.ramChecked ? null : <h1>{t("common.parameters.ram.type.instanceSetting.text")}</h1> : null
                        }

                    </div>
            }
            
            <div
                className={styles.ramContainerDiv}
                style={props.type === "instanceSetting" ? !props.ramChecked ? { filter: "grayscale(1)", WebkitFilter: "grayscale(1)" } : undefined : undefined}
            >
                {
                    props.type === "instanceSetting" ? !props.ramChecked ? <div className={styles.disabledDiv}></div> : null : null
                }

                <div className={styles.leftDiv}>
                    <div className={styles.ramMaxInputDiv}>
                        <h1>{t("common.parameters.ram.input.ramMax.text")}</h1>
                        <input type="number" value={props.ramMax} onChange={(event) => setRamMax(Number(event.target.value))} />
                        <h2>GB</h2>
                    </div>
                    <Slider min={1} max={ramTotal} value={props.ramMax} onChange={setRamMax} />
                    <div className={styles.ramMinInputDiv}>
                        <h1>{t("common.parameters.ram.input.ramMin.text")}</h1>
                        <input type="number" value={props.ramMin} onChange={(event) => setRamMin(Number(event.target.value))} />
                        <h2>GB</h2>
                    </div>
                    <Slider min={1} max={ramTotal} value={props.ramMin} onChange={setRamMin} />
                </div>

                <div className={styles.rightDiv}>
                    <div className={styles.ramNumberContainerDiv}>
                        <h1>{t("common.parameters.ram.ramFree.text")}</h1>
                        <div className={styles.tr}></div>
                        <div className={styles.ramNumberDiv}>
                            <h1>{ramFree}</h1>
                            <h2>GB</h2>
                        </div>
                    </div>
                    <div className={styles.ramNumberContainerDiv}>
                        <h1>{t("common.parameters.ram.ramTotal.text")}</h1>
                        <div className={styles.tr}></div>
                        <div className={styles.ramNumberDiv}>
                            <h1>{ramTotal}</h1>
                            <h2>GB</h2>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    );
}