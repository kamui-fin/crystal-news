import type { AppProps } from "next/app";
import { FC } from "react";
import { wrapper } from "../store";

const App: FC<AppProps> = ({ Component, pageProps }: AppProps) => {
    return <Component {...pageProps} />;
};

export default wrapper.withRedux(App);
