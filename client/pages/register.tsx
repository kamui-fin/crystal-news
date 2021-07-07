import { useForm } from "react-hook-form";
import axios from "axios";

interface RegisterData {
    username: String;
    password: String;
    confirmPassword: String;
    email: String;
};

interface TokenResponse {
    accessToken: String,
    refreshToken: String,
}

const Register = () => {
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<RegisterData>();
    const onSubmit = async (data: RegisterData) => {
        const res = await axios.post("http://localhost:8080/signup", data);
        const tokenRes: TokenResponse = res.data;
        console.log(tokenRes);
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
