import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import HomePage from './Pages/Homepage.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <HomePage />
  </StrictMode>,
)
