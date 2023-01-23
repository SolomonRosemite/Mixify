import { createEffect, createSignal, For, Show } from "solid-js";
import { produce } from "solid-js/store";
import SpotifyWebApi from "spotify-web-api-node";
import {
  ComponentWithAppStore,
  PlaylistConfiguration,
} from "../../types/types";
import { sleep } from "../../utils/common";
import { requestAccessTokenQuery } from "../../utils/gql/queries";
import { drawLineBetweenElements as drawLineBetweenElementsUnsafe } from "../../utils/html/dangerous-html-helpers";
import PlaylistCard from "./PlaylistCard";
import "./PlaylistGraphOverview.style.css";

const PlaylistGraphOverview: ComponentWithAppStore = ({ appStore }) => {
  const [configurationChangesExist, setConfigurationChangesExist] =
    createSignal(false);
  const [addNormalPlaylistPopupOpened, setAddNormalPlaylistPopupCardOpened] =
    createSignal(false);
  const [selectedPlaylist, setSelectedPlaylist] = createSignal();
  const [playlistLayers, setPlaylistLayers] = createSignal<PlaylistLayer[]>([]);

  const [store, setStore] = appStore;

  addSpotifyCLientToStore();

  createEffect(() => {
    const parents = store.playlistConfigurations.filter((p) =>
      p.associations.every((a) => a.parent == p)
    );

    const root: PlaylistConfiguration = {
      id: "root",
      name: "root",
      isMixstack: true,
      associations: parents.map((p) => ({
        id: "test",
        child: p,
        parent: undefined,
      })),
    };

    const layers = dfs({ currentDepth: 0, layers: {}, p: root });
    const translatedLayers = Object.entries(layers).map(
      ([depth, playlists]) => ({
        depth: parseInt(depth),
        playlists,
      })
    );

    translatedLayers.splice(0, 1);
    setPlaylistLayers(translatedLayers);

    // TODO: We should probably replace this with something more better
    // This is a hacky to make sure the PlaylistCards are rendered and ready before we start drawing
    sleep(4000).then(() => {
      store.playlistConfigurations
        .flatMap((p) => p.associations)
        .forEach((a) => {
          if (a.parent) {
            drawLineBetweenElementsUnsafe(a.parent.id, a.child!.id);
          }
        });
    });

    sleep(1000 * 10).then(() => setConfigurationChangesExist(true));
  });

  async function addSpotifyCLientToStore() {
    const response = await requestAccessTokenQuery();

    if (response.error) {
      console.error(response.error);
      return;
    }

    const { accessToken, expiresIn } = response.data!.requestAccessToken;

    const spotifyApi = new SpotifyWebApi();
    spotifyApi.setAccessToken(accessToken);

    setStore(produce((s) => (s.spotifyClient = spotifyApi)));
  }

  const handleAddNormalPlaylistClick = () => {
    setAddNormalPlaylistPopupCardOpened(true);
  };

  const handleAddNormalPlaylistPopupCloseClick = () => {
    setAddNormalPlaylistPopupCardOpened(false);
    (document.activeElement as HTMLElement).blur();
  };

  return (
    <div>
      <div class="h-[95vh] overflow-x-auto overflow-y-auto flex flex-col justify-evenly">
        <For each={playlistLayers()} fallback={<div>No items</div>}>
          {(layer, layerIndex) => (
            <div class="flex justify-evenly" data-index={layerIndex()}>
              <For each={layer.playlists} fallback={<div>No items</div>}>
                {(playlist, index) => (
                  <div id={playlist.id} data-index={layerIndex() + index()}>
                    <PlaylistCard props={{ playlist, appStore }} />
                  </div>
                )}
              </For>
            </div>
          )}
        </For>
      </div>
      <div class="flex justify-between">
        <div class="mx-7 mt-3">
          <div
            class={
              "dropdown dropdown-top" +
              (addNormalPlaylistPopupOpened() ? " dropdown-open" : "")
            }
          >
            <label tabindex="0">
              <button class="btn gap-2" onClick={handleAddNormalPlaylistClick}>
                <img
                  class="w-10"
                  src="/src/assets/images/add-outline.svg"
                  alt="add icon"
                />
                Add normal playlist
              </button>
            </label>
            <div
              tabindex="0"
              class="dropdown-content menu p-4 shadow bg-base-200 rounded-box w-[20vw]"
              id={"some-id"}
            >
              <div class="flex justify-between">
                <div class="flex">
                  <div>
                    <figure></figure>
                  </div>
                  <Show when={true} fallback={<></>}>
                    <div class="ml-3 flex flex-col justify-between">
                      <div></div>
                      <div>
                        <span>
                          created by <br />
                          <span class="text-lg font-bold underline">
                            <a target="_blank" rel="noopener noreferrer">
                              playlistOwner?.displayName
                              {/* TODO: Add Link icon here */}
                            </a>
                          </span>
                        </span>
                      </div>
                    </div>
                  </Show>
                </div>
                <div>
                  <label
                    tabindex="0"
                    for="explain-playlist-config-modal"
                    class="btn btn-circle btn-ghost text-info"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      class="w-6 h-6 stroke-current"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      ></path>
                    </svg>
                  </label>
                </div>
              </div>

              <div class="flex justify-between mt-16">
                <div />
                <div>
                  <button
                    class="btn btn-error"
                    onClick={handleAddNormalPlaylistPopupCloseClick}
                  >
                    Cancel
                  </button>
                  <button
                    class={
                      "ml-2 btn btn-success" +
                      (!!selectedPlaylist() ? " btn-disabled" : "")
                    }
                  >
                    Add playlist
                  </button>
                </div>
              </div>
            </div>
          </div>
          <button class="btn ml-7">
            <img
              class="w-10"
              src="/src/assets/images/add-outline.svg"
              alt="add icon"
            />
            Add mixstack
          </button>
        </div>
        <div class="mx-7 mt-3">
          <button
            class={
              "btn btn-success" +
              (!configurationChangesExist() ? " btn-disabled" : "")
            }
          >
            Save changes
          </button>
        </div>
      </div>
    </div>
  );
};

type PlaylistLayer = {
  depth: number;
  playlists: PlaylistConfiguration[];
};
type DFSPlaylistLayers = {
  [depth: number]: PlaylistConfiguration[];
};

type DFSParams = {
  p: PlaylistConfiguration;
  currentDepth: number;
  layers: DFSPlaylistLayers;
};

const dfs: (params: DFSParams) => DFSPlaylistLayers = ({
  currentDepth,
  layers,
  p,
}) => {
  const layer = layers[currentDepth] || [];
  layer.push(p);
  layers[currentDepth] = layer;

  p.associations.forEach((a) => {
    if (a.child && a.child != p) {
      dfs({ currentDepth: currentDepth + 1, layers, p: a.child });
    }
  });

  return layers;
};

export default PlaylistGraphOverview;
