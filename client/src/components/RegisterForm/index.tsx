import { useForm } from "react-hook-form";
import { useRouter } from "next/dist/client/router";
import { signUp } from "store/actions/authActions";
import { RegisterData } from "types";
import { yupResolver } from '@hookform/resolvers/yup';
import styles from "./style.module.scss"
import Link from "next/link"
import classNames from "classnames/bind";
import * as yup from 'yup';
import { useDispatch } from "react-redux";

const RegisterForm: React.FC = () => {
    const router = useRouter();
    const dispatch = useDispatch();
    const cx = classNames.bind(styles);

    const schema = yup.object().shape({
        username: yup.string().required("Username cannot be empty").min(3, "Username must be atleast 3 characters long"),
        email: yup.string().required("Email cannot be empty").email("Must be a valid email"),
        password: yup.string().required("Password cannot be empty").min(6, "Password must be atleast 6 characters long"),
        confirmPassword: yup.string().required("Password confirmation is required").oneOf([yup.ref('password'), null], "Passwords don't match.")
    });

    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<RegisterData>({ resolver: yupResolver(schema) });
    const onSubmit = async (data: RegisterData) => {
        dispatch(signUp(data, router));
    };

    return (
        <div className={styles.container}>
            <h3 className={styles.welcome}>Welcome Back</h3>
            <p className={styles.prompt}>Enter your credentials to access your account</p>

            <form className={styles.form} onSubmit={handleSubmit(onSubmit)}>
                <div className={styles.inputContainer}>
                    <input {...register("username")} className={cx("inputBox", { "inputError": errors.username })} placeholder="Enter a username" />
                    <p className={styles.error}>{errors.username?.message}</p>
                </div>

                <div className={styles.inputContainer}>
                    <input {...register("email")} className={cx("inputBox", { "inputError": errors.email })} placeholder="Enter an email" />
                    <p className={styles.error}>{errors.email?.message}</p>
                </div>

                <div className={styles.inputContainer}>
                    <input {...register("password")} className={cx("inputBox", { "inputError": errors.password })} type="password" placeholder="Enter a password" />
                    <p className={styles.error}>{errors.password?.message}</p>
                </div>

                <div className={styles.inputContainer}>
                    <input {...register("confirmPassword")} className={cx("inputBox", { "inputError": errors.password })} type="password" placeholder="Confirm your password" />
                    <p className={styles.error}>{errors.confirmPassword?.message}</p>
                </div>

                <input className={styles.submitButton} type="submit" value="Register" />
            </form>

            <p className={styles.loginLink}>Already have an account? <Link href="/login">Login</Link></p>
        </div>
    );
}

export default RegisterForm;
