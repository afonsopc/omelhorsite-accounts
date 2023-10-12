import { Container } from "react-bootstrap";
import "./footer.scss";
import { language } from "../../main";
import LanguageSelector from "../LanguageSelector/LanguageSelector";

const Footer = () => {
  return (
    <footer className="border-top footer bg-body-secondary text-body-secondary">
      <Container className="footer-container">
        <img
          src="/logo.svg"
          style={{ width: "50px", height: "50px" }}
        />
        <span>{language.dictionary.websiteName} &copy; {language.dictionary.copyright}</span>
        <LanguageSelector />
      </Container>
    </footer >
  )
}

export default Footer