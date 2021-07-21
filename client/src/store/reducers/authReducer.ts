import { Actions, AuthState } from "types";
import { Reducer } from "typesafe-actions";

export const auth: Reducer<AuthState, Actions> = (
    state: AuthState = { token: null } as AuthState,
    action: Actions
): AuthState => {
    switch (action.type) {
        case "auth/authenticate":
            return { token: action.payload.token };
        case "auth/deauthenticate":
            return { token: null };
        default:
            return state;
    }
};
