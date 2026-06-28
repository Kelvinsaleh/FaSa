export default function InputField({
  label,
  type = 'text',
  name,
  value,
  onChange,
  placeholder,
  error,
  required = false,
  ...props
}) {
  return (
    <div className="flex flex-col gap-1.5 w-full">
      {/* 1. Label */}
      {label && (
        <label 
          htmlFor={name} 
          className="text-sm font-medium text-slate-700 dark:text-slate-300"
        >
          {label} {required && <span className="text-red-500">*</span>}
        </label>
      )}

      {/* 2. Input Box */}
      <input
        type={type}
        id={name}
        name={name}
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        required={required}
        className={`px-4 py-2.5 rounded-lg border text-slate-900 bg-white placeholder-slate-400 
          transition-all duration-200 ease-in-out text-sm outline-none
          ${
            error
              ? 'border-red-500 focus:border-red-500 focus:ring-4 focus:ring-red-100'
              : 'border-slate-200 hover:border-slate-300 focus:border-indigo-600 focus:ring-4 focus:ring-indigo-100'
          }`}
        {...props}
      />

      {/* 3. Validation Error Message */}
      {error && (
        <p className="text-xs text-red-600 font-medium mt-0.5" id={`${name}-error`}>
          {error}
        </p>
      )}
    </div>
  )
}
