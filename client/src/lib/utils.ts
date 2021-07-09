import axios from "axios"
import { API } from "config";
import { SavedUser } from "types";

export const saveUserData = (user: SavedUser) => {
    localStorage.setItem("accessToken", user.accessToken);
    localStorage.setItem("refreshToken", user.refreshToken);
    localStorage.setItem("userId", user.userId.toString());
}

export const getUserData = (): SavedUser | null => {
    const accessToken = localStorage.getItem("accessToken");
    const refreshToken = localStorage.getItem("refreshToken");
    const userId = localStorage.getItem("userId");

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
        await axios.get(`${API}/me`, {
            headers: `Authorization: Bearer ${accessToken}`
        });
        return true;
    }
    catch (error) {
        return false;
    }
}
