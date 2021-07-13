import { authGuardSSR, getTokensFromServer } from "lib/utils";
import { wrapper } from "store";
import { actions } from "store/actions/authActions";

const Main = () => {
    return (
        <div>
            Feed
        </div>
    )
};

export const getServerSideProps = wrapper.getServerSideProps(store => context => {
    const redirection = authGuardSSR(context);
    if (redirection) {
        return redirection;
    }

    const tokens = getTokensFromServer(context.req);
    if (tokens) {
        store.dispatch(actions.authenticate(tokens));
    }
});

export default Main;
