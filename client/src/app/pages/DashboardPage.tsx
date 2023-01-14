import { ComponentWithAppStore } from "../../types/types";
import CurrentSelectedPlaylistInfoPanel from "../Components/CurrentSelectedPlaylistInfoPanel";
import PlaylistGraphOverview from "../Components/PlaylistGraphOverview";
import PlaylistsConfigurationPanel from "../Components/PlaylistsConfigurationPanel";

const DashboardPage: ComponentWithAppStore = ({ appStore }) => {
  const [store] = appStore;

  return (
    <div>
      {/* <h1>Hi</h1>
      <p>Your email is: {store.user?.email}</p>
      <button class="btn btn-error">Sign out</button> */}

      <div class="flex flex-row">
        <section class="basis-1/6">
          <PlaylistsConfigurationPanel appStore={appStore} />
        </section>
        <div class="basis-4/6">
          <PlaylistGraphOverview appStore={appStore} />
        </div>
        <section class="basis-1/6">
          <CurrentSelectedPlaylistInfoPanel />
        </section>
      </div>
    </div>
  );
};

export default DashboardPage;
