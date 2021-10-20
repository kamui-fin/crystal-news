import Feed from "components/Feed";
import { axiosInstance } from "lib/utils";
import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { Article, RootState } from "types";

const All = () => {
    const isLoggedIn = useSelector<RootState, boolean>(
        (state) => state.auth.isLoggedIn
    );
    const [articles, setArticles] = useState<{
        articles: Article[];
        fetched: boolean;
    }>({ articles: [], fetched: false });

    const loadArticles = async () => {
        if (isLoggedIn && !articles.fetched) {
            setArticles({
                ...articles,
                fetched: false,
            });
            const res = await axiosInstance.get("/feed/all");
            setArticles({ articles: res.data, fetched: true });
        }
    };

    useEffect(() => {
        loadArticles();
    });

    return (
        <>
            {articles.fetched ? (
                <Feed articles={articles.articles} />
            ) : (
                <p>Loading...</p>
            )}
        </>
    );
};

export default All;
