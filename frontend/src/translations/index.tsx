interface Dictionary {
    welcomeToTheWebsite: string
    welcomeBack: string
}

const portugueseDictionary: Dictionary = {
    welcomeToTheWebsite: "Bem vindo ao Melhor Site!",
    welcomeBack: "Bem vindo de volta {name}!"
}

export interface Language {
    code: string;
    name: string;
    flagPath: string;
    dictionary: Dictionary;
}

const portuguese: Language = {
    code: 'portuguese',
    name: 'Português',
    flagPath: '/pt.svg',
    dictionary: portugueseDictionary
}

export interface Languages {
    portuguese: Language;
}

export const languages: Languages = {
    portuguese,
}

export function getLanguage() {
    // const language = localStorage.getItem("language") ? localStorage.getItem("language") : detectAndSetLanguage();

    // if (language === "portuguese") { return languages.portuguese }
    // if (language === "spanish") { return languages.spanish }
    // if (language === "french") { return languages.french }
    // if (language === "german") { return languages.german }
    // else { return languages.english };
    return languages.portuguese;
}

export function detectAndSetLanguage(): string {
    const userLanguage = navigator.language || navigator.languages[0];
    const languageCode = userLanguage.split('-')[0];

    if (languageCode == "pt") {
        localStorage.setItem("language", "portuguese");
        return "portuguese";
    } else if (languageCode == "es") {
        localStorage.setItem("language", "spanish");
        return "spanish";
    } else if (languageCode == "fr") {
        localStorage.setItem("language", "french");
        return "french";
    } else if (languageCode == "de") {
        localStorage.setItem("language", "german");
        return "german";
    } else {
        localStorage.setItem("language", "english");
        return "english";
    }
}