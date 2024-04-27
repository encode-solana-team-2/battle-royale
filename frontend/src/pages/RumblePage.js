import React, { useState, useEffect } from 'react';
import './App.css';
import bonkImageLarge from '../images/bonk-large.jpg'
import wifImageLarge from '../images/wif-large.jpeg'

function RumblePage() {
  return (
    <div className="App">
      <div className="background">
        <div className="countdown-wrapper">
          <Countdown targetDate="2024-05-01T00:00:00" />
        </div>
        <div className="rectangle" style={{ left: '25%' }}>
          <img src={bonkImageLarge} alt="Warrior 1" className="warrior-image" />
          <div>BONK</div>
          <button className="support-button">Support BONK</button>
        </div>
        <div className="rectangle" style={{ right: '25%' }}>
          <img src={wifImageLarge} alt="Warrior 2" className="warrior-image" />
          <div>WIF</div>
          <button className="support-button">Support WIF</button>
        </div>
      </div>
      <div className="create-wrapper">
        <div className="create">
          <button className="create-button">Create your own Battle Royale</button>
        </div>
      </div>
    </div>
  );
}

function Countdown({ targetDate }) {
  const calculateTimeLeft = () => {
    const difference = +new Date(targetDate) - +new Date();
    let timeLeft = {};

    if (difference > 0) {
      timeLeft = {
        days: Math.floor(difference / (1000 * 60 * 60 * 24)),
        hours: Math.floor((difference / (1000 * 60 * 60)) % 24),
        minutes: Math.floor((difference / 1000 / 60) % 60),
        seconds: Math.floor((difference / 1000) % 60)
      };
    }

    return timeLeft;
  };

  const [timeLeft, setTimeLeft] = useState(calculateTimeLeft());

  useEffect(() => {
    const timer = setTimeout(() => {
      setTimeLeft(calculateTimeLeft());
    }, 1000);

    return () => clearTimeout(timer);
  });

  const timerComponents = [];

  Object.keys(timeLeft).forEach((interval) => {
    if (!timeLeft[interval]) {
      return;
    }

    timerComponents.push(
      <span>
        {timeLeft[interval]} {interval}{" "}
      </span>
    );
  });

  return (
    <div className="countdown">
      {timerComponents.length ? timerComponents : <span>Time's up!</span>}
    </div>
  );
}

export default RumblePage;

