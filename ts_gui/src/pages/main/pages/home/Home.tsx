import styles from "./Home.module.scss";
import ReactSkinview3d from "react-skinview3d"
import { WalkingAnimation } from "skinview3d";

import { useLoaderData } from "react-router-dom";
import { IMainLoader, IServerSelectedLoader, serverselectedLoader } from "@/loader";
import ButtonPlay from "../../components/buttonPlay/ButtonPlay";


export default function Home() {


  const loaderData = useLoaderData() as IMainLoader;
  function findChildImageUrlById(childId: string): string | null {
    for (const server of loaderData.servers) {
      const child = server.children.find(child => child.id === childId);
      if (child) {
        return child.imageUrl;
      }
    }
    return null;
  }
  const imgsrc = findChildImageUrlById(loaderData?.selected_server?.childrenServerId);
  return (
    <div className={styles.homeContainer}>
      <h1 className={styles.text}>功能尚未完成</h1>
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
      {imgsrc ? (
        <div>
          <img className={styles.serverBorderContainer} src={imgsrc} />
        </div>
      ) : (
        <p></p>
      )}

      {loaderData?.selected_server ? (
        <div>
          <ButtonPlay
            serverId={loaderData?.selected_server?.server_id}
            childrenServerId={loaderData?.selected_server?.childrenServerId}
          />
        </div>
      ) : (
        <p>沒有最後遊玩的伺服器</p>
      )}

    </div>
  );
}