import { BrianSDK } from "@brian-ai/sdk";

const TIMEOUT = 10000; // Define a timeout (e.g., 10 seconds)

const options = {
  apiKey: process.env.BRIAN_API_KEY,
};

if (!options.apiKey) {
  throw new Error("Missing API key! Please set BRIAN_API_KEY in your environment variables.");
}

const brian = new BrianSDK(options);

describe("POST /agent/smart-contracts", () => {
  test(
    "asks Brian to generate an ERC-20 Smart Contract",
    async () => {
      try {
        const result = await brian.generateCode({
          prompt: "Give me the code for an ERC-20 Smart Contract",
        });

        console.log("Generated Smart Contract:", result); // Log the result

        expect(result).not.toBeNull();
        expect(result.code).toContain("pragma solidity"); // Example assertion to check Solidity code
      } catch (error) {
        console.error("Error generating code:", error);
        throw error; // Ensure test fails if an error occurs
      }
    },
    TIMEOUT
  );
});
