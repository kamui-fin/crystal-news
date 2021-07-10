import axios from "axios";
import { NextRouter } from "next/dist/client/router";
import { AnyAction } from "redux";
import { ThunkDispatch } from "redux-thunk";
import { LoginData, RegisterData, Tokens } from "types";
import { action } from "typesafe-actions";
import {
    SIGN_UP_SUCCESS,
    SIGN_UP_FAILURE,
    SIGN_UP_REQUEST,
    LOGIN_REQUEST,
    LOGIN_SUCCESS,
    LOGIN_FAILURE,
} from "./actionTypes";
import { API } from "config"
import { saveUserData } from "lib/auth";

export const signUpRequest = () => {
    return action(SIGN_UP_REQUEST);
};

export const signUpSuccess = (userId: number) => {
    return action(SIGN_UP_SUCCESS, userId);
};

export const signUpFailure = () => {
    return action(SIGN_UP_FAILURE);
};

export const loginRequest = () => {
    return action(LOGIN_REQUEST);
};

export const loginSuccess = (userId: number) => {
    return action(LOGIN_SUCCESS, userId);
};

export const loginFailure = () => {
    return action(LOGIN_FAILURE);
};

export const signUp = (user: RegisterData, router: NextRouter) => async (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    dispatch(signUpRequest());
    try {
        const res = await axios.post(`${API}/signup`, user);
        const tokenRes: Tokens = res.data.tokens;
        const userId: number = res.data.userId;
        saveUserData({ ...tokenRes, userId });
        dispatch(signUpSuccess(userId));
        router.push("/");
    } catch (error) {
        dispatch(signUpFailure());
    }
};

export const login = (user: LoginData, router: NextRouter) => async (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    dispatch(loginRequest());
    try {
        const res = await axios.post(`${API}/login`, user);
        const tokenRes: Tokens = res.data.tokens;
        const userId: number = res.data.userId;
        saveUserData({ ...tokenRes, userId });
        dispatch(loginSuccess(userId));
        router.push("/");
    } catch (error) {
        dispatch(loginFailure());
    }
};
