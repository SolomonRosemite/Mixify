import { Component } from "solid-js";
import { SetStoreFunction } from "solid-js/store";

export type AppStore = {
  user?: {
    id: string;
    email: string;
  };
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
  appStore: [AppStore, SetStoreFunction<AppStore>];
}>;

export type PlaylistConfiguration = {
  id: string;
  name: string;
  spotifyPlaylistId?: string | null;
  associations: PlaylistAssociation[];
};

export type PlaylistAssociation = {
  id: string;
  parent?: PlaylistConfiguration;
  child?: PlaylistConfiguration;
};
