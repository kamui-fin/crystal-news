import { createWrapper, HYDRATE, MakeStore } from "next-redux-wrapper";
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
import { Reducer } from "typesafe-actions";
import { getTokensFromClient } from "lib/utils";

const bindMiddleware = (middleware: Middleware[]) => {
    if (process.env.NODE_ENV !== 'production') {
        return composeWithDevTools(applyMiddleware(...middleware))
    }
    return applyMiddleware(...middleware)
}

const initialState = {
    auth: {
        tokens: getTokensFromClient()
    }
};

const combinedReducer = combineReducers({ auth })

const reducer: Reducer<RootState, AnyAction> = (state: RootState = initialState, action: AnyAction) => {
    if (action.type == HYDRATE) {
        const nextState = {
            ...state,
            ...action.payload,
        }
        if (state.auth.tokens) nextState.auth.tokens = state.auth.tokens;
        return nextState;
    } else {
        return combinedReducer(state, action);
    }
}

export const initStore: MakeStore<Store<RootState>> = () => createStore(reducer, bindMiddleware([thunk]));

export const wrapper = createWrapper(initStore, {
    debug: true,
    // serializeState: state => {
    //     console.dir(state)
    //     return JSON.stringify(initialState);
    // }
});
