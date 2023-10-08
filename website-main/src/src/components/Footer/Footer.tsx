import { Container } from "react-bootstrap";
import "./footer.scss";
import { language } from "../../main";
import LanguageSelector from "../LanguageSelector/LanguageSelector";
import { Language } from "../../translations";

const Footer = () => {
  const handleChangeLanguage = (language: Language) => {
    localStorage.setItem("language", language.code);
    window.location.reload();
  }

  return (
    <footer className="border-top footer">
      <Container className="footer-container">
        <img
          src="/logo.svg"
          style={{ width: "50px", height: "50px" }}
        />
        <span>{language.dictionary.websiteName} &copy; {language.dictionary.copyright}</span>
        <LanguageSelector
          selectedLanguage={language}
          onChange={(language) => handleChangeLanguage(language)}
        />
      </Container>
    </footer >
  )
}

export default Footer