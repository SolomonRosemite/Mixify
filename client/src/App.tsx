import { Route, Routes } from "@solidjs/router";
import type { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { createClient, Provider } from "solid-urql";
import DashboardPage from "./app/pages/DashboardPage";
import EmailConfirmationPage from "./app/pages/EmailConfirmationPage";
import LandingPage from "./app/pages/LandingPage";
import NotFoundPage from "./app/pages/NotFoundPage";
import {
  AppStore,
  PlaylistAssociation,
  PlaylistConfiguration,
} from "./types/types";

export const graphqlUrl = "http://localhost:5000/query";
const client = createClient({ url: graphqlUrl });

const playlists: PlaylistConfiguration[] = [
  {
    id: "test1",
    name: "Second Best Rap playlist eu",
    associations: [],
  },
  {
    id: "test1",
    name: "Generic Rap playlist",
    associations: [],
  },
  {
    id: "test1",
    name: "Rap Bangers",
    associations: [],
  },
  {
    id: "test1",
    name: "Rap Vibes",
    associations: [],
  },
  {
    id: "test1",
    name: "IDK",
    associations: [],
  },
];

const associations: PlaylistAssociation[] = [
  {
    id: "test1",
    child: playlists[3],
    parent: playlists[1],
  },
  {
    id: "test1",
    child: playlists[2],
    parent: playlists[1],
  },
  {
    id: "test1",
    child: playlists[1],
    parent: playlists[0],
  },
  {
    id: "test1",
    child: playlists[4],
    parent: playlists[0],
  },
];

playlists.forEach((p) => {
  p.associations = associations.filter((a) => a.parent === p || a.child === p);
});

console.log(playlists);

const App: Component = () => {
  const store = createStore<AppStore>({
    playlistConfigurations: playlists,
  });

  return (
    <Provider value={client}>
      <Routes>
        <Route path="/" component={LandingPage} />
        <Route
          path="/confirmation"
          element={<EmailConfirmationPage appStore={store} />}
        />
        <Route path="/dashboard" element={<DashboardPage appStore={store} />} />
        <Route path="/*" element={<NotFoundPage />} />
      </Routes>
    </Provider>
  );
};

export default App;
