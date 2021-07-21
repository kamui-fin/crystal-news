import type { AppProps } from "next/app";
import { FC, useEffect } from "react";
import { wrapper } from "../store";
import { axiosInstance } from "lib/utils";
import { useRouter } from "next/router";
import "styles/global.scss";
import { useDispatch } from "react-redux";
import { actions } from "store/actions/authActions";

const App: FC<AppProps> = ({ Component, pageProps }: AppProps) => {
    const router = useRouter();
    const dispatch = useDispatch();

    useEffect(() => {
        const restoreSession = async () => {
            try {
                const tokens = await axiosInstance.post(`/refreshToken`);
                dispatch(actions.authenticate(tokens.data.token))
            } catch (e) {
                router.push("/login");
            }
        }
        restoreSession();
    }, [])

    return <Component {...pageProps} />;
};

export default wrapper.withRedux(App);
