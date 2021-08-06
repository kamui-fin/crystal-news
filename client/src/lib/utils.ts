import axios from "axios";
import { API } from "config";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { useSelector, useStore } from "react-redux";
import { AuthState, RootState } from "types";
import { authenticate, deauthenticate } from "store/actions/authActions";
import { Store } from "redux";

export const axiosInstance = axios.create({
    baseURL: API,
    withCredentials: true,
});

export const interceptors = (store: Store<RootState>) => {
    axiosInstance.interceptors.request.use((config) => {
        const { token } = store.getState().auth;
        if (token) {
            config.headers["Authorization"] = `Bearer ${token}`;
        }
        return config;
    });

    axiosInstance.interceptors.response.use(
        (res) => res,
        async (err) => {
            const originalRequest = err.config;
            if (
                err.response.status === 401 &&
                originalRequest.url.includes("/refreshToken")
            ) {
                store.dispatch(deauthenticate());
            } else if (err.response.status === 401 && !originalRequest._retry) {
                originalRequest._retry = true;
                const tokens = await axiosInstance.post(`/refreshToken`, null, {
                    withCredentials: true,
                });
                store.dispatch(authenticate(tokens.data.token));
                return axios(originalRequest);
            }
            return Promise.reject(err);
        }
    );
};

export const useUnauthOnly = () => {
    const auth = useSelector<RootState, AuthState>(state => state.auth);
    const router = useRouter();
    useEffect(() => {
        if (auth.token) {
            router.push("/feed/all");
        }
    }, [auth])
}
