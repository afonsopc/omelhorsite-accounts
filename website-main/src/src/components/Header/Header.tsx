import "./header.scss";
import { Navbar, Container, Image } from "react-bootstrap";
import { language } from "../../main";
import AccountDropdown from "../AccountDropdown/AccountDropdown";
import LanguageSelector from "../LanguageSelector/LanguageSelector";
import { useState } from "react";

export interface HeaderProps {
  validAccount: boolean;
}

function Header({ validAccount }: HeaderProps) {
  const [selectedLanguage, setSelectedLanguage] = useState(language);

  return (
    <Navbar bg="light" variant="light" expand="lg" className="border-bottom">
      <Container>
        <Navbar.Brand href="/" className="brand-container gap-4">
          <Image
            src="/logo.svg"
            alt={language.dictionary.logotype}
            style={{ width: "50px", height: "auto" }}
          />
          <span className="brand-name">{language.dictionary.websiteName}</span>
        </Navbar.Brand>
        <AccountDropdown />
      </Container>
    </Navbar>
  );
}

export default Header;
