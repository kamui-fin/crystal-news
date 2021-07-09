import AuthGuard from "middleware/authGuard";

const Main = () => {
    return (
        <AuthGuard>
            <div>
                Feed
            </div>
        </AuthGuard>
    )
};

export default Main;
