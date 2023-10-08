import "./confirmationCodeModal.scss";
import React, { useEffect, useState } from "react";
import { Modal, Button, Form, Alert, Spinner } from "react-bootstrap";
import { language } from "../../main";
import { cancelConfirmations } from "../../services/accountService";

interface ConfirmationCodeModalProps {
    show: boolean;
    showErrorMessage: boolean,
    onHide: () => void;
    onConfirm: (value: string) => void;
    message: string,
    loading: boolean;
}

const ConfirmationCodeModal: React.FC<ConfirmationCodeModalProps> = ({
    show,
    showErrorMessage,
    onHide,
    onConfirm,
    message,
    loading,

}) => {
    const [code, setCode] = useState("");

    const handleCancel = async () => {
        cancelConfirmations();
        onHide();
    };

    useEffect(() => {
        setCode("");
    }, [message])

    return (
        <Modal
            show={show}
            onHide={onHide}
            backdrop="static"
        >
            <Modal.Header>
                <Modal.Title>{language.dictionary.confirmationCodeModalTitle}</Modal.Title>
            </Modal.Header>
            <Modal.Body>
                <div className="text">
                    <h5>{message}</h5>
                    <br />
                    <p><small>{language.dictionary.toConfirmInsertConfirmationCodeBellow}</small></p>
                </div>
                <Form>
                    <Form.Group controlId="confirmationInput">
                        <Form.Control
                            type="number"
                            placeholder="Insira o código"
                            value={code}
                            onChange={(e) => setCode(e.target.value)}
                        />
                    </Form.Group>
                </Form>
                <br />
                <Alert className="alert-danger" hidden={!showErrorMessage}>{language.dictionary.invalidConfirmationCode}</Alert>
            </Modal.Body>
            <Modal.Footer>
                <Button variant="secondary" onClick={() => handleCancel()}>
                    {language.dictionary.cancel}
                </Button>
                <Button variant="primary" onClick={() => onConfirm(code)} disabled={loading}>
                    {loading ?
                        <Spinner animation="border" role="status" style={{ width: "1em", height: "1em" }} />
                        :
                        language.dictionary.confirm
                    }
                </Button>
            </Modal.Footer>
        </Modal>
    );
};

export default ConfirmationCodeModal;