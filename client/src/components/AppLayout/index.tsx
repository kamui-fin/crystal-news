import styles from "./style.module.scss";
import SidebarSources from "components/SidebarSources";
import { useEffect, useState } from "react";
import { axiosInstance } from "lib/utils";
import { useSelector } from "react-redux";
import { RootState } from "types";

interface Props {
    children: React.ReactNode;
}

const AppLayout = (props: Props) => {
    const [sources, setSources] = useState([]);
    const isLoggedIn = useSelector<RootState, boolean>(
        (state) => state.auth.isLoggedIn
    );

    useEffect(() => {
        (async () => {
            if (isLoggedIn && sources.length == 0) {
                const res = await axiosInstance.get("/sources");
                setSources(res.data);
                console.log(res);
            }
        })();
    });

    return (
        <div className={styles.app}>
                <SidebarSources sources={sources} />
            {props.children}
        </div>
    );
};

export default AppLayout;
