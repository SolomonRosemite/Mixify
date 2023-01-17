import { Component, createResource, createSignal, Show } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import SpotifyWebApi from "spotify-web-api-node";
import { ComponentWithProps, PlaylistConfiguration } from "../../types/types";
import { requestAccessTokenQuery } from "../../utils/gql/queries";
import "./PlaylistCard.style.css";

type PlaylistInfoInputStore = {
  value: string;
  isEditMode: boolean;
};

const PlaylistCard: ComponentWithProps<PlaylistConfiguration> = ({ props }) => {
  const { name, spotifyPlaylistId, associations } = props;
  const [accessToken, setAccessToken] = createSignal<string | undefined>();
  const [playlistChangesExist, setPlaylistChangesExist] = createSignal(false);
  const [cardOpened, setCardOpened] = createSignal(false);
  const [playlistDescriptionInfoStore, setPlaylistDescriptionInfoStore] =
    createStore<PlaylistInfoInputStore>({ isEditMode: false, value: "" });
  const [playlistNameInfoStore, setPlaylistNameInfoStore] =
    createStore<PlaylistInfoInputStore>({ isEditMode: false, value: name });

  const spotifyApi = new SpotifyWebApi();

  const [state] = createResource(accessToken, async () => {
    if (!spotifyPlaylistId) {
      return undefined;
    }

    const response = await spotifyApi.getPlaylist(spotifyPlaylistId);

    if (response.statusCode !== 200) {
      console.error(response.body);
      return undefined;
    }

    setPlaylistDescriptionInfoStore({
      value: getPlaylistDescription(response.body.description),
    });
    return response;
  });

  createResource(async () => {
    const response = await requestAccessTokenQuery();

    if (response.error) {
      console.error(response.error);
      return;
    }

    const { accessToken, expiresIn } = response.data!.requestAccessToken;

    spotifyApi.setAccessToken(accessToken);
    setAccessToken(accessToken);
    console.log(cardOpened());
  });

  const handleShowPlaylistDetailsClick = () => {
    setCardOpened(true);
  };

  const handleCancelClick = () => {
    if (playlistChangesExist()) {
      // TODO: If there are changes, ask the user if they want to discard them
      // return;
    }

    setCardOpened(false);
    (document.activeElement as HTMLElement).blur();
    setPlaylistNameInfoStore({ isEditMode: false });
    setPlaylistDescriptionInfoStore({ isEditMode: false });
  };

  return (
    <div>
      <Show when={state()} fallback={<div>loading or error</div>}>
        <div
          class={
            "dropdown dropdown-top" + (cardOpened() ? " dropdown-open" : "")
          }
        >
          <label tabindex="0">
            <div
              class="card custom-button w-52 bg-base-100 shadow-xl image-full"
              onclick={handleShowPlaylistDetailsClick}
            >
              <figure>
                <img src={state()!.body.images[0].url} alt="Playlist logo" />
              </figure>
              <div class="card-body">
                <h2 class="card-title">{name}</h2>
                <p>{state()!.body.description}</p>
              </div>
            </div>
          </label>
          <div
            tabindex="0"
            class="dropdown-content menu p-4 shadow bg-base-200 rounded-box w-[19vw]"
            id={name}
          >
            <div class="flex justify-between">
              <div class="flex">
                <div>
                  <img
                    class="h-16 w-16 rounded m-0"
                    src={state()!.body.images[0].url}
                    alt="Playlist logo"
                  />
                </div>
                <div class="ml-3 flex flex-col justify-between">
                  <div></div>
                  <div>
                    <span>
                      created by <br />
                      <span class="text-lg font-bold">
                        {state()!.body.owner.display_name}
                      </span>
                    </span>
                  </div>
                </div>
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
              <PlaylistInfoInput
                title="Name"
                playlistInfoStore={[
                  playlistNameInfoStore,
                  setPlaylistNameInfoStore,
                ]}
              />
              <div class="my-5"></div>
              <PlaylistInfoInput
                title="Description"
                playlistInfoStore={[
                  playlistDescriptionInfoStore,
                  setPlaylistDescriptionInfoStore,
                ]}
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
      </Show>
      <input
        type="checkbox"
        id="explain-playlist-config-modal"
        class="modal-toggle"
      />
      <div class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">What the hell is happening?</h3>
          <p class="py-4">
            Hi there! This is a short explanation how to use mixify.
            <br />
            TODO
          </p>
          <div class="modal-action">
            <label for="explain-playlist-config-modal" class="btn">
              I'm still confused but thanks
            </label>
          </div>
        </div>
      </div>
    </div>
  );
};

const getPlaylistDescription = (s: string | null | undefined) => {
  if (s) {
    return s;
  }
  return '"None"';
};

type PlaylistInfoProps = {
  title: string;
  playlistInfoStore: [
    PlaylistInfoInputStore,
    SetStoreFunction<PlaylistInfoInputStore>
  ];
};

const PlaylistInfoInput: Component<PlaylistInfoProps> = ({
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
              <h2 class="text-xl font-bold">{store.value}</h2>
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
              {/* <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg> */}
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
            placeholder={store.value}
            oninput={handleValueChange}
            class="input input-bordered input-lg"
          />
        </label>
      </div>
    </Show>
  );
};

export default PlaylistCard;
