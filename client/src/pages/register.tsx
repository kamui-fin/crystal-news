import RegisterForm from "components/RegisterForm";
import { useUnauthOnly } from "lib/utils";

const Register = () => {
    useUnauthOnly();
    return <RegisterForm />;
};

export default Register;
