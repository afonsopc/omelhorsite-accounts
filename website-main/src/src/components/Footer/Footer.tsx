import { Container } from "react-bootstrap";
import "./footer.scss";
import { language } from "../../main";

const Footer = () => {
  return (
    <footer style={{ backgroundColor: "#f8f9fa", padding: "20px" }} className="border-top">
      <Container>
        <div className="d-flex justify-content-center align-items-center footer-content">
          <img
            src="/logo.svg"
            alt="Logo"
            style={{ width: "50px", height: "50px", marginRight: "10px" }}
          />
          <span>{language.dictionary.websiteName} &copy; {language.dictionary.copyright}</span>
        </div>
      </Container>
    </footer>
  )
}

export default Footer