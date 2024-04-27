import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import HomePage from './pages/HomePage';
import RumblePage from './pages/RumblePage';

ReactDOM.render(
  <React.StrictMode>
    <Router>
      <Routes>
        <Route path="/rumble" element={<RumblePage/>} />
        <Route path="/" element={<HomePage/>} />
      </Routes>
    </Router>
  </React.StrictMode>,
  document.getElementById('root')
);
