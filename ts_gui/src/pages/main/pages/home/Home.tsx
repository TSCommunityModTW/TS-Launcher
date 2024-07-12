import styles from "./Home.module.scss";
import ReactSkinview3d from "react-skinview3d"
import { WalkingAnimation } from "skinview3d";

import { useLoaderData } from "react-router-dom";
import { IMainLoader} from "@/loader";
import ButtonPlay from "../../components/buttonPlay/ButtonPlay";
import { ILauncherAssetsServer } from "@/interfaces/ILauncherAssetsServer";
import { ILauncherAssetsServerChildren } from "@/interfaces/ILauncherAssetsServerChildren";

import HomeWidget from "../../components/homePageWidget/HomeWidget";

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


  return (
    <div className={styles.homeContainer}>


      {SelectedServer ? (
        <div className={styles.serverBorderContainer}>
          <img className={styles.serverBorderContainer} src={imgsrc} />
        </div>
      ) : (
        <p></p>
      )}
<HomeWidget playerName=""
/>
      <div>
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

    </div >
  );
}