import { Actions, AuthState } from "types";

export const auth = (state: AuthState = {} as AuthState, action: Actions): AuthState => {
    switch (action.type) {
        case "auth/authenticate":
            return { tokens: action.payload.tokens }
        case "auth/deauthenticate":
            return { tokens: null }
        default:
            return state;
    }
};
