import type { AppProps } from "next/app";
import { FC, useEffect } from "react";
import { wrapper } from "../store";
import { axiosInstance, interceptors } from "lib/utils";
import { useRouter } from "next/router";
import "styles/global.scss";
import { useDispatch, useStore } from "react-redux";
import { authenticate } from "store/actions/authActions";

const App: FC<AppProps> = ({ Component, pageProps }: AppProps) => {
    const router = useRouter();
    const dispatch = useDispatch();
    const store = useStore();

    useEffect(() => {
        interceptors(store);
        const restoreSession = async () => {
            try {
                const tokens = await axiosInstance.post(`/refreshToken`);
                dispatch(authenticate(tokens.data.token))
            } catch (e) {
                router.push("/login");
            }
        }
        restoreSession();
    }, [])

    return <Component {...pageProps} />;
};

export default wrapper.withRedux(App);
