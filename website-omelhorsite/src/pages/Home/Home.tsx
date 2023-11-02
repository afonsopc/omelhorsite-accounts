import "./home.scss"
import { Container } from "react-bootstrap";

const Home = () => {

  return (
    <div className="home-container">
      <Container className="px-4 pt-5 my-5">
        <img src="/palacio.jpg" className="image shadow-lg rounded" />
      </Container>
    </div>
  )
}

export default Home