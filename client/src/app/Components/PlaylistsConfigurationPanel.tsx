import type { Component } from "solid-js";
import { ComponentWithAppStore } from "../../types/types";

const PlaylistsConfigurationPanel: ComponentWithAppStore = ({ appStore }) => {
  const [store, setStore] = appStore;

  const handleAddMixstackClick = () => {
    console.log("Add mixstack");
  };

  const handleAddBasePlaylistClick = () => {
    console.log("Add base playlist");

    setStore({
      playlistConfigurations: [
        ...store.playlistConfigurations,
        {
          id: "test1",
          name: "New Playlist",
          associations: [],
        },
      ],
    });
  };

  return (
    <div class="h-screen bg-slate-900 flex flex-col justify-between">
      <section class="text-center mt-4">
        <div>
          <h1 class="text-center text-2xl font-bold">Ye</h1>
        </div>
      </section>
      <section>
        <div class="m-5">
          <hr class="p-2" />
          <AddPlaylistButton
            title="Add Mixstack"
            onClick={handleAddMixstackClick}
          />
        </div>
        <div class="m-5">
          <AddPlaylistButton
            title="Add base Playlist"
            onClick={handleAddBasePlaylistClick}
          />
        </div>
      </section>
    </div>
  );
};

type AddPlaylistButtonProps = {
  title: string;
  onClick?: () => void;
};

const AddPlaylistButton: Component<AddPlaylistButtonProps> = ({
  title,
  onClick,
}) => {
  return (
    <button class="btn btn-outline w-full" onClick={onClick}>
      {title}
    </button>
  );
};

export default PlaylistsConfigurationPanel;
