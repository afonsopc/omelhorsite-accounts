import "./header.scss";
import { Navbar, Container, Image } from "react-bootstrap";
import { language } from "../../main";
import AccountDropdown from "../AccountDropdown/AccountDropdown";

export interface HeaderProps {
  validAccount: boolean;
}

function Header({ validAccount }: HeaderProps) {
  return (
    <Navbar bg="light" variant="light" expand="lg" className="border-bottom">
      <Container>
        <Navbar.Brand href="/" className="brand-container">
          <Image
            src="/logo.svg"
            alt={language.dictionary.logotype}
            style={{ width: "50px", height: "auto" }}
          />
          <span className="brand-name">{language.dictionary.websiteName}</span>
        </Navbar.Brand>
        <AccountDropdown validAccount={validAccount} />
      </Container>
    </Navbar>
  );
}

export default Header;
