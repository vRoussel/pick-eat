/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')
const svgToDataUri = require('mini-svg-data-uri')

module.exports = {
  content: ['./public/**/*.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    screens: {
      'xs': '400px',
      ...defaultTheme.screens,
    },
  },
  daisyui: {
      themes: [
      {
        pickeat_light: {
            "primary": "#BF9C53",
            "primary-content": "#FFFFFF",
            "secondary": "#5D7772",
            "accent": "#E6E6E6",
            "neutral": "#BF9C53",
            "neutral-content": "#FFFFFF",
            "base-100": "#FFFFFF",
            "info": "#3ABFF8",
            "success": "#36D399",
            "warning": "#FBBD23",
            "error": "#F87272",
        },
        pickeat_dark: {
            "primary": "#BF9C53",
            "primary-content": "#FFFFFF",
            "secondary": "#5D7772",
            "accent": "#B6B6B6",
            "neutral": "#BF9C53",
            "neutral-content": "#FFFFFF",
            "base-100": "#000000",
            "info": "#3ABFF8",
            "success": "#36D399",
            "warning": "#FBBD23",
            "error": "#F87272",
        },
      }
      ]
  },
  plugins: [
    require("daisyui"),
    require('@tailwindcss/typography')
  ]
}
