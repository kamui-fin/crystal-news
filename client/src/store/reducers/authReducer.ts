import { Actions, AuthState } from "src/types";

export const auth = (state: AuthState = {} as AuthState, action: Actions) => {
    switch (action.type) {
        case "auth/SIGN_UP_SUCCESS":
            return {
                userId: action.payload,
            };
        default:
            return state;
    }
};
