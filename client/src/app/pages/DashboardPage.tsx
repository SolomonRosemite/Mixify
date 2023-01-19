import { createEffect, createResource, Show } from "solid-js";
import {
  ComponentWithAppStore,
  PlaylistAssociation,
  PlaylistConfiguration
} from "../../types/types";
import { usePlaylistConfigurationQuery } from "../../utils/gql/queries";
import CurrentSelectedPlaylistInfoPanel from "../Components/CurrentSelectedPlaylistInfoPanel";
import PlaylistGraphOverview from "../Components/PlaylistGraphOverview";
import PlaylistsConfigurationPanel from "../Components/PlaylistsConfigurationPanel";

const DashboardPage: ComponentWithAppStore = ({ appStore }) => {
  const [configuration, { refetch }] = createResource(() =>
    // TODO: Remove hard coded id
    usePlaylistConfigurationQuery("3")
  );
  const [store, setStore] = appStore;

  createEffect(() => {
    const response = configuration();

    if (!response) {
      return;
    } else if (response.error) {
      console.error(response.error);
      return;
    }

    const queriedPlaylists = response.data!.configurations.playlists;
    const associations = queriedPlaylists.flatMap((p) => p.associations);
    const playlists: PlaylistConfiguration[] = queriedPlaylists.map((p) => ({
      id: p.id,
      name: p.name,
      spotifyPlaylistId: p.spotifyPlaylistId ?? undefined,
      associations: [],
    }));

    playlists.forEach((p) => {
      const newAssociations: PlaylistAssociation[] = associations
        .filter(
          (a) => a.childPlaylistId === p.id || a.parentPlaylistId === p.id
        )
        .map((as) => ({
          id: as.id,
          child: playlists.find((p) => p.id == as.childPlaylistId),
          parent: playlists.find((p) => p.id == as.parentPlaylistId),
        }));

      p.associations = newAssociations;
    });

    setStore({ playlistConfigurations: playlists });
  });

  return (
    <div>
      {/* <h1>Hi</h1>
      <p>Your email is: {store.user?.email}</p>
      <button class="btn btn-error">Sign out</button> */}
      <Show when={configuration} fallback={<div>loading or error</div>}>
        <div class="flex flex-row">
          <section class="basis-1/6">
            <PlaylistsConfigurationPanel appStore={appStore} />
          </section>
          <div class="basis-4/6 w-4/6">
            <PlaylistGraphOverview appStore={appStore} />
          </div>
          <section class="basis-1/6">
            <CurrentSelectedPlaylistInfoPanel />
          </section>
        </div>
      </Show>
    </div>
  );
};

export default DashboardPage;
