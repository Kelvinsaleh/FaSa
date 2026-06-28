import { useState } from 'react'
import { Link } from 'react-router-dom'
import InputField from '../components/InputField'
import Button from '../components/Button'
import loginImage from '../assets/login-bg.jpg' // reuse the same illustration

export default function SignUpPage() {
  // Manage form state
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    password: '',
    confirmPassword: '',
  })

  const [errors, setErrors] = useState({})
  const [isLoading, setIsLoading] = useState(false)

  const handleChange = (e) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))
    if (errors[name]) {
      setErrors((prev) => ({ ...prev, [name]: '' }))
    }
  }

  const validateForm = () => {
    const temp = {}
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!formData.name) temp.name = 'Name is required'
    if (!formData.email) temp.email = 'Email is required'
    else if (!emailRegex.test(formData.email)) temp.email = 'Enter a valid email address'
    if (!formData.password) temp.password = 'Password is required'
    else if (formData.password.length < 8) temp.password = 'Password must be at least 8 characters'
    if (formData.confirmPassword !== formData.password) temp.confirmPassword = 'Passwords do not match'
    setErrors(temp)
    return Object.keys(temp).length === 0
  }

  const handleSubmit = (e) => {
    e.preventDefault()
    if (validateForm()) {
      setIsLoading(true)
      setTimeout(() => {
        setIsLoading(false)
        alert('Sign‑up successful! (Backend integration later)')
      }, 1500)
    }
  }

  return (
    <div className="min-h-screen grid grid-cols-1 lg:grid-cols-2 bg-[#F4EBA4] text-zinc-900 font-sans">
      {/* LEFT COLUMN – Sign‑up form */}
      <div className="flex flex-col justify-center px-8 py-12 sm:px-16 lg:px-20 xl:px-24 bg-transparent z-10">
        <div className="mx-auto w-full max-w-sm">
          {/* Brand logo */}
          <div className="flex items-center gap-2 mb-8">
            <div className="h-7 w-7 bg-zinc-950 rounded flex items-center justify-center text-white font-extrabold text-sm tracking-tighter">
              F
            </div>
            <span className="font-semibold text-lg tracking-tight text-zinc-900">FaSa</span>
          </div>

          {/* Header */}
          <div className="space-y-2 mb-8">
            <h1 className="text-3xl font-semibold text-zinc-900 tracking-tight">Create your account</h1>
            <p className="text-sm text-zinc-500">Join us and start using FaSa today</p>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-5">
            <div className="space-y-4">
              <InputField
                label="Full name"
                type="text"
                name="name"
                value={formData.name}
                onChange={handleChange}
                placeholder="John Doe"
                error={errors.name}
                required
              />
              <InputField
                label="Email address"
                type="email"
                name="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="john@example.com"
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
              <InputField
                label="Confirm password"
                type="password"
                name="confirmPassword"
                value={formData.confirmPassword}
                onChange={handleChange}
                placeholder="••••••••"
                error={errors.confirmPassword}
                required
              />
            </div>

            {/* Terms checkbox */}
            <div className="flex items-center justify-between text-sm">
              <label className="flex items-center gap-2 text-zinc-600 select-none cursor-pointer">
                <input
                  type="checkbox"
                  className="h-4 w-4 rounded border-zinc-300 text-zinc-950 focus:ring-zinc-950 accent-zinc-950"
                />
                I agree to the <a href="#" className="font-medium text-zinc-900 hover:text-zinc-600 underline">Terms</a>
              </label>
            </div>

            {/* Submit button */}
            <Button type="submit" isLoading={isLoading} className="w-full bg-zinc-950 hover:bg-zinc-900 focus:ring-zinc-200">
              Sign Up
            </Button>
          </form>

          {/* Footer link back to login */}
          <p className="text-center text-sm text-zinc-500 mt-8">
            Already have an account?{' '}
            <Link to="/login" className="font-semibold text-zinc-900 hover:text-zinc-700 underline underline-offset-4 decoration-zinc-200 hover:deco
ration-zinc-900 transition-all duration-150">
              Sign in
            </Link>
          </p>
        </div>
      </div>

      {/* RIGHT COLUMN – same illustration as login page */}
      <div className="hidden lg:flex bg-[#F4EBA4] border-l border-zinc-100 items-center justify-center p-12 xl:p-16 select-none">
        <div className="w-full h-full max-h-[80vh] rounded-2xl overflow-hidden shadow-xl border border-zinc-200 bg-white">
          <img src={loginImage} alt="Sign‑up illustration" className="w-full h-full object-cover hover:scale-[1.02] transition-transform duration-700 ease-out" />
        </div>
      </div>
    </div>
  )
}
