import ShieldPicture from "../ShieldPicture/ShieldPicture";
import "./logo.scss"

interface LogoProps {
    userPictureUrl?: string;
    className?: string;
}

const Logo = ({ userPictureUrl, className }: LogoProps) => {
    return (
        <div
            className={
                "logo-container" +
                (userPictureUrl ? " with-user-picture" : "") +
                (className ? ` ${className}` : "")
            }
        >
            <img className="picture website" src="/logo.svg" />
            {userPictureUrl ?
                <ShieldPicture className="picture user" src={userPictureUrl} />
                : ""
            }
        </div >
    )
}

export default Logo