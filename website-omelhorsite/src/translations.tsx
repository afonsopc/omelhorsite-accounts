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
    confirmationEmailSent: string,
    confirmationEmailSentSad: string,
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
    confirmationEmailSent: "Um email de confirmação foi enviado.",
    confirmationEmailSentSad: "Um email de confirmação foi enviado. 😔",
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
    websiteName: "O Melhor Website"
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