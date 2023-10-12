import { Form } from "react-bootstrap"
import { language } from "../../../main"
import PasswordInput from "../../PasswordInput/PasswordInput"

interface SignUpProps {
    username: string;
    onUsernameChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
    email: string;
    onEmailChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
    password: string;
    onPasswordChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
}

const SignUpForm = ({
    username, onUsernameChange,
    email, onEmailChange,
    password, onPasswordChange,
}: SignUpProps) => {

    return (
        <Form className="form">
            <Form.Group controlId="username">
                <Form.Label>{language.dictionary.email}</Form.Label>
                <Form.Control
                    type="email"
                    placeholder={language.dictionary.enterEmail}
                    value={email}
                    onChange={(e) => onEmailChange(e)}
                />
            </Form.Group>

            <Form.Group>
                <Form.Label>{language.dictionary.username}</Form.Label>
                <Form.Control
                    placeholder={language.dictionary.enterUsername}
                    value={username}
                    onChange={(e) => onUsernameChange(e)}
                />
            </Form.Group>

            <Form.Group controlId="password">
                <Form.Label>{language.dictionary.password}</Form.Label>
                <PasswordInput required value={password} onChange={(e) => onPasswordChange(e)} />
            </Form.Group>
        </Form >
    )
}

export default SignUpForm