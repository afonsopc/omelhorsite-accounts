import Footer from "../../components/Footer/Footer";
import Header from "../../components/Header/Header";
import { Outlet } from 'react-router-dom';
import "./headerAndFooter.scss"

const HeaderAndFooter = () => {
  return (
    <div className="layout-container">
      <Header />
      <div className="content">
        <Outlet />
      </div>
      <Footer />
    </div>
  )
}

export default HeaderAndFooter