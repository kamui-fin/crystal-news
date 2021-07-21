import { IncomingMessage } from "http";
import { NextApiRequestCookies } from "next/dist/next-server/server/api-utils";
import { ActionType } from "typesafe-actions";
import { actions } from "./store/actions/authActions";

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

export interface RootState {
    auth: AuthState;
}

export interface AuthState {
    token: string | null;
}

export interface Source {
    sourceId: number;
    title: string;
    website: string;
}

export enum RequestSource {
    CLIENT,
    SERVER,
}

export type Actions = ActionType<typeof actions>;
export type AuthRoute = "/login" | "/signup";
export type Request = IncomingMessage & { cookies: NextApiRequestCookies };
