import { getCookieFromServer } from "lib/cookie";
import { GetServerSideProps, GetServerSidePropsResult } from "next";

const Main = () => {
    return (
        <div> Feed
        </div>
    )
};

export const getServerSideProps: GetServerSideProps = async (context) => {
    const redirection: GetServerSidePropsResult<{}> = {
        redirect: {
            destination: '/login',
            permanent: false,

        }
    };
    if (context.req.headers.cookie) {
        const accessToken = getCookieFromServer('accessToken', context.req);
        const refreshToken = getCookieFromServer('refreshToken', context.req);
        if (!accessToken || !refreshToken) {
            return redirection;
        }
    } else {
        return redirection;
    }
    return {
        props: {}
    }
}

export default Main;
