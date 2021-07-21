import { Source } from "types";
import styles from "./style.module.scss";

interface Props {
    sources: Source[];
}

const SidebarSources: React.FC<Props> = ({ sources }: Props) => {
    const src = sources.map((s: Source) => (
        <div className={styles.source} key={s.sourceId}>
            <a href={s.website}>{s.title}</a>
        </div>
    ));
    return <>{src}</>;
};

export default SidebarSources;
