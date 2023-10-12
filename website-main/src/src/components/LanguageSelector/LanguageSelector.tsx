import "./languageSelector.scss"
import { Dropdown } from "react-bootstrap";
import { Language, languages } from "../../translations";
import { language } from "../../main";

const LanguageSelector = () => {
  const onLanguageClick = (language: Language) => {
    localStorage.setItem("language", language.code);
    window.location.reload();
  }
  return (
    <Dropdown>
      <Dropdown.Toggle variant="default" id="dropdownMenu1" className="language">
        <img src={language.flagPath} className="flag" />{language.name}
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
