import { getTokensFromClient } from "lib/utils";
import { Actions, AuthState } from "types";
import { Reducer } from "typesafe-actions";

export const auth: Reducer<AuthState, Actions> = (state: AuthState = { tokens: getTokensFromClient() } as AuthState, action: Actions): AuthState => {
    switch (action.type) {
        case "auth/authenticate":
            return { tokens: action.payload.tokens }
        case "auth/deauthenticate":
            return { tokens: null }
        default:
            return state;
    }
};
