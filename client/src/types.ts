import { IncomingMessage } from "http";
import { NextApiRequestCookies } from "next/dist/next-server/server/api-utils";
import { ActionType } from "typesafe-actions";
import * as actions from "./store/actions/authActions";

export interface RegisterData {
    username: string;
    password: string;
    confirmPassword: string;
    email: string;
}

export interface LoginData {
    usernameOrEmail: string;
    password: string;
}

export interface Tokens {
    accessToken: string;
    refreshToken: string;
}

export interface SavedUser extends Tokens {
    userId: number;
}

export interface RootState {
    auth: AuthState;
}

export interface AuthState {
    tokens: Tokens | null;
}

export type Actions = ActionType<typeof actions>;
export type AuthRoute = '/login' | '/signup'
export type Request = IncomingMessage & { cookies: NextApiRequestCookies };
