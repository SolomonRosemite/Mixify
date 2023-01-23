import { createResource, createSignal, Match, Show, Switch } from "solid-js";
import { createStore } from "solid-js/store";
import {
  AppStore,
  ComponentWithProps,
  GetSetStore,
  PlaylistConfiguration,
} from "../../types/types";
import PlaylistCardContentPart from "./Partials/PlaylistCard/PlaylistCardContentPart";
import PlaylistCardModalPart from "./Partials/PlaylistCard/PlaylistCardModalPart";
import "./PlaylistCard.style.css";

export type PlaylistInfoInputStore = {
  fallbackValue?: string;
  value?: string;
  isEditMode: boolean;
};

export type PlaylistCardProps = {
  playlist: PlaylistConfiguration;
  appStore: GetSetStore<AppStore>;
};

const PlaylistCard: ComponentWithProps<PlaylistCardProps> = ({ props }) => {
  const { name, spotifyPlaylistId, isMixstack } = props.playlist;
  const [store] = props.appStore;

  const [playlistChangesExist, setPlaylistChangesExist] = createSignal(false);
  const [cardOpened, setCardOpened] = createSignal(false);
  const [playlistDescriptionInfoStore, setPlaylistDescriptionInfoStore] =
    createStore<PlaylistInfoInputStore>({
      isEditMode: false,
      fallbackValue: `"None"`,
    });
  const [playlistNameInfoStore, setPlaylistNameInfoStore] =
    createStore<PlaylistInfoInputStore>({ isEditMode: false, value: name });

  const [playlist] = createResource(
    () => store.spotifyClient,
    async (client) => {
      if (!spotifyPlaylistId) {
        return undefined;
      }

      const response = await client.getPlaylist(spotifyPlaylistId);

      if (response.statusCode !== 200) {
        console.error(response.body);
        return undefined;
      } else if (response.body.description) {
        setPlaylistDescriptionInfoStore({
          value: response.body.description,
        });
      }
      return response;
    }
  );

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
      <Switch>
        <Match when={spotifyPlaylistId}>
          <Show when={playlist()} fallback={<div>loading or error</div>}>
            <PlaylistCardContentPart
              playlistIsMixstack={isMixstack}
              playlistDescription={playlist()!.body.description ?? undefined}
              playlistImageUrl={playlist()!.body.images[0].url}
              playlistName={name}
              playlistOwner={{
                displayName: playlist()!.body.owner.display_name!,
                uri: playlist()!.body.owner.uri,
              }}
              playlistNameInfoStore={[
                playlistNameInfoStore,
                setPlaylistNameInfoStore,
              ]}
              playlistDescriptionInfoStore={[
                playlistDescriptionInfoStore,
                setPlaylistDescriptionInfoStore,
              ]}
              playlistChangesExist={playlistChangesExist}
              cardOpened={cardOpened}
              handleCancelClick={handleCancelClick}
              handleShowPlaylistDetailsClick={handleShowPlaylistDetailsClick}
            />
          </Show>
        </Match>
        <Match when={!spotifyPlaylistId}>
          <PlaylistCardContentPart
            playlistImageUrl={"src/assets/images/empty_playlist.png"}
            playlistName={name}
            playlistIsMixstack={isMixstack}
            playlistNameInfoStore={[
              playlistNameInfoStore,
              setPlaylistNameInfoStore,
            ]}
            playlistDescriptionInfoStore={[
              playlistDescriptionInfoStore,
              setPlaylistDescriptionInfoStore,
            ]}
            playlistChangesExist={playlistChangesExist}
            cardOpened={cardOpened}
            handleCancelClick={handleCancelClick}
            handleShowPlaylistDetailsClick={handleShowPlaylistDetailsClick}
          />
        </Match>
      </Switch>
      <PlaylistCardModalPart />
    </div>
  );
};

export default PlaylistCard;
