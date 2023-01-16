import { OperationContext } from "solid-urql";
import {
  useConfigurationsQuery,
  useRequestAccessTokenQuery,
  useRequestUserConfirmationCodeQuery,
} from "../../graphql/generated/graphql";
import { toPromise } from "./query-converter";

const context: Partial<OperationContext> = {
  // When refetching a query the provided solid urql client can not be found for some reason.
  // This is why we have to provide the url manually.
  url: "http://localhost:5000/query",
  // https://formidable.com/open-source/urql/docs/basics/document-caching/
  requestPolicy: "cache-and-network",
};

export const requestAccessTokenQuery = () => {
  const [, state] = useRequestAccessTokenQuery({
    context,
  });
  return toPromise(state);
};

export const usePlaylistConfigurationQuery = (id: string) => {
  const [, state] = useConfigurationsQuery({
    context,
    variables: { id },
  });
  return toPromise(state);
};

export const requestUserConfirmationCode = (email: string) => {
  const [, state] = useRequestUserConfirmationCodeQuery({
    context,
    variables: { email },
  });
  return toPromise(state);
};
