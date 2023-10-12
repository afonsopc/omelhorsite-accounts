interface Dictionary {
    unknownError: string,
    email: string
    enterEmail: string
    enterNewEmail: string
    enterNewPassword: string
    show: string
    hide: string
    changeEmail: string
    change: string
    changePassword: string
    newEmail: string
    password: string,
    enterPassword: string,
    newPassword: string,
    dont_have_account_yet: string,
    already_have_account: string,
    signIn: string,
    signUp: string,
    close: string,
    copyright: string,
    logotype: string,
    home: string,
    about: string,
    pricing: string,
    account: string,
    myDrive: string,
    settings: string,
    logout: string,
    changeLanguage: string,
    language: string,
    deleteAccount: string,
    deleteAccountConfirmation: string,
    homeDescription: string,
    homeTitle: string,
    free: string,
    freePlanPrice: string,
    premium: string,
    premiumPlanPrice: string,
    monthsShort: string,
    freePlan1: string,
    premiumPlan1: string,
    unavailable: string,
    websiteName: string,
    enterUsername: string
    username: string,
    invalidConfirmationCode: string,
    confirmationCodeModalTitle: string,
    toConfirmInsertConfirmationCodeBellow: string,
    confirmationEmailSent: string,
    confirmationEmailSentToOriginal: string,
    confirmationEmailSentToNew: string,
    cancel: string,
    cancelPendingConfirmations: string,
    pendingConfirmationsCanceledSuccessfully: string,
    confirm: string,
    enterNewUsername: string,
    newUsername: string,
    changeUsername: string,
    currentUsername: string,
    currentEmail: string,
    dark: string,
    light: string,
    automatic: string,
    changeTheme: string,
}

const portugueseDictionary: Dictionary = {
    unknownError: "Erro desconhecido.",
    email: "Email",
    enterEmail: "Introduza o seu Email",
    enterNewEmail: "Introduza o seu novo Email",
    password: "Palavra-Passe",
    enterPassword: "Introduza a sua Palavra-Passe",
    dont_have_account_yet: "Ainda não tem uma conta?",
    already_have_account: "Já tem uma conta?",
    signIn: "Iniciar Sessão",
    signUp: "Criar Conta",
    close: "Fechar",
    copyright: "Afonso Coutinho, 2023",
    logotype: "logótipo",
    home: "Inicio",
    about: "Sobre",
    pricing: "Preços",
    account: "Conta",
    myDrive: "O meu Drive",
    settings: "Definições",
    logout: "Sair",
    changeEmail: "Alterar o Email",
    change: "Alterar",
    newEmail: "Novo Email",
    changePassword: "Alterar Palavra-Passe",
    newPassword: "Nova Palavra-Passe",
    enterNewPassword: "Introduza a sua nova Palavra-Passe",
    show: "Mostrar",
    hide: "Esconder",
    changeLanguage: "Alterar Linguagem",
    language: "Língua",
    deleteAccount: "Apagar Conta",
    deleteAccountConfirmation: "Tem certeza que quer apagar a sua conta? Esta ação é irreversível.",
    homeDescription: "Oferecemos até 50GB de espaço de armazenamento para cada utilizador. Armazene e aceda aos seus ficheiros com facilidade, segurança e comodidade. Aceda aos seus ficheiros em todos os seus dispositivos, partilhe documentos importantes e evite a perda de informação!",
    homeTitle: "A Sua Nuvem Pessoal",
    free: "Grátis",
    premium: "Premium",
    monthsShort: "mês",
    freePlan1: "20 GB de armazenamento",
    premiumPlan1: "50 GB de armazenamento",
    unavailable: "Indisponivel",
    freePlanPrice: "0",
    premiumPlanPrice: "5",
    websiteName: "O Melhor Site",
    enterUsername: "Introduza o seu Nome de Utilizador",
    username: "Nome de Utilizador",
    invalidConfirmationCode: "Código de confirmação inválido.",
    confirmationCodeModalTitle: "Código de Confirmação",
    toConfirmInsertConfirmationCodeBellow: "Para confirmar introduza o código de confirmação na caixa de texto abaixo.",
    confirmationEmailSent: "Foi enviado um código de confirmação para o email previamente fornecido.",
    cancel: "Cancelar",
    confirm: "Confirmar",
    cancelPendingConfirmations: "Cancelar confirmações pendentes",
    pendingConfirmationsCanceledSuccessfully: "Confirmações pendentes canceladas com sucesso!",
    enterNewUsername: "Introduza o novo Nome de Utilizador",
    newUsername: "Novo Nome de Utilizador",
    changeUsername: "Alterar o Nome de Utilizador",
    confirmationEmailSentToOriginal: "Foi enviado um código de confirmação para o email original.",
    confirmationEmailSentToNew: "Foi enviado um código de confirmação para o novo email.",
    currentUsername: "Nome de Utilizador atual",
    currentEmail: "Email atual",
    dark: "Escuro",
    light: "Claro",
    automatic: "Automático",
    changeTheme: "Alterar o Tema"
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
    const language = localStorage.getItem("language") ? localStorage.getItem("language") : detectAndSetLanguage();

    if (language === "portuguese") { return languages.portuguese }
    // if (language === "spanish") { return languages.spanish }
    // if (language === "french") { return languages.french }
    // if (language === "german") { return languages.german }
    // else { return languages.english };
    return languages.portuguese

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