import { Actions, AuthState } from "types";

export const auth = (state: AuthState = {} as AuthState, action: Actions) => {
    switch (action.type) {
        case "auth/SIGN_UP_SUCCESS":
            return {
                userId: action.payload,
            };
        case "auth/LOGIN_SUCCESS":
            return {
                userId: action.payload,
            };
        default:
            return state;
    }
};
