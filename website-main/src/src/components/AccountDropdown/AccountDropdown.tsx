import "./accountDropdown.scss";
import { language } from "../../main";
import { Image, Dropdown, Button } from "react-bootstrap";
import { Link } from "react-router-dom";
import AuthenticationModal from "../AuthenticationModal/AuthenticationModal";
import SettingsModal from "../SettingsModal/SettingsModal";
import { useState } from "react";
import { logout } from "../../services/accountService";

export interface AccountDropdownProps {
  validAccount: boolean;
}

const AccountDropdown = ({ validAccount }: AccountDropdownProps) => {
  const [authenticationModalVisibility, setAuthenticationModalVisibility] = useState(false);
  const [settingsModalVisibility, setSettingsModalVisibility] = useState(false);

  const handleLogout = () => {
    logout();
    window.location.reload()
  }

  return (
    <div>
      {validAccount ? (
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
            <Dropdown.Item onClick={() => setSettingsModalVisibility(true)}>{language.dictionary.settings}</Dropdown.Item>
            <Dropdown.Item onClick={() => handleLogout()}>{language.dictionary.logout}</Dropdown.Item>
          </Dropdown.Menu>
        </Dropdown>
      ) : (
        <Button onClick={() => setAuthenticationModalVisibility(true)}>{language.dictionary.signIn}</Button>
      )}
      <AuthenticationModal show={authenticationModalVisibility} onHide={() => setAuthenticationModalVisibility(!authenticationModalVisibility)} />
      <SettingsModal show={settingsModalVisibility} onHide={() => setSettingsModalVisibility(!settingsModalVisibility)} />
    </div>
  )
}

export default AccountDropdown