import { ActionType } from "typesafe-actions";
import * as actions from "./store/actions/authActions";

export interface RegisterData {
    username: string;
    password: string;
    confirmPassword: string;
    email: string;
}

export interface TokenResponse {
    accessToken: string;
    refreshToken: string;
}

export interface RootState {
    auth: AuthState;
}

export interface AuthState {
    userId: number;
}

export type Actions = ActionType<typeof actions>;
