import Logo from '../../components/Logo/Logo'
import { language } from '../../main'
import './home.scss'

const Home = () => {
    return (
        <div className="home-container">
            <Logo className="logo" userPictureUrl="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2F4.bp.blogspot.com%2F-NXA-XoDClDw%2FWb75VATyR2I%2FAAAAAAAAUF4%2FaOqYhCqNeZEeIqzRu0WXomHEeomHubbRwCLcBGAs%2Fs1600%2FD._carlos_3.jpg&f=1&nofb=1&ipt=b6958c21bfcaf498a3e07a64ac3b5cee30cc3d169ccdeb9824941928d7494af1&ipo=images" />
            <h1>{language.dictionary.welcomeToTheWebsite}</h1>
        </div>
    )
}

export default Home