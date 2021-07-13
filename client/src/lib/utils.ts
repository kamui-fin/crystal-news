import { getCookieFromBrowser, getCookieFromServer } from "./cookie";
import { Request, Tokens } from "types"

export const getTokensFromServer = (request: Request): Tokens | null => {
    const accessToken = getCookieFromServer('accessToken', request);
    const refreshToken = getCookieFromServer('refreshToken', request);
    if (!accessToken || !refreshToken) {
        return null;
    }
    return {
        accessToken,
        refreshToken
    }
}

export const getTokensFromClient = (): Tokens | null => {
    const accessToken = getCookieFromBrowser('accessToken');
    const refreshToken = getCookieFromBrowser('refreshToken');
    if (!accessToken || !refreshToken) {
        return null;
    }
    return {
        accessToken,
        refreshToken
    }
}


export const getTokensFromStore = (store): Tokens | null => {
    return store.getState().auth?.tokens;
}

export const authGuardSSR = (context) => {
    const redirection = {
        redirect: {
            destination: '/login',
            permanent: false,
        }
    };
    if (context.req.headers.cookie) {
        const tokens = getTokensFromServer(context.req);
        if (!tokens) {
            return redirection;
        }
    } else {
        return redirection;
    }
    return null;
}
