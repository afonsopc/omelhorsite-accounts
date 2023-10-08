import { useState } from "react";
import { Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";

interface UsernameChangeProps {
    currentUsername: string;
    loading: boolean;
    onSubmit: (username: string) => void;
}

const UsernameChangeForm = ({ currentUsername, loading, onSubmit }: UsernameChangeProps) => {
    const [username, setUsername] = useState("");

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        onSubmit(username);
    }

    return (
        <Form onSubmit={(e) => handleSubmit(e)} className="form-container">
            <Form.Label>{language.dictionary.currentUsername}: {currentUsername}</Form.Label>

            <Form.Group controlId="username">
                <Form.Label>{language.dictionary.newUsername}</Form.Label>
                <Form.Control
                    type="username"
                    placeholder={language.dictionary.enterNewUsername}
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    autoComplete="username"
                />
            </Form.Group>

            <Button variant="primary" type="submit" disabled={!username || loading}>
                {loading ?
                    <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
                    :
                    language.dictionary.change
                }
            </Button>
        </Form>
    )
}

export default UsernameChangeForm