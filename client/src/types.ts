import { IncomingMessage } from "http";
import { NextApiRequestCookies } from "next/dist/next-server/server/api-utils";
import actions from "store/actions";
import { ActionType } from "typesafe-actions";

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
    isLoggedIn: boolean;
}

export interface Source {
    sourceId: number;
    title: string;
    website: string;
}

export interface Article {
    article_id: number;
    source_id: number;
    item_link: string;
    title: string;
    description: string;
    author: string;
    pub_date: Date;
    content: string;
    guid: string;
}

export type Actions = ActionType<typeof actions>;
export type AuthRoute = "/login" | "/signup";
