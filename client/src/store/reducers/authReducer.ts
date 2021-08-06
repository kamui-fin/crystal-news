import { Actions, AuthState } from "types";
import { Reducer } from "typesafe-actions";

export const auth: Reducer<AuthState, Actions> = (
    state: AuthState = { token: null, isLoggedIn: false } as AuthState,
    action: Actions
): AuthState => {
    switch (action.type) {
        case "auth/authenticate":
            return { token: action.payload.token, isLoggedIn: true };
        case "auth/deauthenticate":
            return { token: null, isLoggedIn: false };
        default:
            return state;
    }
};
