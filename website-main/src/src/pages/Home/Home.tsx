import "./home.scss"
import { Container, Image } from "react-bootstrap";

const Home = () => {
  return (
    <div>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/es.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt7.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/de.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt6.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/en.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/br.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/joaov.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt4.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/fr.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt3.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
      <Container className="px-4 pt-5 my-5 text-center">
        <Image src="/pt5.svg" className="shadow-lg w-75 bg-white rounded" />
      </Container>
    </div>
  )
}

export default Home