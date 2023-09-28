import "./languageSelector.scss"
import React from "react";
import { Dropdown } from "react-bootstrap";
import { Languages, Language } from "../../translations";

interface LanguageSelectorProps {
  selectedLanguage: Language;
  onChange: (language: Language) => void;
  languages: Languages;
}

const LanguageSelector: React.FC<LanguageSelectorProps> = ({
  selectedLanguage,
  onChange,
  languages,
}) => {
    const onLanguageClick = (language: Language) => {
        onChange(language);
        console.log(language);
        
    }
  return (
    <Dropdown>
      <Dropdown.Toggle variant="default" id="dropdownMenu1" className="language">
        <img src={selectedLanguage.flagPath} className="flag"/>{selectedLanguage.name}
      </Dropdown.Toggle>
      <Dropdown.Menu>
        {Object.values(languages).map((language: Language) => (
            <Dropdown.Item key={language.code} className="language" onClick={() => onLanguageClick(language)}>
                <img src={language.flagPath} className="flag"/>{language.name}
            </Dropdown.Item>
        ))}
      </Dropdown.Menu>
    </Dropdown>
  );
};

export default LanguageSelector;
