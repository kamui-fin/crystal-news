import type { AppProps } from "next/app";
import { FC, useEffect } from "react";
import { wrapper } from "../store";
import { axiosInstance, interceptors } from "lib/utils";
import { useRouter } from "next/router";
import { useDispatch, useStore } from "react-redux";
import { authenticate } from "store/actions/authActions";
import AppLayout from "components/AppLayout";
import "styles/global.scss";

const App: FC<AppProps> = ({ Component, pageProps }: AppProps) => {
    const router = useRouter();
    const dispatch = useDispatch();
    const store = useStore();

    const setupAuth = async () => {
        interceptors(store);
        const restoreSession = async () => {
            try {
                const tokens = await axiosInstance.post(`/refreshToken`);
                dispatch(authenticate(tokens.data.token));
            } catch (e) {
                router.push("/login");
            }
        };
        restoreSession();
    };

    useEffect(() => {
        setupAuth();
    }, []);

    return (
        <AppLayout>
            <Component {...pageProps} />
        </AppLayout>
    );
};

export default wrapper.withRedux(App);
