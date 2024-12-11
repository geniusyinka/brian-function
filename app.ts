const BRIAN_API_KEY = '';

export const main = async (req: any) => {
  console.log(req);
  const { method, body } = req;

  if (method === "GET") {
    console.log("IN GET BLOCK");
    const resp = {
      icon: "https://t3.ftcdn.net/jpg/05/59/27/48/360_F_559274893_O9iSRQwTKIkAooNTglilMgx2yMcXK9Or.jpg",
      title: "Brian Knows API",
      description: "Invoke the Brian Knows API for transactions Main",
      label: "Activate Transaction",
    };

    return { status: 200, body: JSON.stringify(resp), headers: [] };
  } else if (method === "POST") {
    const { prompt, address } = body;

    if (!prompt || !address) {
      return {
        status: 400,
        body: JSON.stringify({ error: "Prompt and address are required" }),
        headers: [],
      };
    }

    try {
      const response = await fetch("https://api.brianknows.org/api/v0/agent/transaction", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-brian-api-key": BRIAN_API_KEY,
        },
        body: JSON.stringify({ prompt, address }),
      });

      {}

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const responseData = await response.json();

      const resp = {
        output: responseData,
        message: "Transaction processed successfully",
      };

      return { status: 200, body: JSON.stringify(resp), headers: [] };
    } catch (error: any) {
      console.error("Error processing POST request:", error.message);

      return {
        status: 500,
        body: JSON.stringify({ error: error.message }),
        headers: [],
      };
    }
  }

  return { status: 405, body: JSON.stringify({ error: "Method not allowed" }), headers: [] };
};
