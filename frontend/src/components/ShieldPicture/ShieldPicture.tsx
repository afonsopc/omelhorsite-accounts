import "./shieldPicture.scss";

interface ShieldPictureProps {
    src?: string;
    className?: string;
}

const ShieldPicture = ({ src, className }: ShieldPictureProps) => {
    return (
        <img
            src={src}
            className={"shield-picture" + (className ? ` ${className}` : "")}
        />
    )
}

export default ShieldPicture