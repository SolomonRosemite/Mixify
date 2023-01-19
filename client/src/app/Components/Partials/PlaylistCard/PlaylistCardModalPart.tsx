import type { Component } from "solid-js";

const PlaylistCardModalPart: Component = () => {
  return (
    <>
      <input
        type="checkbox"
        id="explain-playlist-config-modal"
        class="modal-toggle"
      />
      <div class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">What the hell is happening?</h3>
          <p class="py-4">
            Hi there! This is a short explanation how to use mixify.
            <br />
            TODO
          </p>
          <div class="modal-action">
            <label for="explain-playlist-config-modal" class="btn">
              I'm still confused but thanks
            </label>
          </div>
        </div>
      </div>
    </>
  );
};

export default PlaylistCardModalPart;
