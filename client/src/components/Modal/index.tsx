import styles from "./style.module.scss";

const Modal = (props: { show: boolean; children: React.ReactChildren }) => {
    const showHideClassName = props.show
        ? "modal display-block"
        : "modal display-none";

    return (
        <div className={showHideClassName}>
            <section className="modal-main">
                {props.children}
                <button type="button" onClick={handleClose}>
                    Close
                </button>
            </section>
        </div>
    );
};

export default Modal;
