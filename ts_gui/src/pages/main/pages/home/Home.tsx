import styles from "./Home.module.scss";
import ReactSkinview3d from "react-skinview3d"
import { WalkingAnimation } from "skinview3d";

import { useLoaderData } from "react-router-dom";
import { IMainLoader } from "@/loader";
import { ILauncherAssetsServer } from "@/interfaces/ILauncherAssetsServer";
import { ILauncherAssetsServerChildren } from "@/interfaces/ILauncherAssetsServerChildren";

import { useState } from "react";
import HomeWidget from "../../components/homePageWidget/HomeWidget";
import ButtonPlay from "../../components/buttonPlay/ButtonPlay";

export default function Home() {


  interface ILauncherAssetsServerResult {
    server: ILauncherAssetsServer;
    child: ILauncherAssetsServerChildren;
  }
  const loaderData = useLoaderData() as IMainLoader;
  function findChildServerById(childId: String): ILauncherAssetsServerResult | null {
    for (const server of loaderData.servers) {
      const child = server.children.find(child => child.id === childId);
      if (child) {
        return { server, child };
      }
    }
    return null;
  }


  const SelectedServer = findChildServerById(loaderData?.selected_server?.childrenServerId);
  const imgsrc = SelectedServer?.child?.imageUrl;

  const [selectedWidgetButton, setSelectedWidgetButton] = useState<string | null>(null);
  const handleWidgetButtonClick = (buttonKey: string) => {
    setSelectedWidgetButton(buttonKey);
  };

  return (
    <div className={styles.homeContainer}>
      <div className={styles.leftContainer}>
        <div className={styles.WidgetWindow}>
          {SelectedServer && selectedWidgetButton === "home" ? (
            <div className={styles.serverBorderContainer}>
              <img className={styles.serverBorderContainer} src={imgsrc} />
            </div>
          ) : (
            <div></div>
          )}
        </div>
        <div className={styles.Widget}>
          <HomeWidget
            playerName={loaderData.player.name ? loaderData.player.name : ""}
            onButtonClick={handleWidgetButtonClick}
          />
        </div>
      </div>
      <div className={styles.SkinView}>
        {loaderData?.player?.uuid ? (
          <ReactSkinview3d
            skinUrl={`https://crafatar.com/skins/${loaderData.player.uuid}`}
            height="500"
            width="500"
            onReady={({ viewer }) => {
              const walkingAnimation = new WalkingAnimation();
              walkingAnimation.headBobbing = false;
              viewer.animation = walkingAnimation;
              viewer.animation.speed = 0.5;
            }}
          />
        ) : (
          <p>無法加載皮膚</p>
        )}
        <div className={styles.playButton}>
          {SelectedServer ? (
            <div className={styles.buttonPlay}>
              <ButtonPlay
                serverId={loaderData?.selected_server?.server_id}
                childrenServerId={loaderData?.selected_server?.childrenServerId}
              />
            </div>
          ) : (
            <p></p>
          )}
        </div>

      </div>

    </div >
  );
}