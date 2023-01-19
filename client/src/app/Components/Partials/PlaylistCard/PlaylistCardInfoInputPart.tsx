import { Component, Show } from "solid-js";
import { GetSetStore } from "../../../../types/types";
import { PlaylistInfoInputStore } from "../../PlaylistCard";

type PlaylistInfoProps = {
  title: string;
  playlistInfoStore: GetSetStore<PlaylistInfoInputStore>;
};

const PlaylistCardInfoInputPart: Component<PlaylistInfoProps> = ({
  title,
  playlistInfoStore,
}) => {
  const [store, setStore] = playlistInfoStore;

  const handleEditClick = () => setStore({ isEditMode: true });

  const handleValueChange = (e: Event) =>
    setStore({ value: (e.target as HTMLInputElement).value });

  return (
    <Show
      when={store.isEditMode}
      fallback={
        <div>
          <div class="flex justify-between">
            <div>
              <label>
                <span>{title}</span>
              </label>
              <h2 class="text-xl font-bold">
                {store.value ?? store.fallbackValue}
              </h2>
            </div>
            <button
              class="btn btn-square btn-outline"
              onClick={handleEditClick}
            >
              <img
                class="h-6 w-6"
                src="/src/assets/images/edit-outline.svg"
                alt="edit"
              />
            </button>
          </div>
        </div>
      }
    >
      <div class="form-control">
        <label class="input-group input-group-lg">
          <span>{title}</span>
          <input
            type="text"
            placeholder={store.value ?? store.fallbackValue}
            oninput={handleValueChange}
            class="input input-bordered input-lg"
          />
        </label>
      </div>
    </Show>
  );
};

export default PlaylistCardInfoInputPart;
