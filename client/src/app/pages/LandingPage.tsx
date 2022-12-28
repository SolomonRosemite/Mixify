import { useRequestUserConfirmationCodeQuery } from "../../graphql/generated/graphql";
import { Component, createResource } from "solid-js";
import { graphqlUrl } from "../../App";

function requestUserConfirmationCode() {
  const [, state] = useRequestUserConfirmationCodeQuery({
    context: {
      // When refetching a query the provided solid urql client can not be found for some reason.
      // This is why we have to provide the url manually.
      url: graphqlUrl,
      requestPolicy: "network-only",
    },
    variables: { email: "test@mail.com" },
  });
  return state;
}

const LandingPage: Component = () => {
  const [query, { refetch }] = createResource(requestUserConfirmationCode);
  // Does not work...
  // const [{ latest }, { refetch }] = createResource(requestUserConfirmationCode);

  function handleClick() {
    refetch();
  }

  return (
    <div class="flex justify-center inset-0 absolute items-center">
      <div>
        <div class="bg-red-300 p-5">
          <h1 class="text-4xl text-center">
            Login to Mixify
            {/* <div>
              {latest?.fetching && <div>Loading...</div>}
              {!latest?.fetching &&
                latest?.data?.requestConfirmationCode.confirmationSecret}
            </div> */}
            <div>
              {query()!()?.fetching && <div>Loading...</div>}
              {!query()!()?.fetching &&
                query()!()?.data?.requestConfirmationCode.confirmationSecret}
            </div>
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
