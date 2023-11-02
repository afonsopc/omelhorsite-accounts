import { useState } from "react";
import { Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";

interface EmailChangeFormProps {
    currentEmail: string;
    loading: boolean;
    onSubmit: (email: string) => void;
}

const EmailChangeForm = ({ currentEmail, loading, onSubmit }: EmailChangeFormProps) => {
    const [email, setEmail] = useState<string>("");

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        onSubmit(email);
    }

    return (
        <Form onSubmit={(e) => handleSubmit(e)} className="form-container">
            <Form.Label>{language.dictionary.currentEmail}: {currentEmail}</Form.Label>
            <Form.Group controlId="email">
                <Form.Label>{language.dictionary.newEmail}</Form.Label>

                <Form.Control
                    type="email"
                    placeholder={language.dictionary.enterNewEmail}
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    autoComplete="email"
                />
            </Form.Group>

            <Button variant="primary" type="submit" disabled={!email || loading}>
                {loading ?
                    <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
                    :
                    language.dictionary.change
                }
            </Button>
        </Form>
    )
}

export default EmailChangeForm