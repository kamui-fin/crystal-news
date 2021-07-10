import { getLoggedInUserId } from "lib/auth";
import { Actions, AuthState } from "types";

const getInitialState = (): AuthState => {
    const userId = getLoggedInUserId();
    if (userId) {
        return {
            userId: Number.parseInt(userId),
            signedIn: true,
            failed: false,
        }
    } else {
        return {} as AuthState;
    }
}

export const auth = (state: AuthState = getInitialState(), action: Actions): AuthState => {
    switch (action.type) {
        case "auth/SIGN_UP_SUCCESS":
        case "auth/LOGIN_SUCCESS":
            return {
                userId: action.payload,
                signedIn: true,
                failed: false
            };
        case "auth/LOGIN_FAILURE":
        case "auth/SIGN_UP_FAILURE":
            return {
                ...state,
                signedIn: false,
                failed: true
            };
        case "auth/LOGIN_REQUEST":
        case "auth/SIGN_UP_REQUEST":
            return {
                ...state,
                signedIn: false,
                failed: false
            };
        default:
            return state;
    }
};
