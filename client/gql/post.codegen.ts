// This file was generated using chat gpt using the following prompt:
// Write a function in typescript that reads a file and deletes every line with the following pattern "Document = {*" and replaces all occurrences of "UseQueryArgs" to "CreateQueryArgs" and "useQuery" to "createQuery" and "from 'urql'" to "from 'solid-urql'". Then finally save the file.

// This post codegen file is required because the generated file from the codegen config file is not compatible with solid-urql. This file is a workaround to make the generated file compatible with solid-urql.
import * as fs from "fs";

modifyAndSaveFile("src/graphql/generated/graphql.ts").catch(console.log);

async function modifyAndSaveFile(filePath: string): Promise<void> {
  // Read the file
  const fileContent = await fs.promises.readFile(filePath, "utf-8");

  // Split the file into an array of lines
  const lines = fileContent.split("\n");

  // Iterate through the lines and modify them as needed
  const modifiedLines = lines.map((line) => {
    if (line.includes("Document = {")) {
      // Delete lines with this pattern
      return "";
    } else {
      // Replace "UseQueryArgs" with "CreateQueryArgs"
      line = line.replace(/UseQueryArgs/g, "CreateQueryArgs");
      // Replace "useQuery" with "createQuery"
      line = line.replace(/useQuery/g, "createQuery");
      // Replace "from 'urql'" with "from 'solid-urql'"
      line = line.replace(/from 'urql'/g, "from 'solid-urql'");
      return line;
    }
  });

  // Join the modified lines back into a single string
  const modifiedContent = modifiedLines.join("\n");

  // Save the modified file
  await fs.promises.writeFile(filePath, modifiedContent);
}
