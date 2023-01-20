import { createResource, createSignal, Match, Show, Switch } from "solid-js";
import { createStore } from "solid-js/store";
import SpotifyWebApi from "spotify-web-api-node";
import { ComponentWithProps, PlaylistConfiguration } from "../../types/types";
import { requestAccessTokenQuery } from "../../utils/gql/queries";
import PlaylistCardContentPart from "./Partials/PlaylistCard/PlaylistCardContentPart";
import PlaylistCardModalPart from "./Partials/PlaylistCard/PlaylistCardModalPart";
import "./PlaylistCard.style.css";

export type PlaylistInfoInputStore = {
  fallbackValue?: string;
  value?: string;
  isEditMode: boolean;
};

const PlaylistCard: ComponentWithProps<PlaylistConfiguration> = ({ props }) => {
  const { name, spotifyPlaylistId, isMixstack } = props;
  const [accessToken, setAccessToken] = createSignal<string | undefined>();
  const [playlistChangesExist, setPlaylistChangesExist] = createSignal(false);
  const [cardOpened, setCardOpened] = createSignal(false);
  const [playlistDescriptionInfoStore, setPlaylistDescriptionInfoStore] =
    createStore<PlaylistInfoInputStore>({
      isEditMode: false,
      fallbackValue: `"None"`,
    });
  const [playlistNameInfoStore, setPlaylistNameInfoStore] =
    createStore<PlaylistInfoInputStore>({ isEditMode: false, value: name });

  const spotifyApi = new SpotifyWebApi();

  const [playlist] = createResource(accessToken, async () => {
    if (!spotifyPlaylistId) {
      return undefined;
    }

    const response = await spotifyApi.getPlaylist(spotifyPlaylistId);

    if (response.statusCode !== 200) {
      console.error(response.body);
      return undefined;
    } else if (response.body.description) {
      setPlaylistDescriptionInfoStore({
        value: response.body.description,
      });
    }
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
