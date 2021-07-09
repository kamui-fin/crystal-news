import { useForm } from "react-hook-form";
import { useRouter } from "next/dist/client/router";
import { LoginData } from "types";
import { useDispatch } from "react-redux";
import { login } from "store/actions/authActions";

const LoginForm: React.FC = () => {
    const router = useRouter();
    const dispatch = useDispatch();

    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<LoginData>();
    const onSubmit = async (data: LoginData) => {
        dispatch(login(data, router));
    };

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <label>Username or Email</label>
            <input {...register("usernameOrEmail")} />
            <label>Password</label>
            <input {...register("password")} />
            <input type="submit" value="Login" />
        </form>
    );
}

export default LoginForm;
