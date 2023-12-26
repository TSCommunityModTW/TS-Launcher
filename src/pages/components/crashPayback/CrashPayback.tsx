import { useAppSelector, useAppDispatch } from "@/hooks";
import ButtonFocus from "../buttonFocus/ButtonFocus";
import styles from "./CrashPayback.module.scss";
import { setCrashOpen } from "@/slices/stateSlice";

export default function CrashPayback() {

    const dispatch = useAppDispatch();
    const crashOpenState = useAppSelector((state) => state.stateSlice.crashOpen);

    if (crashOpenState.state) {
        return (
            <div className={styles.crashPaybackWrapper}>
    
                <div className={styles.crashPaybackContainer}>
    
                    <h1 className={styles.titleDiv}>
                        TS Launcher Error !!!
                    </h1>
    
                    <div className={styles.textContainer}>
                        <h1 className={styles.description}>{crashOpenState.errorMessage}</h1>
                    </div>
    
                    <div className={styles.buttonContainer}>
                        <ButtonFocus
                            className={styles.buttonFocus}
                            content="Close"
                            onClick={() => {
                                dispatch(setCrashOpen({ state: false }));
                            }}
                        />
                    </div>
    
                </div>
    
            </div>
        )
    } else {
        return null;
    }
}