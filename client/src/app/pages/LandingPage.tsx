import { Component } from "solid-js";

const LandingPage: Component = () => {
  return (
    <div class="flex justify-center inset-0 absolute items-center">
      <div>
        <div class="bg-red-300 p-5">
          <h1 class="text-4xl text-center">Login to Mixify</h1>
          <div class="flex flex-col items-center">
            <div class="form-control w-full max-w-xs">
              <label class="label">
                <span class="label-text">Your Email</span>
              </label>
              <input
                type="text"
                placeholder="example@mail.com"
                class="input input-bordered w-full max-w-xs"
              />
            </div>
            <button class="btn my-2">Sign in</button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default LandingPage;
