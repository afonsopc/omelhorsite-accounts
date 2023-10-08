import { sendRequest } from '../main';

const API_URL = "http://localhost:3002"

export interface ConfirmationCode {
    confirmationCode: string
}

interface ConfirmationCodeRequest {
    confirmation_code: string
}

interface EmptyResponse { }

interface TokenResponse {
    token?: string,
}


export async function cancelConfirmations(): Promise<number> {
    const url = `${API_URL}/confirmations/cancel`

    const response = await sendRequest<EmptyResponse>("post", url);

    return response.statusCode
}

export interface SignUpCredentials {
    username: string,
    email: string,
    password: string
}

interface SignUpRequest {
    username: string,
    email: string,
    password: string
}

export async function signUp(signUp: SignUpCredentials): Promise<number> {
    const url = `${API_URL}/signup`

    let signUpRequest: SignUpRequest = {
        username: signUp.username,
        email: signUp.email,
        password: signUp.password
    };

    const response = await sendRequest<TokenResponse>("post", url, signUpRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
        return response.statusCode
    }

    return response.statusCode
}

export async function signUpConfirm(confirmationCode: ConfirmationCode): Promise<number> {
    const url = `${API_URL}/signup/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendRequest<TokenResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
        return response.statusCode
    }

    return response.statusCode;
}

export interface SignInCredentials {
    email: string,
    password: string
}

interface SignInRequest {
    email: string,
    password: string
}

export async function signIn(signIn: SignInCredentials): Promise<number> {
    const url = `${API_URL}/authenticate`

    let signInRequest: SignInRequest = {
        email: signIn.email,
        password: signIn.password
    };

    const response = await sendRequest<TokenResponse>("post", url, signInRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
        return response.statusCode
    }

    return response.statusCode
}

export function logout() { localStorage.removeItem("token") };


export async function deleteAccount(): Promise<number> {
    const url = `${API_URL}/change/delete`

    const response = await sendRequest<EmptyResponse>("post", url);

    return response.statusCode
}

export async function deleteAccountConfirm(confirmationCode: ConfirmationCode): Promise<number> {
    const url = `${API_URL}/change/delete/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendRequest<EmptyResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200) {
        localStorage.removeItem("token");
        return response.statusCode
    }

    return response.statusCode;
}