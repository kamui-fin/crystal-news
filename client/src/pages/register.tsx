import { useForm } from "react-hook-form";
import { useRouter } from "next/dist/client/router";
import { signUp } from "src/store/actions/authActions";
import { RegisterData } from "src/types";
import { useDispatch } from "react-redux";

const Register: React.FC = () => {
    const router = useRouter();
    const dispatch = useDispatch();

    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<RegisterData>();
    const onSubmit = async (data: RegisterData) => {
        dispatch(signUp(data, router));
    };

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <label>Username</label>
            <input {...register("username")} />
            <label>Password</label>
            <input {...register("password")} />
            <label>Confirm password</label>
            <input {...register("confirmPassword")} />
            <label>Email</label>
            <input {...register("email")} />
            <input type="submit" value="Sign Up" />
        </form>
    );
};

export default Register;
