import { useRouter } from "next/router";
import { useEffect } from "react";
import { RootState, AuthState } from "types";
import { useSelector } from "react-redux"

export const useUnauthorizedOnly = () => {
    const auth = useSelector<RootState, AuthState>(state => state.auth);
    const router = useRouter();

    useEffect(() => {
        if (auth.tokens) {
            router.push("/");
        }
    }, []);
}
