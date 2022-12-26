import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import type { Component } from "solid-js";
import EmailConfirmationPage from "./app/pages/EmailConfirmationPage";
import PlaylistBuilderPage from "./app/pages/PlaylistBuilderPage";
import LandingPage from "./app/pages/LandingPage";

const queryClient = new QueryClient();

const App: Component = () => (
  <QueryClientProvider client={queryClient}>
    <LandingPage />
    {/* <PlaylistBuilderPage /> */}
    {/* <EmailConfirmationPage /> */}
  </QueryClientProvider>
);

export default App;
