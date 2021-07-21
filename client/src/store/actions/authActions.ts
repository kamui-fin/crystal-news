import { AnyAction } from "redux";
import { ThunkDispatch } from "redux-thunk";
import { AuthRoute, LoginData, RegisterData } from "types";
import { AUTHENTICATE, DEAUTHENTICATE } from "./actionTypes";
import { API } from "config";
import Router from "next/router";
import { action } from "typesafe-actions";
import { axiosInstance } from "lib/utils";

export const actions = {
    authenticate: (token: string) => action(AUTHENTICATE, { token }),
    deauthenticate: () => action(DEAUTHENTICATE),
};

export const deauthenticate = () => (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    Router.push("/login");
    dispatch(actions.deauthenticate);
};

export const fetchToken = (
    user: RegisterData | LoginData,
    route: AuthRoute
) => async (dispatch: ThunkDispatch<{}, {}, AnyAction>) => {
    try {
        const res = await axiosInstance.post(`${API}${route}`, user);
        const { token } = res.data;
        dispatch(actions.authenticate(token));
        Router.push("/");
    } catch (error) {
        throw new Error(error);
    }
};
