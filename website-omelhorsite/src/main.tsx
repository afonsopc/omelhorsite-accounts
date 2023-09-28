import React, { useEffect } from "react"
import ReactDOM from "react-dom/client"
import "./index.scss"
import "bootstrap/dist/css/bootstrap.min.css";
import Home from "./routes/Home/Home";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import HeaderAndFooter from "./layouts/HeaderAndFooter/HeaderAndFooter";
import axios from "axios";
import { languages } from "./translations";
import Account from "./routes/Account/Account";
 
const router = createBrowserRouter([
  {
    path: "/",
    element: <HeaderAndFooter/>,
    children: [
      {
        path: "",
        element: <Home/>
      },
      {
        path: "account",
        element: <Account/>
      }
    ]
  }
]);

const apiUrl = "http://0.0.0.0:3000";

export const createAccount = async (email: string, password: string) => {
  try {
    const response = await sendRequest("POST", `${apiUrl}/account/signup`, {
      email: email,
      password: password,
    });

    return response;
  } catch (error) {
    throw error;
  }
};


export const changeEmail = async (newEmail: string) => {
  try {
    const response = await sendRequest("PATCH", `${apiUrl}/account/email`, {
      email: newEmail,
    });

    return response;
  } catch (error) {
    throw error;
  }
};

export const changePassword = async (newPassword: string) => {
  try {
    const response = await sendRequest("PATCH", `${apiUrl}/account/password`, {
      password: newPassword,
    });

    return response;
  } catch (error) {
    throw error;
  }
};

export const deleteAccount = async () => {
  try {
    const response = await sendRequest("DELETE", `${apiUrl}/account/`);

    return response;
  } catch (error) {
    throw error;
  }
};

export const authenticateAccount = async (email: string, password: string) => {
  try {
    const response = await sendRequest("POST", `${apiUrl}/account/signin`, {
      email: email,
      password: password,
    });

    return response;
  } catch (error) {
    throw error;
  }
};

export const verifyAccountToken = async () => {
  try {
    const response = await sendRequest("GET", `${apiUrl}/account`);

    return response;
  } catch (error) {
    throw error;
  }
};

const sendRequest = async (method: string, url: string, data?: any) => {
  try {
    const headers = {
      "Authorization": `Bearer ${localStorage.getItem("token")}`
    }
    const response = await axios.request({
      method: method,
      url: url,
      data: data,
      headers: headers
    });

    return response;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      if (error.response) {
        return error.response;
      }
      throw error
    } else {
      throw error;
    }
  }
};


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
    <>
      <RouterProvider router={router}/>
    </>
  )
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Main/>
  </React.StrictMode>,
)