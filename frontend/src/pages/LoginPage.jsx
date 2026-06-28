import { useState } from 'react'
import { Link } from 'react-router-dom'
import InputField from '../components/InputField'
import Button from '../components/Button'
import loginImage from '../assets/login-bg.jpg'

export default function LoginPage() {
  // 1. Manage form state in a single object
  const [formData, setFormData] = useState({
    email: '',
    password: '',
  })
  
  // Track client-side validation errors
  const [errors, setErrors] = useState({})
  
  // Simulate network request during submission
  const [isLoading, setIsLoading] = useState(false)

  // 2. Reusable dynamic change handler
  const handleChange = (e) => {
    const { name, value } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }))
    
    // Clear the error message for this input when the user starts typing again
    if (errors[name]) {
      setErrors((prev) => ({
        ...prev,
        [name]: '',
      }))
    }
  }

  // 3. Client-side form validation
  const validateForm = () => {
    const tempErrors = {}
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

    if (!formData.email) {
      tempErrors.email = 'Email is required'
    } else if (!emailRegex.test(formData.email)) {
      tempErrors.email = 'Please enter a valid email address'
    }

   if (!formData.password) {
  tempErrors.password = 'Password is required'
} else if (formData.password.length < 8) {
  tempErrors.password = 'Password must be at least 8 characters'
} 
     setErrors(tempErrors)
    return Object.keys(tempErrors).length === 0
  }

  // 4. Handle Form Submission
  const handleSubmit = (e) => {
    e.preventDefault()
    if (validateForm()) {
      setIsLoading(true)
      
      // Simulate API request delay
      setTimeout(() => {
        setIsLoading(false)
        alert('Form submitted successfully! (Backend connection comes later)')
      }, 1500)
    }
  }
  return (
    <div className="min-h-screen grid grid-cols-1 lg:grid-cols-2 bg-[#F4EBA4] text-zinc-900 font-sans">
      
      {/* LEFT COLUMN: Form container (Takes exactly 50% width on large screens) */}
      <div className="flex flex-col justify-center px-8 py-12 sm:px-16 lg:px-20 xl:px-24 bg-transparent z-10">
        <div className="mx-auto w-full max-w-sm">
          
          {/* Brand Logo Header */}
          <div className="flex items-center gap-2 mb-8">
            <div className="h-7 w-7 bg-zinc-950 rounded flex items-center justify-center text-white font-extrabold text-sm tracking-tighter">
              F
            </div>
            <span className="font-semibold text-lg tracking-tight text-zinc-900">FaSa</span>
          </div>

          {/* Title Header */}
          <div className="space-y-2 mb-8">
            <h1 className="text-3xl font-semibold text-zinc-900 tracking-tight">
              Welcome back
            </h1>
            <p className="text-sm text-zinc-500">
              Sign in to your account to continue
            </p>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-5">
            <div className="space-y-4">
              <InputField
                label="Email address"
                type="email"
                name="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="name@company.com"
                error={errors.email}
                required
              />

              <InputField
                label="Password"
                type="password"
                name="password"
                value={formData.password}
                onChange={handleChange}
                placeholder="••••••••"
                error={errors.password}
                required
              />
            </div>

            {/* Remember Me & Forgot Password link */}
            <div className="flex items-center justify-between text-sm">
              <label className="flex items-center gap-2 text-zinc-600 select-none cursor-pointer">
                <input
                  id="remember-me"
                  name="remember-me"
                  type="checkbox"
                  className="h-4 w-4 rounded border-zinc-300 text-zinc-950 focus:ring-zinc-950 cursor-pointer accent-zinc-950"
                />
                Remember me
              </label>
              <a 
                href="#" 
                className="font-medium text-zinc-900 hover:text-zinc-600 transition-colors duration-150"
              >
                Forgot password?
              </a>
            </div>

            {/* Submit Button (Black theme style override) */}
            <Button 
              type="submit" 
              isLoading={isLoading} 
              className="w-full bg-zinc-950 hover:bg-zinc-900 focus:ring-zinc-200"
            >
              Sign In
            </Button>
          </form>

          {/* Footer Link */}
          <p className="text-center text-sm text-zinc-500 mt-8">
            Don't have an account?{' '}
            <Link 
              to="/signup" 
              className="font-semibold text-zinc-900 hover:text-zinc-700 underline underline-offset-4 decoration-zinc-200 hover:decoration-zinc-900 transition-all duration-150"
            >
              Sign up
            </Link>
          </p>

        </div>
      </div>

      {/* RIGHT COLUMN: Image Showcase (Takes exactly 50% width, hidden on mobile) */}
      <div className="hidden lg:flex bg-zinc-50 border-l border-zinc-100 items-center justify-center p-12 xl:p-16 select-none">
        
        {/* Framed Image Card */}
        <div className="w-full h-full max-h-[80vh] rounded-2xl overflow-hidden shadow-xl border border-zinc-200 bg-white">
          <img 
            src={loginImage} 
            alt="Login illustration" 
            className="w-full h-full object-cover hover:scale-[1.02] transition-transform duration-700 ease-out"
          />
        </div>

      </div>

    </div>
  )


}
