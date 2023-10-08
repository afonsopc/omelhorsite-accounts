import React, { useEffect } from "react"
import ReactDOM from "react-dom/client"
import "./index.scss"
import "bootstrap/dist/css/bootstrap.min.css";
import Home from "./pages/Home/Home";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import HeaderAndFooter from "./layouts/HeaderAndFooter/HeaderAndFooter";
import { getLanguage, languages } from "./translations";

const router = createBrowserRouter([
  {
    path: "/",
    element: <HeaderAndFooter validAccount={localStorage.getItem("token") ? true : false} />,
    children: [
      {
        path: "",
        element: <Home />
      },
    ]
  }
]);

export const sleep = (seconds: number) => {
  let secondsToMiliseconds = seconds * 1000;
  return new Promise(resolve => setTimeout(resolve, secondsToMiliseconds));
}

export const language = getLanguage();

const Main = () => {
  useEffect(() => {
    document.title = language.dictionary.websiteName;
  }, []);
  console.log(language);

  return (
    <div className="app">
      <RouterProvider router={router} />
    </div>
  )
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>,
)

import axios, { AxiosError, AxiosResponse } from 'axios';

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


    const response: AxiosResponse<T> = await axios({
      method: method,
      url: url,
      data: data,
      headers: headers
    });

    return {
      statusCode: response.status,
      data: response.data,
    };
  } catch (error) {
    if (axios.isAxiosError(error)) {
      const axiosError: AxiosError = error;
      const statusCode = axiosError.response ? axiosError.response.status : 500;
      return {
        statusCode,
      };
    } else {
      return {
        statusCode: 500,
      };
    }
  }
}