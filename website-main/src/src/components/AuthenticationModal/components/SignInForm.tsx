import { Form } from "react-bootstrap"
import { language } from "../../../main"
import PasswordInput from "../../PasswordInput/PasswordInput"

interface SignInProps {
    email: string;
    onEmailChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
    password: string;
    onPasswordChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
}

const SignInForm = ({
    email, onEmailChange,
    password, onPasswordChange,
}: SignInProps) => {

    return (
        <Form>
            <Form.Group controlId="email">
                <Form.Label>{language.dictionary.email}</Form.Label>
                <Form.Control
                    type="email"
                    placeholder={language.dictionary.enterEmail}
                    value={email}
                    onChange={(e) => onEmailChange(e)}
                />
            </Form.Group>

            <Form.Group controlId="password">
                <Form.Label>{language.dictionary.password}</Form.Label>
                <PasswordInput value={password} onChange={(e) => onPasswordChange(e)} />
            </Form.Group>
        </Form >
    )
}

export default SignInForm