import axios from "axios";
import { AnyAction } from "redux";
import { ThunkDispatch } from "redux-thunk";
import { AuthRoute, LoginData, RegisterData, Tokens } from "types";
import {
    AUTHENTICATE, DEAUTHENTICATE,
} from "./actionTypes";
import { API } from "config"
import Router from "next/router"
import { action } from "typesafe-actions";
import { removeCookie, setCookie } from "lib/cookie";

export const authenticate = (tokens: Tokens) => (dispatch: ThunkDispatch<{}, {}, AnyAction>) => {
    dispatch(action(AUTHENTICATE, { tokens }))
}

export const deauthenticate = (tokens: Tokens) => (dispatch: ThunkDispatch<{}, {}, AnyAction>) => {
    removeCookie('token');
    Router.push("/");
    dispatch(action(DEAUTHENTICATE, { tokens }))
}

export const fetchTokens = (user: RegisterData | LoginData, route: AuthRoute) => async (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    try {
        const res = await axios.post(`${API}${route}`, user);
        const tokens: Tokens = res.data.tokens;
        setCookie('accessToken', tokens.accessToken);
        setCookie('refreshToken', tokens.refreshToken);
        dispatch(authenticate(tokens));
        Router.push("/");
    } catch (error) {
        throw new Error(error);
    }
};

