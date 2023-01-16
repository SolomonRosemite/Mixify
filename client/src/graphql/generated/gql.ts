/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel-plugin for production.
 */
const documents = {
    "query ConfirmConfirmationCode($code: String!, $secret: String!) {\n  confirmConfirmationCode(confirmationCode: $code, confirmationSecret: $secret) {\n    email\n    id\n  }\n}": types.ConfirmConfirmationCodeDocument,
    "query Configurations($id: ID!) {\n  configurations(id: $id) {\n    id\n    playlists {\n      id\n      name\n      spotifyPlaylistId\n      playlistOrder\n      associations {\n        id\n        childPlaylistId\n        parentPlaylistId\n      }\n    }\n  }\n}": types.ConfigurationsDocument,
    "query RequestAccessToken {\n  requestAccessToken {\n    accessToken\n    expiresIn\n  }\n}": types.RequestAccessTokenDocument,
    "query RequestUserConfirmationCode($email: String!) {\n  requestConfirmationCode(email: $email) {\n    confirmationSecret\n  }\n}": types.RequestUserConfirmationCodeDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query ConfirmConfirmationCode($code: String!, $secret: String!) {\n  confirmConfirmationCode(confirmationCode: $code, confirmationSecret: $secret) {\n    email\n    id\n  }\n}"): (typeof documents)["query ConfirmConfirmationCode($code: String!, $secret: String!) {\n  confirmConfirmationCode(confirmationCode: $code, confirmationSecret: $secret) {\n    email\n    id\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query Configurations($id: ID!) {\n  configurations(id: $id) {\n    id\n    playlists {\n      id\n      name\n      spotifyPlaylistId\n      playlistOrder\n      associations {\n        id\n        childPlaylistId\n        parentPlaylistId\n      }\n    }\n  }\n}"): (typeof documents)["query Configurations($id: ID!) {\n  configurations(id: $id) {\n    id\n    playlists {\n      id\n      name\n      spotifyPlaylistId\n      playlistOrder\n      associations {\n        id\n        childPlaylistId\n        parentPlaylistId\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query RequestAccessToken {\n  requestAccessToken {\n    accessToken\n    expiresIn\n  }\n}"): (typeof documents)["query RequestAccessToken {\n  requestAccessToken {\n    accessToken\n    expiresIn\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query RequestUserConfirmationCode($email: String!) {\n  requestConfirmationCode(email: $email) {\n    confirmationSecret\n  }\n}"): (typeof documents)["query RequestUserConfirmationCode($email: String!) {\n  requestConfirmationCode(email: $email) {\n    confirmationSecret\n  }\n}"];

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
**/
export function graphql(source: string): unknown;

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;