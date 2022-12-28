import { Component, createResource, Show } from "solid-js";
import { graphqlUrl } from "../../App";
import { useRequestUserConfirmationCodeQuery } from "../../graphql/generated/graphql";
import { toPromise } from "../utils/gql/query-converter";

function requestUserConfirmationCode() {
  const [, state] = useRequestUserConfirmationCodeQuery({
    context: {
      // When refetching a query the provided solid urql client can not be found for some reason.
      // This is why we have to provide the url manually.
      url: graphqlUrl,
    },
    variables: { email: "test@mail.com" },
  });
  return toPromise(state);
}

const LandingPage: Component = () => {
  const [query, { refetch }] = createResource(requestUserConfirmationCode);

  const handleClick = () => refetch();

  return (
    <div class="flex justify-center inset-0 absolute items-center">
      <div>
        <div class="bg-red-300 p-5">
          <h1 class="text-4xl text-center">
            Login to Mixify
            <Show when={query()} fallback={<div>Loading...</div>}>
              <div>
                {query()?.error && query()!.error?.message}
                {query()?.data &&
                  query()?.data?.requestConfirmationCode.confirmationSecret}
              </div>
            </Show>
          </h1>
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
            <button class="btn my-2" onClick={handleClick}>
              Sign in
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default LandingPage;
