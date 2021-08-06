import { AnyAction } from "redux";
import { ThunkDispatch } from "redux-thunk";
import { AuthRoute, LoginData, RegisterData } from "types";
import { AUTHENTICATE, DEAUTHENTICATE } from "./actionTypes";
import { API } from "config";
import Router from "next/router";
import { action } from "typesafe-actions";
import { axiosInstance } from "lib/utils";

export const authenticate = (token: string) => action(AUTHENTICATE, { token })
export const deauthenticate = () => action(DEAUTHENTICATE)

export const removeToken = () => (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    Router.push("/login");
    dispatch(deauthenticate);
};

export const fetchToken = (
    user: RegisterData | LoginData,
    route: AuthRoute
) => async (dispatch: ThunkDispatch<{}, {}, AnyAction>) => {
    try {
        const res = await axiosInstance.post(`${API}${route}`, user);
        const { token } = res.data;
        dispatch(authenticate(token));
        Router.push("/feed/all");
    } catch (error) {
        throw new Error(error);
    }
};
