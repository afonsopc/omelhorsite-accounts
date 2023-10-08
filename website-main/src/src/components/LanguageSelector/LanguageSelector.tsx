import "./languageSelector.scss"
import React from "react";
import { Dropdown } from "react-bootstrap";
import { Languages, Language, languages } from "../../translations";
import { language } from "../../main";

interface LanguageSelectorProps {
  selectedLanguage: Language;
  onChange: (language: Language) => void;
}

const LanguageSelector: React.FC<LanguageSelectorProps> = ({
  selectedLanguage,
  onChange,
}) => {
  const onLanguageClick = (language: Language) => {
    onChange(language);
    console.log(language);

  }
  return (
    <Dropdown>
      <Dropdown.Toggle variant="default" id="dropdownMenu1" className="language">
        <img src={selectedLanguage.flagPath} className="flag" />{selectedLanguage.name}
      </Dropdown.Toggle>
      {Object.values(languages).length !== 1 ?
        <Dropdown.Menu>
          {Object.values(languages).map((languageEntry: Language) => (
            <Dropdown.Item hidden={language.code === languageEntry.code} key={languageEntry.code} className="language" onClick={() => onLanguageClick(languageEntry)}>
              <img src={languageEntry.flagPath} className="flag" />{languageEntry.name}
            </Dropdown.Item>
          ))}
        </Dropdown.Menu>
        :
        ""
      }
    </Dropdown>
  );
};

export default LanguageSelector;
