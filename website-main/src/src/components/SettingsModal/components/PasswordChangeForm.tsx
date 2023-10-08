import { useState } from "react";
import { Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";
import PasswordInput from "../../PasswordInput/PasswordInput";

interface PasswordChangeProps {
  loading: boolean;
  onSubmit: (password: string) => void;
}

const PasswordChangeForm = ({ loading, onSubmit }: PasswordChangeProps) => {
  const [password, setPassword] = useState<string>("");

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    onSubmit(password);
  }

  return (
    <Form onSubmit={(e) => handleSubmit(e)} className="form-container">
      <Form.Group controlId="newPassword" className="form-container">
        <Form.Label>{language.dictionary.newPassword}</Form.Label>
        <PasswordInput value={password} onChange={(e) => setPassword(e.currentTarget.value)} />
      </Form.Group>
      <Button type="submit" variant="primary" disabled={!password || loading}>
        {loading ?
          <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
          :
          language.dictionary.change
        }
      </Button>
    </Form>
  )
}

export default PasswordChangeForm