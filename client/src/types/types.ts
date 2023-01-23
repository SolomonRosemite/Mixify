import { Component } from "solid-js";
import { SetStoreFunction } from "solid-js/store";
import SpotifyWebApi from "spotify-web-api-node";

export type GetSetStore<T> = [T, SetStoreFunction<T>];

export type AppStore = {
  user?: {
    id: string;
    email: string;
  };
  spotifyClient?: SpotifyWebApi;
  playlistConfigurations: PlaylistConfiguration[];
};

export type EmailConfirmationNavState = {
  email: string;
  secret: string;
};

export type ComponentWithProps<T> = Component<{
  props: T;
}>;
export type ComponentWithAppStore = Component<{
  appStore: GetSetStore<AppStore>;
}>;

export type PlaylistConfiguration = {
  id: string;
  name: string;
  isMixstack: boolean;
  spotifyPlaylistId?: string | null;
  associations: PlaylistAssociation[];
};

export type PlaylistAssociation = {
  id: string;
  parent?: PlaylistConfiguration;
  child?: PlaylistConfiguration;
};
