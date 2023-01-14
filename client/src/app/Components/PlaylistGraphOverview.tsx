import { ComponentWithAppStore } from "../../types/types";

const PlaylistGraphOverview: ComponentWithAppStore = ({ appStore }) => {
  const [store] = appStore;
  return <div>{store.playlistConfigurations.length}</div>;
  //   return <div>{store.playlistConfigurations.map((p) => p.name)}</div>;
};

export default PlaylistGraphOverview;
