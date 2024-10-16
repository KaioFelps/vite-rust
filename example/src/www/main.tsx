import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from '@/www/pages/App.tsx'
import '@/www/index.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
