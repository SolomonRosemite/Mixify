import type { Component } from "solid-js";

const EmailConfirmationPage: Component = () => {
  return (
    <div class="my-80">
      <div>
        <img
          src="https://getlogovector.com/wp-content/uploads/2021/01/tailwind-css-logo-vector.png"
          alt="logo"
          class="w-32 mx-auto"
        />
      </div>
      <div>
        <div class="flex justify-between flex-col items-center mx-auto">
          <div>
            <h1 class="text-center text-2xl font-bold">
              Confirmation Mail Sent
            </h1>
            <p>
              Please check your email for a confirmation code in your inbox.
            </p>
            <hr class="p-2" />
          </div>
          <div class="flex flex-col items-center">
            <div class="form-control w-full max-w-xs">
              <label class="label">
                <span class="label-text">Confirmation Code</span>
              </label>
              <input
                type="text"
                placeholder="Code"
                class="input input-bordered w-full max-w-xs"
              />
            </div>
            <button class="block btn my-2">Sign in</button>
            <p>
              Still never received any mail? <a class="link">Resent code.</a>
            </p>
          </div>
          <div>
            <p>
              If you don't see it, check your spam folder. And you still don't
              see it, please contact <a href="">us here</a>.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default EmailConfirmationPage;
