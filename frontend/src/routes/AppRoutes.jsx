import { Routes, Route, Navigate } from 'react-router-dom'
import LoginPage from '../pages/LoginPage'
import SignUpPage from '../pages/SignUpPage'

export default function AppRoutes() {
  return (
    <Routes>
      {/* Route mapping */}
      <Route path="/login" element={<LoginPage />} />
      <Route path="/signup" element={<SignUpPage />} />

      {/* Redirect any other path (like /) to the Login page */}
      <Route path="*" element={<Navigate to="/login" replace />} />
    </Routes>
  )
}
