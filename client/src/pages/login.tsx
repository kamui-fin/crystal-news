import LoginForm from "components/LoginForm";
import { useUnauthOnly } from "lib/utils";

const Login = () => {
    useUnauthOnly();
    return <LoginForm />;
};

export default Login;
