import { ComponentWithAppStore } from "../../types/types";

const PlaylistBuilderPage: ComponentWithAppStore = ({ appStore }) => {
  const [store] = appStore;

  return (
    <div>
      <h1>Hi</h1>
      <p>Your email is: {store.user?.email}</p>
      <button class="btn btn-error">Sign out</button>
    </div>
  );
};

export default PlaylistBuilderPage;
