import Footer from "../../components/Footer/Footer";
import Header from "../../components/Header/Header";
import { Outlet } from 'react-router-dom';
import "./headerAndFooter.scss"

export interface HeaderAndFooterProps {
  validAccount: boolean;
}

const HeaderAndFooter = ({ validAccount }: HeaderAndFooterProps) => {
  return (
    <div className="layout-container">
        <Header validAccount={validAccount}/>
        <div className="content">
          <Outlet/>
        </div>
        <Footer/>
    </div>
  )
}

export default HeaderAndFooter