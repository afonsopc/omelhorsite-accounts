import React from "react"
import ReactDOM from "react-dom/client"
import "./index.css"
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Home from "./pages/Home/Home";
import Account from "./pages/account/Account";
import { getLanguage } from "./translations";

export interface ApiResponse<T> {
  statusCode: number;
  data?: T;
}

export async function sendApiRequest<T>(
  method: "get" | "post" | "put" | "delete" | "patch",
  url: string,
  data?: any
): Promise<ApiResponse<T>> {
  try {
    const headers = {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${localStorage.getItem("token")}`,
    };

    console.log({
      method: method,
      url: url,
      data: data,
      headers: headers
    });

    const response = await fetch(url, {
      method: method,
      headers: headers,
      body: data ? JSON.stringify(data) : undefined,
    });

    if (!response.ok) {
      const statusCode = response.status;
      return {
        statusCode,
      };
    }

    const responseData: T = await response.json();

    return {
      statusCode: response.status,
      data: responseData,
    };
  } catch (error) {
    return {
      statusCode: 500,
    };
  }
}

export const language = getLanguage();

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />
  },
  {
    path: "/account",
    element: <Account />,
  },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
