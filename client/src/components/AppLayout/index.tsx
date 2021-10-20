import styles from "./style.module.scss";
import Sidebar from "components/Sidebar";
import { useEffect, useState } from "react";
import { axiosInstance } from "lib/utils";
import { useSelector } from "react-redux";
import { RootState } from "types";

interface Props {
    children: React.ReactNode;
}

const AppLayout = (props: Props) => {
    const [sources, setSources] = useState({ sources: [], fetched: false });
    const isLoggedIn = useSelector<RootState, boolean>(
        (state) => state.auth.isLoggedIn
    );

    const loadUserSubs = async () => {
        if (isLoggedIn && !sources.fetched) {
            const res = await axiosInstance.get("/sources");
            setSources({ sources: res.data, fetched: res.status == 200 });
        }
    };

    useEffect(() => {
        loadUserSubs();
    });

    return (
        <div className={styles.app}>
            <Sidebar sources={sources.sources} />
            <div className={styles.children}>{props.children}</div>
        </div>
    );
};

export default AppLayout;
