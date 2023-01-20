import { createEffect, createSignal, For } from "solid-js";
import {
  ComponentWithAppStore,
  PlaylistConfiguration,
} from "../../types/types";
import { sleep } from "../../utils/common";
import { drawLineBetweenElements as drawLineBetweenElementsUnsafe } from "../../utils/html/dangerous-html-helpers";
import PlaylistCard from "./PlaylistCard";
import "./PlaylistGraphOverview.style.css";

const PlaylistGraphOverview: ComponentWithAppStore = ({ appStore }) => {
  const [playlistLayers, setPlaylistLayers] = createSignal<PlaylistLayer[]>([]);
  const [configurationChangesExist, setConfigurationChangesExist] =
    createSignal(false);
  const [store] = appStore;

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

  return (
    <div>
      <div class="h-[95vh] overflow-x-auto overflow-y-auto flex flex-col justify-evenly">
        <For each={playlistLayers()} fallback={<div>No items</div>}>
          {(layer, layerIndex) => (
            <div class="flex justify-evenly" data-index={layerIndex()}>
              <For each={layer.playlists} fallback={<div>No items</div>}>
                {(playlist, index) => (
                  <div id={playlist.id} data-index={layerIndex() + index()}>
                    <PlaylistCard props={playlist} />
                  </div>
                )}
              </For>
            </div>
          )}
        </For>
      </div>
      <div class="flex justify-between">
        <div class="mx-7 mt-3">
          <button class="btn gap-2">
            <img
              class="w-10"
              src="/src/assets/images/add-outline.svg"
              alt="add icon"
            />
            Add normal playlist
          </button>
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
