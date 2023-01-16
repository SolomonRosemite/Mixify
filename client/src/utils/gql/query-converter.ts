import { CombinedError, CreateQueryState } from "solid-urql";
import { sleep } from "../common";

type QueryResponse<T> = {
  error?: CombinedError;
  data?: T;
};

export function toPromise<T>(state: () => CreateQueryState<T>) {
  return new Promise<QueryResponse<T>>(async (resolve) => {
    // TODO: Implement timeout
    while (state().fetching) {
      // This is a hacky way to wait for the query to finish.
      await sleep(10);
    }

    return resolve(state());
  });
}
