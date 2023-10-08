import { ApiResponse, sendApiRequest } from '../main';

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

export interface AccountInfo {
    username: string,
    email: string,
}

interface AccountInfoResponse {
    username: string,
    email: string,
}


export async function cancelConfirmations(): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/confirmations/cancel`

    const response = await sendApiRequest<EmptyResponse>("post", url);

    return response
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

export async function signUp(signUp: SignUpCredentials): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/signup`

    let signUpRequest: SignUpRequest = {
        username: signUp.username,
        email: signUp.email,
        password: signUp.password
    };

    const response = await sendApiRequest<TokenResponse>("post", url, signUpRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response;
}

export async function signUpConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/signup/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<TokenResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response;
}

export interface SignInCredentials {
    email: string,
    password: string
}

interface SignInRequest {
    email: string,
    password: string
}

export async function signIn(signIn: SignInCredentials): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/authenticate`

    let signInRequest: SignInRequest = {
        email: signIn.email,
        password: signIn.password
    };

    const response = await sendApiRequest<TokenResponse>("post", url, signInRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response
}

export function logout() { localStorage.removeItem("token") };


export async function deleteAccount(): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/delete`

    const response = await sendApiRequest<EmptyResponse>("post", url);

    return response
}

export async function deleteAccountConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/delete/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<EmptyResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200) {
        localStorage.removeItem("token");
    }

    return response;
}

export async function getAccountInfo(): Promise<ApiResponse<AccountInfoResponse>> {
    const url = `${API_URL}/account`

    const response = await sendApiRequest<AccountInfoResponse>("get", url);

    return response;
}



interface ChangeUsernameRequest {
    username: string,
}

export async function changeUsername(username: string): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/username`

    let changeUsernameRequest: ChangeUsernameRequest = {
        username: username,
    };

    const response = await sendApiRequest<EmptyResponse>("post", url, changeUsernameRequest);

    return response;
}

export async function changeUsernameConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/change/username/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<TokenResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response;
}

interface ChangePasswordRequest {
    password: string,
}

export async function changePassword(password: string): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/password`

    let changePasswordRequest: ChangePasswordRequest = {
        password: password,
    };

    const response = await sendApiRequest<EmptyResponse>("post", url, changePasswordRequest);

    return response;
}

export async function changePasswordConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/change/password/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<TokenResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response;
}

interface ChangeEmailRequest {
    email: string,
}

export async function changeEmail(email: string): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/email`

    let changeEmailRequest: ChangeEmailRequest = {
        email: email,
    };

    const response = await sendApiRequest<EmptyResponse>("post", url, changeEmailRequest);

    return response;
}

export async function changeEmailStepOneConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<EmptyResponse>> {
    const url = `${API_URL}/change/email/one/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<EmptyResponse>("post", url, confirmationCodeRequest);

    return response;
}

export async function changeEmailStepTwoConfirm(confirmationCode: ConfirmationCode): Promise<ApiResponse<TokenResponse>> {
    const url = `${API_URL}/change/email/two/confirm`

    let confirmationCodeRequest: ConfirmationCodeRequest = {
        confirmation_code: confirmationCode.confirmationCode
    };

    const response = await sendApiRequest<TokenResponse>("post", url, confirmationCodeRequest);

    if (response.statusCode == 200 && response.data && response.data.token) {
        localStorage.setItem("token", response.data.token);
    }

    return response;
}