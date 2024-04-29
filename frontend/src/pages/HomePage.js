import { Link } from 'react-router-dom';
import './App.css';

function HomePage() {
  return (
    <div className="App">
      <div className="background">
        <Link to="/rumble" className="enter-link">
          <button className="enter-button">Enter the Battle Royale</button>
        </Link>
      </div>
    </div>
  );
}

export default HomePage;
