import { useState } from "react";
import { Alert, Button, Form, Spinner } from "react-bootstrap"
import { language } from "../../../main";
import LanguageSelector from "../../LanguageSelector/LanguageSelector";
import { Language } from "../../../translations";

const LanguageChangeForm = () => {
  const [selectedLanguage, setSelectedLanguage] = useState<Language>(language); // Estado para armazenar o idioma selecionado

  return (
    <Form.Group controlId="selectedLanguage" className="form-container">
      <Form.Label>{language.dictionary.language}</Form.Label>
      <LanguageSelector
        selectedLanguage={selectedLanguage}
        onChange={(language) => setSelectedLanguage(language)}
      />
      <Button variant="primary" disabled={selectedLanguage === language}>
        {language.dictionary.change}
      </Button>
    </Form.Group>
  )
}

export default LanguageChangeForm