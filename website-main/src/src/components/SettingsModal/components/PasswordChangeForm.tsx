import { useState } from "react";
import { Alert, Button, Form, InputGroup, Spinner } from "react-bootstrap"
import { language } from "../../../main";

const PasswordChangeForm = () => {
    const [newPassword, setNewPassword] = useState<string>("");
    const [showNewPassword, setShowNewPassword] = useState(false);
    const [showNewPasswordErrorMessage, setShowNewPasswordErrorMessage] = useState<boolean>(false);
    const [showNewPasswordSuccessMessage, setShowNewPasswordSuccessMessage] = useState<boolean>(false);
    const [loadingNewPassword, setLoadingNewPassword] = useState<boolean>(false);
    const [newPasswordErrorMessage, setNewPasswordErrorMessage] = useState<string>("");

  return (
    <Form className="form-container">
      <Form.Group controlId="newPassword" className="form-container">
        <Form.Label>{language.dictionary.newPassword}</Form.Label>
        <InputGroup>
          <Form.Control
            type={showNewPassword ? "text" : "password"}
            placeholder={language.dictionary.enterNewPassword}
            required
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            autoComplete="new-password"
          />
          <Button
            variant="outline-secondary"
            onClick={() => setShowNewPassword(!showNewPassword)}
          >
            {showNewPassword ? `${language.dictionary.hide}` : `${language.dictionary.show}`}
          </Button>
        </InputGroup>
      </Form.Group>
      <Alert className="alert-danger" hidden={!showNewPasswordErrorMessage}>{newPasswordErrorMessage}</Alert>
      <Alert className="alert-success" hidden={!showNewPasswordSuccessMessage}>{language.dictionary.confirmationEmailSent}</Alert>
      <Button variant="primary" type="submit" disabled={!newPassword || loadingNewPassword}>
            {loadingNewPassword ?
                <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }}/>
              :
                language.dictionary.change
            }
      </Button>
    </Form>
  )
}

export default PasswordChangeForm