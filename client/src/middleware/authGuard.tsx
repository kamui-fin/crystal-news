import { getUserData, isValidToken } from "lib/auth";
import { useRouter } from "next/dist/client/router";
import { useEffect, useState } from "react";

interface Props {
    children: React.ReactNode;
}

const AuthGuard: React.FC<Props> = ({ children }) => {
    const [loggedIn, setLoggedIn] = useState(false);
    const router = useRouter();

    useEffect(() => {
        const userData = getUserData();
        if (!userData || !isValidToken(userData.accessToken)) {
            router.push("/login");
        } else {
            setLoggedIn(true);
        }
    })

    return (
        <>
            {loggedIn && children}
        </>
    )
}

export default AuthGuard;
