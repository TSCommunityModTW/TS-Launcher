import styles from "./Toggle.module.scss";

type IProps = {
    state: boolean;
    onChange: (state: boolean) => void;
    className?: string;
}

export default function Toggle(props: IProps) {

    // const [toggleOffDivClassName, setToggleOffDivClassName] = React.useState(props.state ? styles.toggleOnDiv : "");

    return (
        <div className={`${styles.toggleDiv} ${props.state ? styles.toggleOnDiv : null} ${props.className}`} onClick={() => {

            // setToggleOffDivClassName(toggleOffDivClassName === styles.toggleOnDiv ? "" : styles.toggleOnDiv);
            props.onChange(!props.state)

        }}>
            <div className={styles.innerCircle}></div>
        </div>
    );
}