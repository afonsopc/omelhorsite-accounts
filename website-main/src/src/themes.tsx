import { getLanguage } from "./translations";

export const language = getLanguage();

export const setThemeBasedOnUserPreference = () => {
    const htmlElement = document.querySelector<HTMLElement>("html");
    if (!htmlElement) { return };

    let storedUserPreference = localStorage.getItem("theme");

    if (storedUserPreference) {
        htmlElement.setAttribute("data-bs-theme", storedUserPreference);
    }
    else {
        htmlElement.setAttribute(
            "data-bs-theme",
            window.matchMedia("(prefers-color-scheme: dark)")
                .matches ? "dark" : "light"
        );
    }
}

export const startListenerToThemeChange = () => {
    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", setThemeBasedOnUserPreference);
}

export const getCurrentTheme = (): Theme => {
    const themeCode = localStorage.getItem("theme");
    switch (themeCode) {
        case "dark":
            return dark;
        case "light":
            return light;
        default:
            return automatic;
    }
};

export const setTheme = (theme: Theme): void => {
    if (theme.code === automatic.code) {
        localStorage.removeItem("theme");
    }
    else {
        localStorage.setItem("theme", theme.code);
    }
};

export interface Theme {
    code: string;
    name: string;
}

const dark: Theme = {
    code: "dark",
    name: language.dictionary.dark,
}

const light: Theme = {
    code: "light",
    name: language.dictionary.light,
}

const automatic: Theme = {
    code: "automatic",
    name: language.dictionary.automatic,
}

export interface Themes {
    dark: Theme;
    light: Theme;
    automatic: Theme;
}

export const themes: Themes = {
    dark,
    light,
    automatic,
}