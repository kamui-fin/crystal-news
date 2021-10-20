import Feed from "components/Feed";
import { axiosInstance } from "lib/utils";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { Article, RootState } from "types";

const SourceFeed = () => {
    const router = useRouter();
    const sourceId = Number.parseInt(router.query.id as string);

    const isLoggedIn = useSelector<RootState, boolean>(
        (state) => state.auth.isLoggedIn
    );
    const [articles, setArticles] = useState<{
        articles: Article[];
        fetched: boolean;
        sourceId: number;
    }>({ articles: [], fetched: false, sourceId });

    const loadArticles = async () => {
        if (
            sourceId != articles.sourceId ||
            (isLoggedIn && !articles.fetched)
        ) {
            setArticles({
                ...articles,
                fetched: false,
            });
            const res = await axiosInstance.get(`/feed/${sourceId}`);
            setArticles({ articles: res.data, fetched: true, sourceId });
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

export default SourceFeed;
