/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
import gql from 'graphql-tag';
import * as Urql from 'solid-urql';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Mutation = {
  __typename?: 'Mutation';
  createPlaylistSnapshotConfiguration: PlaylistSnapshotConfiguration;
  createSyncPlaylistsEvent: SyncPlaylistsEvent;
};


export type MutationCreatePlaylistSnapshotConfigurationArgs = {
  input: NewPlaylistSnapshotConfiguration;
};


export type MutationCreateSyncPlaylistsEventArgs = {
  input: NewSyncPlaylistsEvent;
};

export type NewPlaylistAssociationSnapshot = {
  childPlaylistId: Scalars['ID'];
  parentPlaylistId: Scalars['ID'];
};

export type NewPlaylistSnapshot = {
  associations: Array<NewPlaylistAssociationSnapshot>;
  name: Scalars['String'];
  playlistId: Scalars['ID'];
  playlistOrder: Array<InputMaybe<Scalars['Int']>>;
  spotifyPlaylistId?: InputMaybe<Scalars['String']>;
};

export type NewPlaylistSnapshotConfiguration = {
  playlists: Array<NewPlaylistSnapshot>;
};

export type NewSyncPlaylistsEvent = {
  configurationSnapshotId: Scalars['ID'];
};

export type PlaylistAssociationSnapshot = {
  __typename?: 'PlaylistAssociationSnapshot';
  childPlaylistId: Scalars['ID'];
  id: Scalars['ID'];
  parentPlaylistId: Scalars['ID'];
};

export type PlaylistSnapshot = {
  __typename?: 'PlaylistSnapshot';
  associations: Array<PlaylistAssociationSnapshot>;
  id: Scalars['ID'];
  name: Scalars['String'];
  playlistOrder: Array<Maybe<Scalars['Int']>>;
  spotifyPlaylistId?: Maybe<Scalars['String']>;
};

export type PlaylistSnapshotConfiguration = {
  __typename?: 'PlaylistSnapshotConfiguration';
  id: Scalars['ID'];
  playlists: Array<PlaylistSnapshot>;
};

export type Query = {
  __typename?: 'Query';
  confirmConfirmationCode: User;
  requestConfirmationCode: RequestConfirmationCodeResponse;
  syncEvents: SyncPlaylistsEvent;
};


export type QueryConfirmConfirmationCodeArgs = {
  confirmationCode: Scalars['String'];
  confirmationSecret?: InputMaybe<Scalars['String']>;
};


export type QueryRequestConfirmationCodeArgs = {
  email: Scalars['String'];
};


export type QuerySyncEventsArgs = {
  id: Scalars['ID'];
};

export type RequestConfirmationCodeResponse = {
  __typename?: 'RequestConfirmationCodeResponse';
  confirmationSecret: Scalars['String'];
};

export type SyncPlaylistsEvent = {
  __typename?: 'SyncPlaylistsEvent';
  configurationSnapshot: Array<Maybe<PlaylistSnapshotConfiguration>>;
  id: Scalars['ID'];
  userId: Scalars['ID'];
};

export type User = {
  __typename?: 'User';
  email: Scalars['String'];
  id: Scalars['ID'];
  spotifyUserId: Scalars['String'];
  syncEvents: Array<SyncPlaylistsEvent>;
  username: Scalars['String'];
};

export type ConfirmConfirmationCodeQueryVariables = Exact<{
  code: Scalars['String'];
  secret: Scalars['String'];
}>;


export type ConfirmConfirmationCodeQuery = { __typename?: 'Query', confirmConfirmationCode: { __typename?: 'User', email: string, id: string } };

export type RequestUserConfirmationCodeQueryVariables = Exact<{
  email: Scalars['String'];
}>;


export type RequestUserConfirmationCodeQuery = { __typename?: 'Query', requestConfirmationCode: { __typename?: 'RequestConfirmationCodeResponse', confirmationSecret: string } };





export const ConfirmConfirmationCodeDocument = gql`
    query ConfirmConfirmationCode($code: String!, $secret: String!) {
  confirmConfirmationCode(confirmationCode: $code, confirmationSecret: $secret) {
    email
    id
  }
}
    `;

export function useConfirmConfirmationCodeQuery(options: Omit<Urql.CreateQueryArgs<ConfirmConfirmationCodeQueryVariables>, 'query'>) {
  return Urql.createQuery<ConfirmConfirmationCodeQuery, ConfirmConfirmationCodeQueryVariables>({ query: ConfirmConfirmationCodeDocument, ...options });
};
export const RequestUserConfirmationCodeDocument = gql`
    query RequestUserConfirmationCode($email: String!) {
  requestConfirmationCode(email: $email) {
    confirmationSecret
  }
}
    `;

export function useRequestUserConfirmationCodeQuery(options: Omit<Urql.CreateQueryArgs<RequestUserConfirmationCodeQueryVariables>, 'query'>) {
  return Urql.createQuery<RequestUserConfirmationCodeQuery, RequestUserConfirmationCodeQueryVariables>({ query: RequestUserConfirmationCodeDocument, ...options });
};