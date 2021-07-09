import axios from "axios";
import { NextRouter } from "next/dist/client/router";
import { AnyAction } from "redux";
import { ThunkDispatch } from "redux-thunk";
import { RegisterData, TokenResponse } from "src/types";
import { action } from "typesafe-actions";
import {
    SIGN_UP_SUCCESS,
    SIGN_UP_FAILURE,
    SIGN_UP_REQUEST,
} from "./actionTypes";
import { API } from "src/config"

export const signUpRequest = () => {
    return action(SIGN_UP_REQUEST);
};

export const signUpSuccess = (userId: number) => {
    return action(SIGN_UP_SUCCESS, userId);
};

export const signUpFailure = () => {
    return action(SIGN_UP_FAILURE);
};

export const signUp = (user: RegisterData, router: NextRouter) => async (
    dispatch: ThunkDispatch<{}, {}, AnyAction>
) => {
    dispatch(signUpRequest());
    try {
        const res = await axios.post(`${API}/signup`, user);
        const tokenRes: TokenResponse = res.data.tokens;
        const userId: number = res.data.userId;
        localStorage.setItem("accessToken", tokenRes.accessToken);
        localStorage.setItem("refreshToken", tokenRes.refreshToken);
        dispatch(signUpSuccess(userId));
        router.push("/");
    } catch (error) {
        console.error(error)
        dispatch(signUpFailure());
    }
};
