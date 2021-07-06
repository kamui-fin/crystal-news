import { Context, createWrapper } from "next-redux-wrapper";
import { applyMiddleware, combineReducers, createStore, Store } from "redux"
import thunk from "redux-thunk"

export interface State { }

const reducer = combineReducers({});

export const initStore = (context: Context) => {
    return createStore(reducer, applyMiddleware(thunk));
}

export const wrapper = createWrapper<Store<State>>(initStore, { debug: true });
