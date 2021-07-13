import LoginForm from "components/LoginForm"
import { useUnauthorizedOnly } from "lib/hooks";

const Login = () => {
    useUnauthorizedOnly();

    return (
        <LoginForm />
    )
};

export default Login;
