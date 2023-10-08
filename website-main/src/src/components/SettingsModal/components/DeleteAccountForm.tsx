import { Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";

interface DeleteAccountFormProps {
  loading: boolean;
  onDelete: () => void;
}

const DeleteAccountForm = ({ loading, onDelete }: DeleteAccountFormProps) => {
  return (
    <Form.Group controlId="selectedLanguage" className="form-container">

      <Button
        variant="danger"
        disabled={loading}
        onClick={() => onDelete()}>
        {loading ?
          <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
          :
          language.dictionary.deleteAccount
        }
      </Button>
    </Form.Group>
  )
}

export default DeleteAccountForm