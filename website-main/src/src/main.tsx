import React, { useEffect } from "react"
import ReactDOM from "react-dom/client"
import "./index.scss"
import "bootstrap/dist/css/bootstrap.min.css";
import Home from "./pages/Home/Home";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import HeaderAndFooter from "./layouts/HeaderAndFooter/HeaderAndFooter";
import { languages } from "./translations";

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

const apiUrl = "http://0.0.0.0:3000";

export const logout = () => {
  localStorage.removeItem("token");
  window.location.reload();
};

function getLanguage() {
  const language = localStorage.getItem("language");

  if (language === "portuguese") { return languages.portuguese }
  // if (language === "spanish") { return languages.spanish }
  // if (language === "french") { return languages.french }
  // if (language === "german") { return languages.german }
  // else { return languages.english };
  else { return languages.portuguese };

}
export const language = getLanguage();

function detectAndSetLanguage() {
  const language = localStorage.getItem("language");

  if (language) { return };

  const browserLanguage = navigator.language;

  if (browserLanguage.startsWith("pt")) {
    localStorage.setItem("language", "portuguese");
  } else {
    localStorage.removeItem("language");
  }
}



const Main = () => {
  useEffect(() => {
    detectAndSetLanguage();
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

export interface Response<T> {
  statusCode: number;
  data?: T;
}

export async function sendRequest<T>(
  method: "get" | "post" | "put" | "delete" | "patch",
  url: string,
  data?: any
): Promise<Response<T>> {
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