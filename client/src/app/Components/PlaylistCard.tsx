import { createResource, createSignal, Show } from "solid-js";
import SpotifyWebApi from "spotify-web-api-node";
import { ComponentWithProps, PlaylistConfiguration } from "../../types/types";
import { requestAccessTokenQuery } from "../../utils/gql/queries";
import "./PlaylistCard.style.css";

const PlaylistCard: ComponentWithProps<PlaylistConfiguration> = ({ props }) => {
  const { name, spotifyPlaylistId, associations } = props;
  const [accessToken, setAccessToken] = createSignal<string | undefined>();

  const spotifyApi = new SpotifyWebApi();

  const [state] = createResource(accessToken, async () => {
    if (!spotifyPlaylistId) {
      return undefined;
    }

    return await spotifyApi.getPlaylist(spotifyPlaylistId);
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

  return (
    <div>
      <Show when={state()} fallback={<div>loading or error</div>}>
        <div class="card custom-button w-52 bg-base-100 shadow-xl image-full">
          <figure>
            <img src={state()!.body.images[0].url} alt="Playlist logo" />
          </figure>
          <div class="card-body">
            <h2 class="card-title">{name}</h2>
            <p>{state()!.body.description}</p>
          </div>
        </div>
      </Show>
    </div>
  );
};

export default PlaylistCard;
