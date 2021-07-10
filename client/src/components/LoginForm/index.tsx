import { useForm } from "react-hook-form";
import { useRouter } from "next/dist/client/router";
import { yupResolver } from '@hookform/resolvers/yup';
import { LoginData } from "types";
import { useDispatch } from "react-redux";
import { login } from "store/actions/authActions";
import styles from "./style.module.scss"
import Link from "next/link"
import classNames from "classnames/bind";
import * as yup from 'yup';


const LoginForm: React.FC = () => {
    const cx = classNames.bind(styles);
    const router = useRouter();
    const dispatch = useDispatch();

    const schema = yup.object().shape({
        usernameOrEmail: yup.string().required("Username cannot be empty"),
        password: yup.string().required("Password cannot be empty"),
    });

    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<LoginData>({ resolver: yupResolver(schema) });

    const onSubmit = async (data: LoginData) => {
        dispatch(login(data, router));
    };

    return (
        <div className={styles.loginContainer}>
            <h3 className={styles.welcome}>Welcome Back</h3>
            <p className={styles.prompt}>Enter your credentials to access your account</p>

            <form className={styles.loginForm} onSubmit={handleSubmit(onSubmit)}>
                <div className={styles.inputContainer}>
                    <input {...register("usernameOrEmail")} className={cx("inputBox", { "inputError": errors.usernameOrEmail })} placeholder="Enter your username or email" />
                    <p className={styles.error}>{errors.usernameOrEmail?.message}</p>
                </div>

                <div className={styles.inputContainer}>
                    <input {...register("password")} className={cx("inputBox", { "inputError": errors.password })} type="password" placeholder="Enter your password" />
                    <p className={styles.error}>{errors.password?.message}</p>
                </div>

                <input className={styles.submitButton} type="submit" value="Sign In" />
            </form>

            <p className={styles.signUpLink}>Don&apos;t have an account? <Link href="/register">Sign Up</Link></p>
        </div>
    );
}

export default LoginForm;
