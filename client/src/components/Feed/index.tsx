import styles from "./style.module.scss";
import { Article } from "types";

interface Props {
    articles: Article[];
}

const Feed = (props: Props) => {
    const { articles } = props;
    return (
        <div>
            {articles.map((a) => (
                <p>{a.title}</p>
            ))}
        </div>
    );
};

export default Feed;
