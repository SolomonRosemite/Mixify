import { Accessor, Component, Show } from "solid-js";
import { GetSetStore } from "../../../../types/types";
import { PlaylistInfoInputStore } from "../../PlaylistCard";
import PlaylistCardInfoInputPart from "./PlaylistCardInfoInputPart";

type PlaylistCardContentProps = {
  playlistName: string;
  playlistDescription?: string;
  playlistImageUrl: string;
  playlistOwner?: { displayName: string; uri: string };
  playlistNameInfoStore: GetSetStore<PlaylistInfoInputStore>;
  playlistDescriptionInfoStore: GetSetStore<PlaylistInfoInputStore>;
  cardOpened: Accessor<boolean>;
  playlistChangesExist: Accessor<boolean>;
  handleShowPlaylistDetailsClick: () => void;
  handleCancelClick: () => void;
};

const PlaylistCardContentPart: Component<PlaylistCardContentProps> = ({
  playlistName,
  playlistDescription,
  playlistImageUrl,
  playlistOwner,
  playlistNameInfoStore,
  playlistDescriptionInfoStore,
  playlistChangesExist,
  cardOpened,
  handleShowPlaylistDetailsClick,
  handleCancelClick,
}) => {
  return (
    <div class={"dropdown" + (cardOpened() ? " dropdown-open" : "")}>
      <label tabindex="0">
        <div
          class="card custom-button w-52 bg-base-100 shadow-xl image-full"
          onclick={handleShowPlaylistDetailsClick}
        >
          <figure>
            <img
              src={playlistImageUrl}
              alt="Playlist logo"
              class="aspect-square"
            />
          </figure>
          <div class="card-body">
            <h2 class="card-title">{playlistName}</h2>
            <p>{playlistDescription}</p>
          </div>
        </div>
      </label>
      <div
        tabindex="0"
        class="dropdown-content menu p-4 shadow bg-base-200 rounded-box w-[19vw]"
        id={playlistName}
      >
        <div class="flex justify-between">
          <div class="flex">
            <div>
              <img
                class="h-16 w-16 rounded m-0 aspect-square"
                src={playlistImageUrl}
                alt="Playlist logo"
              />
            </div>
            <Show when={playlistOwner} fallback={<></>}>
              <div class="ml-3 flex flex-col justify-between">
                <div></div>
                <div>
                  <span>
                    created by <br />
                    <span class="text-lg font-bold underline">
                      <a
                        href={playlistOwner?.uri}
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        {playlistOwner?.displayName}
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
        <div class="mt-5">
          <PlaylistCardInfoInputPart
            title="Name"
            playlistInfoStore={playlistNameInfoStore}
          />
          <div class="my-5"></div>
          <PlaylistCardInfoInputPart
            title="Description"
            playlistInfoStore={playlistDescriptionInfoStore}
          />
        </div>
        <hr class="h-1.5 my-4 bg-white rounded" />
        <div>Filter options</div>
        <hr class="h-1.5 my-4 bg-white rounded" />
        <div>Define target playlists</div>

        <div class="flex justify-between mt-16">
          <button
            class={
              "btn btn-success" +
              (!playlistChangesExist() ? " btn-disabled" : "")
            }
          >
            IDK
          </button>
          <div>
            <button class="btn btn-error" onClick={handleCancelClick}>
              Cancel
            </button>
            <button
              class={
                "btn btn-success" +
                (!playlistChangesExist() ? " btn-disabled" : "")
              }
            >
              Save changes
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PlaylistCardContentPart;
