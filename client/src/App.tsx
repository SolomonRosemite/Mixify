import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import type { Component } from "solid-js";
import EmailConfirmationPage from "./app/pages/EmailConfirmationPage";
import PlaylistBuilderPage from "./app/pages/PlaylistBuilderPage";
import LandingPage from "./app/pages/LandingPage";
import { createClient, Provider } from "solid-urql";

export const graphqlUrl = "http://localhost:5000/query";
const client = createClient({ url: graphqlUrl });

const App: Component = () => (
  <Provider value={client}>
    <LandingPage />
    {/* <PlaylistBuilderPage /> */}
    {/* <EmailConfirmationPage /> */}
  </Provider>
);

export default App;
