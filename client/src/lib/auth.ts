import axios from "axios"
import { API } from "config";
import { SavedUser } from "types";

export const saveUserData = (user: SavedUser) => {
    localStorage.setItem("accessToken", user.accessToken);
    localStorage.setItem("refreshToken", user.refreshToken);
    localStorage.setItem("userId", user.userId.toString());
}

export const getAccessToken = (): string | null => {
    return localStorage.getItem("accessToken");
}

export const getRefreshToken = (): string | null => {
    return localStorage.getItem("refreshToken");
}

export const getLoggedInUserId = (): string | null => {
    return localStorage.getItem("userId");
}

export const getUserData = (): SavedUser | null => {
    const accessToken = getAccessToken();
    const refreshToken = getRefreshToken();
    const userId = getLoggedInUserId();

    if (!accessToken || !refreshToken || !userId) {
        return null;
    }

    return {
        accessToken,
        refreshToken,
        userId: Number.parseInt(userId)
    }
}

export const isValidToken = async (accessToken: string): Promise<boolean> => {
    try {
        await axios.get(`${API}/authCheck`, {
            headers: `Authorization: Bearer ${accessToken}`
        });
        return true;
    }
    catch (error) {
        return false;
    }
}
