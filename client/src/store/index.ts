import { Context, createWrapper } from "next-redux-wrapper";
import {
    applyMiddleware,
    combineReducers,
    createStore,
    Reducer,
    Store,
} from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import thunk from "redux-thunk";
import { RootState } from "src/types";
import { auth } from "./reducers/authReducer";

const reducer = combineReducers({
    auth,
});

export const initStore = (context: Context) => {
    return createStore(reducer, composeWithDevTools(applyMiddleware(thunk)));
};

export const wrapper = createWrapper<Store<RootState>>(initStore, {
    debug: true,
});
