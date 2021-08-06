import AppLayout from "components/AppLayout";
import Feed from "components/Feed";
import { useRouter } from "next/router";

const SourceFeed = () => {
    const router = useRouter();
    const sourceId = Number.parseInt(router.query.id as string);

    return (
        <AppLayout>
            <Feed selectionType={sourceId}/>
        </AppLayout>
    );
};

export default SourceFeed;
