import { CombinedError, CreateQueryState } from "solid-urql";

type QueryResponse<T> = {
  error?: CombinedError;
  data?: T;
};

export function toPromise<T>(state: () => CreateQueryState<T>) {
  return new Promise<QueryResponse<T>>(async (resolve) => {
    // TODO: Implement timeout
    while (state().fetching) {
      await sleep(10);
    }

    return resolve(state());
  });
}

function sleep(ms: number) {
  return new Promise<void>((resolve) => setTimeout(resolve, ms));
}
