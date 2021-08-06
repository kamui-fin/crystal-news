import { Source } from "types";
import Link from "next/link";
import styles from "./style.module.scss";

interface Props {
    sources: Source[];
}

const SidebarSources: React.FC<Props> = ({ sources }: Props) => {
    return (
        <div className={styles.sources}>
            {sources.map((s: Source) => (
                <div className={styles.source} key={s.sourceId}>
                    <Link href={`/feed/${s.sourceId}`}>{s.title}</Link>
                </div>
            ))}
        </div>
    );
};

export default SidebarSources;
