import { language } from '../../../main'
import { Button } from 'react-bootstrap'
import { logout } from '../../../services/accountService'

const LogoutButton = () => {
  const handleLogout = () => {
    logout();
    window.location.reload()
  }

  return (
    <Button onClick={() => handleLogout()} variant="outline-danger">
      {language.dictionary.logout}
    </Button>
  )
}

export default LogoutButton