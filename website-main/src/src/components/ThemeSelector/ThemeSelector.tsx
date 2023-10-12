import "./themeSelector.scss"
import { Dropdown } from "react-bootstrap";
import { Theme, language, setTheme, themes } from "../../themes";

const ThemeSelector = () => {
  const handleChangeTheme = (theme: Theme) => {
    setTheme(theme);
    window.location.reload();
  }
  return (
    <Dropdown>
      <Dropdown.Toggle variant="default" id="dropdownMenu1" className="theme border">
        {language.dictionary.changeTheme}
      </Dropdown.Toggle>
      <Dropdown.Menu>
        {Object.values(themes).map((themeEntry: Theme) => (
          <Dropdown.Item
            key={themeEntry.code}
            className="theme"
            onClick={() => handleChangeTheme(themeEntry)}
          >
            {themeEntry.name}
          </Dropdown.Item>
        ))}
      </Dropdown.Menu>
    </Dropdown>
  );
};

export default ThemeSelector;
