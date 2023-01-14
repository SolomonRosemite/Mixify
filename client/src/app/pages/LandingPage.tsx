import { useNavigate } from "@solidjs/router";
import { Component, createSignal } from "solid-js";
import { requestUserConfirmationCode } from "../../utils/gql/queries";

const LandingPage: Component = () => {
  const navigate = useNavigate();
  const [email, setEmail] = createSignal("");

  const handleSignInClick = async () => {
    const response = await requestUserConfirmationCode(email());

    if (response.error) {
      console.error(response.error);
      return;
    }

    const secret = response.data!.requestConfirmationCode.confirmationSecret;
    const state = { email: email(), secret };
    navigate(`/confirmation`, {
      state,
    });
  };

  const handleEmailChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setEmail(target.value);
  };

  const handleKeyPressEnter = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSignInClick();
    }
  };

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
                class="input input-bordered w-full max-w-xs"
                placeholder="example@mail.com"
                type="text"
                onsubmit={() => console.log("submit")}
                onKeyPress={handleKeyPressEnter}
                onInput={handleEmailChange}
              />
            </div>
            <button class="btn my-2" onClick={handleSignInClick}>
              Sign in
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default LandingPage;
