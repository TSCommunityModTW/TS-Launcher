import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useLoaderData, useParams } from "react-router-dom";

import styles from "./Parameters.module.scss";

import JavaParameter from "./components/javaParameter/JavaParameter";
import JavaPath from "./components/javaPath/JavaPath";
import Ram from "./components/ram/Ram";
import Store from "@/invoke/store";
import Java from "@/invoke/java";
import { IStoreSettingsJava } from "@/interfaces/IStoreSettingsJava";
import { useAppDispatch } from "@/hooks";
import { setCrashOpen } from "@/slices/stateSlice";

type IProps = {
    checkbox: boolean;
}

interface ILoaderData {
    storeSettingsJava: IStoreSettingsJava,
    systemMaxMemorySize: number
}

export default function Parameters(props: IProps) {

    const { instanceId } = useParams<{ instanceId: string }>();
    const { t } = useTranslation();
    const dispatch = useAppDispatch();

    const loaderData = useLoaderData() as ILoaderData;
    const storeSettingsJava = loaderData.storeSettingsJava;
    const systemMaxMemorySize = loaderData.systemMaxMemorySize;

    const id = props.checkbox ? instanceId === undefined ? "global" : instanceId : "global";
    const isCheckbox = props.checkbox ? "instanceSetting" : "setting";

    const ramTotal = systemMaxMemorySize / 1024;

    const [ramMax, setRamMax] = useState(storeSettingsJava.ram_max_size / 1024);
    const [ramMin, setRamMin] = useState(storeSettingsJava.ram_min_size / 1024);
    const [javaPath, setJavaPath] = useState(storeSettingsJava.java_path);
    const [javaParameter, setJavaParameter] = useState(storeSettingsJava.java_parameter);

    const [javaPathChecking, setJavaPathChecking] = useState<boolean | undefined>(undefined);

    const [javaInJavaVMToggle, setJavaInJavaVMToggle] = useState(storeSettingsJava.is_built_in_java_vm);
    const [ramChecked, setRamChecked] = useState(id !== "global" ? storeSettingsJava.ram_checked : false);
    const [javaPathChecked, setJavaPathChecked] = useState(id !== "global" ? storeSettingsJava.java_path_checked : false);
    const [javaParameterChecked, setJavaParameterChecked] = useState(id !== "global" ? storeSettingsJava.java_parameter_checked : false);

    useEffect(() => {

        if (ramMax > ramTotal) {
            setRamMax(ramTotal);
            return;
        }

        if (ramMin * 1024 >= ramMax * 1024) setRamMin(ramMax);

        storeSettingsJava.ram_max_size = ramMax * 1024;
        Store.setSettingsJava(id, storeSettingsJava);

    }, [ramMax]);

    useEffect(() => {

        if (ramMin <= 0) {
            setRamMin(1);
            return;
        }

        if (ramMin * 1024 >= ramMax * 1024) setRamMax(ramMin);

        storeSettingsJava.ram_min_size = ramMin * 1024;
        Store.setSettingsJava(id, storeSettingsJava);

    }, [ramMin]);

    useEffect(() => {

        storeSettingsJava.java_path = javaPath;
        storeSettingsJava.java_parameter = javaParameter;
        storeSettingsJava.is_built_in_java_vm = javaInJavaVMToggle;

        Store.setSettingsJava(id, storeSettingsJava);

    }, [javaPath, javaParameter, javaInJavaVMToggle]);

    if (id !== "global") {
        useEffect(() => {

            storeSettingsJava.ram_checked = ramChecked;
            storeSettingsJava.java_path_checked = javaPathChecked;
            storeSettingsJava.java_parameter_checked = javaParameterChecked;

            Store.setSettingsJava(id, storeSettingsJava);

        }, [ramChecked, javaPathChecked, javaParameterChecked]);
    }

    return (
        <div className={styles.parametersDiv}>

            <h1 className={styles.headline}>{t("instanceSetting.menu.title_1.subTitle_1")}</h1>

            <Ram
                type={isCheckbox}
                ramChecked={ramChecked}
                ramMax={ramMax}
                ramMin={ramMin}
                onRamMaxChange={setRamMax}
                onRamMinChange={setRamMin}
                onRamChecked={setRamChecked}
            />
            <JavaPath
                type={isCheckbox}
                checked={javaPathChecked}
                value={javaPath}
                toggle={javaInJavaVMToggle}
                pathChecking={javaPathChecking}
                onChangeJavaToggle={setJavaInJavaVMToggle}
                onChangeJavaPath={setJavaPath}
                onChecked={setJavaPathChecked}
                onClickTest={async () => {

                    if (javaPath.length < 0) {
                        return;
                    }

                    Java.test_jar(javaPath)
                        .then((v) => setJavaPathChecking(v))
                        .catch((error) => {
                            dispatch(setCrashOpen({ state: true, errorMessage: error.message ? error.message : error }));
                        });

                }}
                // onClickAutoSearch={async () => {
                //     // const javaPath = await os.java.getPath();
                //     // setJavaPath(javaPath);
                //     setJavaPathChecking(undefined);
                // }}
                onClickManualSearched={(path) => {
                    setJavaPath(path);
                    setJavaPathChecking(undefined);
                }}
            />
            <JavaParameter
                type={isCheckbox}
                checked={javaParameterChecked}
                value={javaParameter}
                onChangeJavaParameter={setJavaParameter}
                onChecked={setJavaParameterChecked}
            />
        </div>
    );
}