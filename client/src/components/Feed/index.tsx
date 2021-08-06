import styles from "./style.module.scss";
import { FeedSelection } from "types";
import { useEffect, useState } from "react";
import { axiosInstance } from "lib/utils";
import { useSelector } from "react-redux";
import { RootState } from "types";

interface Props {
    selectionType: FeedSelection;
}

const Feed = (props: Props) => {
    const [articles, setArticles] = useState([]);
    const isLoggedIn = useSelector<RootState, boolean>((state) => state.auth.isLoggedIn);
    useEffect(() => {
        (async () => {
            if (isLoggedIn && props.selectionType == "ALL") {
                const res = await axiosInstance.post("/feed");
                setArticles(res.data);
            }
        })();
    });
    return <div></div>;
};

export default Feed;
