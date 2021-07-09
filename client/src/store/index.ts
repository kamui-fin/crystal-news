import { Context, createWrapper, HYDRATE } from "next-redux-wrapper";
import {
    AnyAction,
    applyMiddleware,
    combineReducers,
    createStore,
    Middleware,
    Store,
} from "redux";
import thunk from "redux-thunk";
import { RootState } from "types";
import { auth } from "./reducers/authReducer";
import { composeWithDevTools } from "redux-devtools-extension";

const bindMiddleware = (middleware: Middleware[]) => {
    if (process.env.NODE_ENV !== 'production') {
        return composeWithDevTools(applyMiddleware(...middleware))
    }
    return applyMiddleware(...middleware)
}

const reducer = (state: RootState = {} as RootState, action: AnyAction) => {
    if (action.type == HYDRATE) {
        return { ...state, ...action.payload }
    }
    else {
        return combineReducers({ auth })
    }
}

export const initStore = (context: Context) => {
    return createStore(reducer, bindMiddleware([thunk]));
};

export const wrapper = createWrapper<Store<RootState>>(initStore, {
    debug: true,
});
