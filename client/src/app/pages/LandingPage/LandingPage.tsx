import { Component } from "solid-js";

const LandingPage: Component = () => {
  return (
    <div class="flex justify-center inset-0 absolute items-center">
      <div>
        <div class="bg-red-300 p-5">
          <h1 class="text-4xl text-center">Login to Mixify</h1>
          <div class="flex justify-center">
            <div class="mx-1 my-2">
              <label class="block">Email</label>
              <input class="rounded p-2" type="text" />
            </div>
          </div>
          <div class="flex justify-center">
            <button class="bg-black text-white rounded-md px-5 py-2 outline-none border-none">
              Sign in
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default LandingPage;
