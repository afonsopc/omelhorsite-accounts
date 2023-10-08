import React, { useState } from 'react'
import { Button, Form, InputGroup } from 'react-bootstrap'
import { language } from '../../main'

interface PasswordInputProps {
    value: string;
    onChange: (e: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
}

const PasswordInput = ({ value, onChange }: PasswordInputProps) => {
    const [showPassword, setShowPassword] = useState(false);

    return (
        <InputGroup>
            <Form.Control
                type={showPassword ? "text" : "password"}
                placeholder={language.dictionary.enterPassword}
                required
                value={value}
                onChange={(e) => onChange(e)}
            />
            <Button
                variant="outline-secondary"
                onClick={() => setShowPassword(!showPassword)}
            >
                {showPassword ? language.dictionary.hide : language.dictionary.show}
            </Button>
        </InputGroup>
    )
}

export default PasswordInput