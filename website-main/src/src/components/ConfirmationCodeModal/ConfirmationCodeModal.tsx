import "./confirmationCodeModal.scss";
import React, { useState } from "react";
import { Modal, Button, Form, Alert } from "react-bootstrap";
import { language } from "../../main";
import { cancelConfirmations } from "../../services/accountService";

interface ConfirmationCodeModalProps {
    show: boolean;
    showErrorMessage: boolean,
    onHide: () => void;
    onConfirm: (value: string) => void;
}

const ConfirmationCodeModal: React.FC<ConfirmationCodeModalProps> = ({
    show,
    showErrorMessage,
    onHide,
    onConfirm,
}) => {
    const [code, setCode] = useState("");

    const handleCancel = async () => {
        cancelConfirmations();
        onHide();
    };

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
                <p>{language.dictionary.confirmationEmailSent}</p>
                <p>{language.dictionary.toConfirmInsertConfirmationCodeBellow}</p>
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
            </Modal.Body>
            <Alert className="alert-danger" hidden={!showErrorMessage}>{language.dictionary.invalidConfirmationCode}</Alert>
            <Modal.Footer>
                <Button variant="secondary" onClick={() => handleCancel()}>
                    {language.dictionary.cancel}
                </Button>
                <Button variant="primary" onClick={() => onConfirm(code)}>
                    {language.dictionary.confirm}
                </Button>
            </Modal.Footer>
        </Modal>
    );
};

export default ConfirmationCodeModal;