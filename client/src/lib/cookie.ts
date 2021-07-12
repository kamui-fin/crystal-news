import { Request } from "types"
import cookie from "js-cookie"

export const setCookie = (key: string, value: string) => {
    cookie.set(key, value, {
        expires: 1,
        sameSite: "strict",
        path: '/',
    })
}

export const removeCookie = (key: string) => {
    cookie.remove(key, {
        expires: 1,
    })
}

export const getCookieFromBrowser = (key: string) => {
    return cookie.get(key);
};

export const getCookieFromServer = (key: string, req: Request) => {
    if (!req.headers.cookie) {
        return undefined;
    }
    const rawCookie = req.headers.cookie
        .split(';')
        .find(c => c.trim().startsWith(`${key}=`));
    return rawCookie?.split('=')[1];
};
