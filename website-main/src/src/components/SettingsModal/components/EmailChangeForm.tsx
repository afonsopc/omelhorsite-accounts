import { useState } from "react";
import { Alert, Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";

const EmailChangeForm = () => {
    const [newEmail, setNewEmail] = useState<string>("");
    const [showNewEmailErrorMessage, setShowNewEmailErrorMessage] = useState<boolean>(false);
    const [showNewEmailSuccessMessage, setShowNewEmailSuccessMessage] = useState<boolean>(false);
    const [loadingNewEmail, setLoadingNewEmail] = useState<boolean>(false);
    const [newEmailErrorMessage, setNewEmailErrorMessage] = useState<string>("");

  return (
    <Form className="form-container">
        <Form.Group controlId="email">
            <Form.Label>{language.dictionary.newEmail}</Form.Label>
            
            <Form.Control
            type="email"
            placeholder={language.dictionary.enterNewEmail}
            value={newEmail}
            onChange={(e) => setNewEmail(e.target.value)}
            autoComplete="email"
            />
        </Form.Group>

        <Alert className="alert-danger" hidden={!showNewEmailErrorMessage}>{newEmailErrorMessage}</Alert>
        <Alert className="alert-success" hidden={!showNewEmailSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>

        <Button variant="primary" type="submit" disabled={!newEmail || loadingNewEmail}>
            {loadingNewEmail ? 
                <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
            :
                language.dictionary.change
            }
        </Button>
    </Form>
  )
}

export default EmailChangeForm