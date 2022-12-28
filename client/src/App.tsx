import { Route, Routes } from "@solidjs/router";
import type { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { createClient, Provider } from "solid-urql";
import DashboardPage from "./app/pages/DashboardPage";
import EmailConfirmationPage from "./app/pages/EmailConfirmationPage";
import LandingPage from "./app/pages/LandingPage";
import NotFoundPage from "./app/pages/NotFoundPage";
import { AppStore } from "./types/types";

export const graphqlUrl = "http://localhost:5000/query";
const client = createClient({ url: graphqlUrl });

const App: Component = () => {
  const store = createStore<AppStore>({});

  return (
    <Provider value={client}>
      <Routes>
        <Route path="/" component={LandingPage} />
        <Route
          path="/confirmation/:id"
          element={<EmailConfirmationPage appStore={store} />}
        />
        <Route path="/dashboard" element={<DashboardPage appStore={store} />} />
        <Route path="/*" element={<NotFoundPage />} />
      </Routes>
    </Provider>
  );
};

export default App;
