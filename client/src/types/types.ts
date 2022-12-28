import { Component } from "solid-js";
import { SetStoreFunction } from "solid-js/store";

export type AppStore = {
  user?: {
    id: string;
    email: string;
  };
};

export type ComponentWithProps<T> = Component<{
  props: T;
}>;
export type ComponentWithAppStore = Component<{
  appStore: [AppStore, SetStoreFunction<AppStore>];
}>;
