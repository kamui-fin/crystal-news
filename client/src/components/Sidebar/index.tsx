import { Source } from "types";
import Link from "next/link";
import styles from "./style.module.scss";
import classnames from "classnames";

interface Props {
    sources: Source[];
}

const Sidebar: React.FC<Props> = ({ sources }: Props) => {
    return (
        <div className={styles.sidebar}>
            <div className={styles.top}>
                <div className={styles.logo}>
                    <h3>Crystal News</h3>
                </div>
                {/* TODO: Theme toggle */}
                <div className={styles.themeSetter}></div>
            </div>
            {/* Setup modal here */}
            <div className={classnames(styles.addSource, styles.btn)}>
                Add source
            </div>
            <div className={styles.quickLinks}>
                <Link href="/feed/all">All time feed</Link>
                <Link href="/feed/today">Today's feed</Link>
                <Link href="/feed/recently-read">Recently read</Link>
                <Link href="/bookmarks">Bookmarks</Link>
            </div>
            <ul className={styles.sources}>
                <h3>Sources</h3>
                {sources.map((s: Source) => (
                    <li className={styles.source} key={s.sourceId}>
                        <Link href={`/feed/${s.sourceId}`}>{s.title}</Link>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default Sidebar;
