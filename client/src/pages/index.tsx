import { wrapper } from "store";
import SidebarSources from "components/SidebarSources";
import { Source } from "types";
import { authServer, axiosInstance, interceptors } from "lib/utils";

interface AppData {
    sources: Source[];
}

const Main = (data: AppData) => {
    return (
        <div>
            <SidebarSources sources={data.sources} />
        </div>
    );
};

export const getServerSideProps = wrapper.getServerSideProps(
    (store) => async (context) => {
        interceptors(store);
        const res = await authServer(store, context);
        if (res) return res;
        const sources = await axiosInstance.get("/sources");
        return {
            props: {
                sources: sources.data,
            },
        };
    }
);

export default Main;
