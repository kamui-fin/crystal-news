import axios from "axios";
import { API } from "config";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { useSelector } from "react-redux";
import { AuthState, RootState } from "types";
import { actions } from "store/actions/authActions";
import { Store } from "redux";
import { GetServerSidePropsContext } from "next";
import { ParsedUrlQuery } from "querystring";

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
                store.dispatch(actions.deauthenticate());
            } else if (err.response.status === 401 && !originalRequest._retry) {
                originalRequest._retry = true;
                const tokens = await axiosInstance.post(`/refreshToken`, null, {
                    withCredentials: true,
                });
                store.dispatch(actions.authenticate(tokens.data.token));
                return axios(originalRequest);
            }
            return Promise.reject(err);
        }
    );
};

export const authServer = async (
    store: Store<RootState>,
    context: GetServerSidePropsContext<ParsedUrlQuery>
) => {
    try {
        const tokens = await axiosInstance.post(`/refreshToken`, null, {
            headers: {
                Cookie: context.req.headers.cookie,
            },
        });
        context.res.setHeader(
            "Set-Cookie",
            `refresh_token=${tokens.data.refreshToken};HttpOnly;Max-Age=${tokens.data.refreshTokenExpiry};Path=/`
        );
        store.dispatch(actions.authenticate(tokens.data.token));
    } catch (e) {
        context.res.setHeader(
            "Set-Cookie",
            `refresh_token=;HttpOnly;Max-Age=-1;Path=/`
        );
        return {
            redirect: {
                destination: "/login",
                permanent: false,
            },
        };
    }
};

export const useUnauthOnly = () => {
    const auth = useSelector<RootState, AuthState>(state => state.auth);
    const router = useRouter();
    useEffect(() => {
        if (auth.token) {
            router.push("/");
        }
    }, [auth])
}
