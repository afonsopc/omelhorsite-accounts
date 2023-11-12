import "./userPicture.scss";

interface UserPictureProps {
    src?: string;
    className?: string;
}

const UserPicture = ({ src, className }: UserPictureProps) => {
    return (
        <img
            src={src}
            className={"user-picture" + (className ? ` ${className}` : "")}
        />
    )
}

export default UserPicture