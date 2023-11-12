import "./welcomeText.scss"

interface WelcomeTextProps {
    userName?: string;
}

const WelcomeText = ({ userName }: WelcomeTextProps) => {
    return (
        <div>WelcomeText</div>
    )
}

export default WelcomeText