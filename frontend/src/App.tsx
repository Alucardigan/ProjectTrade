import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import LoginPage from './Pages/LoginPage';
import LandingPage from './Pages/LandingPage';
import PortfolioPage from './Pages/PortfolioPage';
import BuyStockPage from './Pages/BuyStockPage';
import OrderHistoryPage from './Pages/OrderHistoryPage';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<LoginPage />} />
        <Route path="/landing_page" element={<LandingPage />} />
        <Route path="/portfolio" element={<PortfolioPage />} />
        <Route path="/buy" element={<BuyStockPage />} />
        <Route path="/orders" element={<OrderHistoryPage />} />
      </Routes>
    </Router>
  );
}

export default App;
