import styles from "./ButtonFocus.module.scss";

type IProps = {
    content: string | number;
    disabled?: boolean;
    className?: string;
    onClick?: () => void;
    themeColor?: "green" | "purple" | "gray"
}

export default function ButtonFocus(props: IProps) {

    const componentStyles = () => {

        const styles = [];

        if (props.disabled) {
            styles.push({ cursor: "not-allowed" });
        } else {
            styles.push({ cursor: "pointer" });
        }

        return styles.reduce((acc, style) => ({ ...acc, ...style }), {});
    }

    const componentClassNames = () => {

        const classNames = [];

        classNames.push(styles.buttonFocusContainer);
        classNames.push(props.className);

        const themeColor = props.themeColor ? props.themeColor : "purple";

        switch (themeColor) {
            case "purple": {
                classNames.push(styles.purple);
                break;
            }
            case "green": {
                classNames.push(styles.green);
                break;
            }
            case "gray": {
                classNames.push(styles.gray);
                break;
            }
        }

        return classNames.join(" ");
    }

    return (
        <div className={`${componentClassNames()}`}>
            <button
                style={componentStyles()}
                onClick={() => {if (props.onClick !== undefined && (props.disabled !== undefined ? !props.disabled : true)) props.onClick()}}
            >{props.content}</button>
        </div>
    );
}