import "./accountDropdown.scss";
import { language } from "../../main";
import { Image, Dropdown, Button } from "react-bootstrap";
import AuthenticationModal from "../AuthenticationModal/AuthenticationModal";
import SettingsModal from "../SettingsModal/SettingsModal";
import { useEffect, useState } from "react";
import { getAccountInfo, logout } from "../../services/accountService";

const AccountDropdown = () => {
  const [authenticationModalVisibility, setAuthenticationModalVisibility] = useState(false);
  const [settingsModalVisibility, setSettingsModalVisibility] = useState(false);
  const [username, setUsername] = useState("");


  const handleLogout = () => {
    logout();
    window.location.reload()
  }

  useEffect(() => {
    const fetchData = async () => {
      const data = await getAccountInfo();
      if (data.data && data.data.username) {
        setUsername(data.data.username);
      }
    }

    fetchData();
  }, [])

  return (
    <div>
      {username ? (
        <Dropdown>
          <Dropdown.Toggle variant="outline-dark" id="dropdown-basic" className="account-dropdown">
            <Image
              src="https://pagman.org/pagman.jpg"
              roundedCircle
              style={{ width: "30px", height: "30px", marginRight: "10px" }}
            />
            <span className="account-dropdown-text">
              {username}
            </span>
          </Dropdown.Toggle>
          <Dropdown.Menu>
            <Dropdown.Item onClick={() => setSettingsModalVisibility(true)}>{language.dictionary.settings}</Dropdown.Item>
            <Dropdown.Item onClick={() => handleLogout()}>{language.dictionary.logout}</Dropdown.Item>
          </Dropdown.Menu>
        </Dropdown>
      ) : (
        <Button onClick={() => setAuthenticationModalVisibility(true)}>{language.dictionary.signIn}</Button>
      )}
      <AuthenticationModal startingTab="sign-in" show={authenticationModalVisibility} onHide={() => setAuthenticationModalVisibility(!authenticationModalVisibility)} />
      <SettingsModal show={settingsModalVisibility} onHide={() => setSettingsModalVisibility(!settingsModalVisibility)} />
    </div>
  )
}

export default AccountDropdown