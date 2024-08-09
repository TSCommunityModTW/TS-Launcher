import React, { useState, useEffect, useRef } from 'react';
import styles from '../widgetContect/WidgetContent.module.scss';
import { ILauncherAssetsServerChildren } from "@/interfaces/ILauncherAssetsServerChildren";
import ReactSkinview3d from 'react-skinview3d';
import { WalkingAnimation } from "skinview3d";

interface WidgetContentProps {
    selectedButton: string | null;
    serverChild: ILauncherAssetsServerChildren | undefined;
}

const skins = [
    { id: 1, name: 'Skin 1', imageUrl: 'path/to/skin1.png' },
    { id: 2, name: 'Skin 2', imageUrl: 'path/to/skin2.png' },
];

const WidgetContent: React.FC<WidgetContentProps> = ({ selectedButton, serverChild }) => {
    const [selectedSkin, setSelectedSkin] = useState<number | null>(null);
    const skinViewerRefs = useRef<(HTMLDivElement | null)[]>([]);

    useEffect(() => {
        skinViewerRefs.current.forEach(skinViewer => {
            if (skinViewer) {
            }
        });
    }, [selectedSkin]);

    if (!serverChild) return null;

    const applySkin = () => {
    };

    switch (selectedButton) {
        case 'home':
            return (
                <div className={styles.serverBorderContainer}>
                    <img className={styles.serverBorderContainer} src={serverChild.imageUrl} alt="Server" />
                </div>
            );
        case 'skin':
            return (
                <div className={styles.skinContainer}>
                    <div className={styles.skinGrid}>
                        {skins.map((skin, index) => (
                            <div
                                key={skin.id}
                                className={`${styles.skinItem} ${selectedSkin === skin.id ? styles.selected : ''}`}
                                onClick={() => setSelectedSkin(skin.id)}
                            >
                                <div ref={el => skinViewerRefs.current[index] = el}>
                                    <ReactSkinview3d
                                        skinUrl={`https://crafatar.com/skins/9a18d613-c78d-47d3-a58e-8bc56edcb5f0`}
                                        height="200"
                                        width="200"
                                    />
                                </div>
                            </div>
                        ))}
                    </div>
                    {selectedSkin && (
                        <button className={styles.applyButton} onClick={applySkin}>
                            Apply Selected Skin
                        </button>
                    )}
                </div>
            );
        case 'inventory':
            return <div className={styles.serverBorderContainer}>Inventory content</div>;
        case 'friend':
            return <div className={styles.serverBorderContainer}>Friend content</div>;
        case 'world':
            return <div className={styles.serverBorderContainer}>World content</div>;
        case 'money':
            return <div className={styles.serverBorderContainer}>Money content</div>;
        default:
            return null;
    }
};

export default WidgetContent;
