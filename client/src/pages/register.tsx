import RegisterForm from "components/RegisterForm"
import { useUnauthorizedOnly } from "lib/hooks";

const Register = () => {
    useUnauthorizedOnly();
    return (
        <RegisterForm />
    )
};

export default Register;
