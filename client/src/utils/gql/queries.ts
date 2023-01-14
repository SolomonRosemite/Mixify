import { graphqlUrl } from "../../App";
import { useRequestUserConfirmationCodeQuery } from "../../graphql/generated/graphql";
import { toPromise } from "./query-converter";

export const requestUserConfirmationCode = (email: string) => {
  const [, state] = useRequestUserConfirmationCodeQuery({
    context: {
      // When refetching a query the provided solid urql client can not be found for some reason.
      // This is why we have to provide the url manually.
      url: graphqlUrl,
      // This query is used to request and resent the confirmation code as the user wishes.
      // By preventing the query from being cached another request will be made to the server.
      requestPolicy: "network-only",
    },
    variables: { email },
  });
  return toPromise(state);
};
