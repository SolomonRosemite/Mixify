import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  overwrite: true,
  schema: "http:localhost:5000/query",
  documents: "src/graphql/schemas/**/*.graphql",
  generates: {
    "src/graphql/generated/": {
      preset: "client",
      plugins: ["typescript-urql"],
    },
  },
};

export default config;
