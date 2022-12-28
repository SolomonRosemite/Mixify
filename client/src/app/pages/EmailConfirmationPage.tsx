import { useNavigate, useParams } from "@solidjs/router";
import { createSignal } from "solid-js";
import { produce } from "solid-js/store";
import { graphqlUrl } from "../../App";
import { useConfirmConfirmationCodeQuery } from "../../graphql/generated/graphql";
import { ComponentWithAppStore } from "../../types/types";
import { toPromise } from "../../utils/gql/query-converter";

const EmailConfirmationPage: ComponentWithAppStore = ({ appStore }) => {
  const navigate = useNavigate();
  const params = useParams();
  const [, setStore] = appStore;

  const [confirmationSecret, setConfirmationSecret] = createSignal(params.id);
  const [confirmationCode, setConfirmationCode] = createSignal("");

  const handleConfirmationCodeChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setConfirmationCode(target.value);
  };

  const handleSignInClick = async () => {
    console.log(confirmationSecret(), confirmationCode());
    const response = await confirmConfirmationCode(
      confirmationCode(),
      confirmationSecret()
    );

    if (response.error) {
      console.error(response.error);
      return;
    }

    const { email, id } = response.data!.confirmConfirmationCode;
    setStore(produce((store) => (store.user = { email, id })));
    navigate("/dashboard");
  };

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
                class="input input-bordered w-full max-w-xs"
                type="text"
                placeholder="Code"
                oninput={handleConfirmationCodeChange}
              />
            </div>
            <button class="block btn my-2" onClick={handleSignInClick}>
              Sign in
            </button>
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

const confirmConfirmationCode = (code: string, secret: string) => {
  const [, state] = useConfirmConfirmationCodeQuery({
    context: {
      // When refetching a query the provided solid urql client can not be found for some reason.
      // This is why we have to provide the url manually.
      url: graphqlUrl,
    },
    variables: { code, secret },
  });
  return toPromise(state);
};

export default EmailConfirmationPage;
