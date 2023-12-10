import React from "react";
import styles from "./Loading.module.scss";
import logoImg from "@/assets/images/logo/logo.png";

import { animated, useTrail, config } from "react-spring";

type IProps = {
    text?: string;
}

export default function Loading(props: IProps) {

    const titleTexts = "模組伺服器社群".split("");
    const [open, setOpen] = React.useState(true);

    const trail = useTrail(titleTexts.length + 1, {
        config: config.gentle,
        opacity: open ? 1 : 0.2,
        x: open ? 0 : 10,
        scale: open ? 1.2 : 1,
        from: {
            opacity: 0.2,
            x: 10,
            scale: 1.2
        }
    });

    // const fadeInProps = useSpring({
    //     opacity: 1,
    //     from: { opacity: 0 },
    //     config: { duration: 3000 },
    // });

    React.useEffect(() => {

        let isCancelled = false;

        setTimeout(() => {
            if(!isCancelled) setOpen((value) => !value);
        }, 1500);

        return () => {
            isCancelled = true;
        };
    }, [open]);

    return (
        <div className={styles.loadingContainer}>
            <div className={styles.loadingTitleContainer}>
                <img src={logoImg} alt="logo" />
                {
                    trail.map(({ ...style }, index) => (
                        <animated.div
                            key={index}
                            style={style}
                        >
                            <span>{titleTexts[index]}</span>
                        </animated.div>
                    ))
                }
            </div>
            <div className={styles.loadingText}>
                <h1>{props.text}</h1>
                {/* <animated.div className="loadingPoint" style={fadeInProps}>
                    <p>.</p>
                    <p>.</p>
                    <p>.</p>
                </animated.div> */}
            </div>
        </div>
    );
}