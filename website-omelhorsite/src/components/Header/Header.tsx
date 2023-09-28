import { Navbar, Nav, Container, Image, Dropdown, Button } from "react-bootstrap";
import "./header.scss";
import { useEffect, useState } from "react";
import AuthenticationModal from "../AuthenticationModal/AuthenticationModal";
import { language, logout, verifyAccountToken } from "../../main";
import SettingsModal from "../SettingsModal/SettingsModal";

function Header() {
  const [showAuthenticationModal, setShowAuthenticationModal] = useState(false);
  const [showSettingsModal, setShowSettingsModal] = useState(false);
  const [isAccountTokenValid, setIsAccountTokenValid] = useState(false);

  useEffect(() => {
    const verify = async () => {
       const response = await verifyAccountToken();
       if (response.status === 200) {
         setIsAccountTokenValid(true);
       }
       else {
        setIsAccountTokenValid(false);
       }
    }
  
    verify();
  }, [])

  const handleShowAuthenticationModal = () => {
    setShowAuthenticationModal(true);
  };

  const handleCloseAuthenticationModal = () => {
    setShowAuthenticationModal(false);
  };

  const handleShowSettingsModal = () => {
    setShowSettingsModal(true);
  };

  const handleCloseSettingsModal = () => {
    setShowSettingsModal(false);
  };

  return (
    <Navbar bg="light" variant="light" expand="lg" className="border-bottom">
      <Container>
        <Navbar.Brand href="/" className="brand-container">
          <img
            src="/logo.svg"
            alt={language.dictionary.logotype}
            style={{ width: "50px", height: "auto" }}
          />
          <span className="brand-name">{language.dictionary.websiteName}</span>
        </Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        {isAccountTokenValid ? (
          <Dropdown>
            <Dropdown.Toggle variant="outline-dark" id="dropdown-basic">
              <Image
                src="https://pagman.org/pagman.jpg"
                roundedCircle
                style={{ width: "30px", height: "30px", marginRight: "10px" }}
              />
              {language.dictionary.account}
            </Dropdown.Toggle>
            <Dropdown.Menu>
              <Dropdown.Item href="/drive">{language.dictionary.myDrive}</Dropdown.Item>
              <Dropdown.Item onClick={handleShowSettingsModal}>{language.dictionary.settings}</Dropdown.Item>
              <Dropdown.Item onClick={logout}>{language.dictionary.logout}</Dropdown.Item>
            </Dropdown.Menu>
          </Dropdown>
        ) : (
          <Nav.Link onClick={handleShowAuthenticationModal}><Button>{language.dictionary.signIn}</Button></Nav.Link>
        )}
      </Container>
      <AuthenticationModal show={showAuthenticationModal} onHide={handleCloseAuthenticationModal} startingTab="sign-in"/>
      <SettingsModal show={showSettingsModal} onHide={handleCloseSettingsModal} />
    </Navbar>
  );
}

export default Header;
