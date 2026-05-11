/**
 * Tailwind CSS v3 preset — maps KoloHub / Iskalo Tech tokens for shadcn-style apps.
 *
 * tailwind.config.js:
 *   const kolohub = require('./design-system/tailwind.preset.js');
 *   module.exports = {
 *     presets: [kolohub],
 *     content: ['./src/**/*.{ts,tsx}'],
 *   };
 */

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ["class"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-sans)", "ui-sans-serif", "system-ui", "sans-serif"],
        mono: ["var(--font-mono)", "ui-monospace", "monospace"],
      },
      borderRadius: {
        lg: "var(--radius-lg)",
        md: "var(--radius-md)",
        sm: "var(--radius-sm)",
        xl: "var(--radius-xl)",
      },
      colors: {
        background: "hsl(var(--background) / <alpha-value>)",
        foreground: "hsl(var(--foreground) / <alpha-value>)",
        card: {
          DEFAULT: "hsl(var(--card) / <alpha-value>)",
          foreground: "hsl(var(--card-foreground) / <alpha-value>)",
        },
        popover: {
          DEFAULT: "hsl(var(--popover) / <alpha-value>)",
          foreground: "hsl(var(--popover-foreground) / <alpha-value>)",
        },
        primary: {
          DEFAULT: "hsl(var(--primary) / <alpha-value>)",
          foreground: "hsl(var(--primary-foreground) / <alpha-value>)",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
          foreground: "hsl(var(--secondary-foreground) / <alpha-value>)",
        },
        muted: {
          DEFAULT: "hsl(var(--muted) / <alpha-value>)",
          foreground: "hsl(var(--muted-foreground) / <alpha-value>)",
        },
        accent: {
          DEFAULT: "hsl(var(--accent) / <alpha-value>)",
          foreground: "hsl(var(--accent-foreground) / <alpha-value>)",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
          foreground: "hsl(var(--destructive-foreground) / <alpha-value>)",
        },
        border: "hsl(var(--border) / <alpha-value>)",
        input: "hsl(var(--input) / <alpha-value>)",
        ring: "hsl(var(--ring) / <alpha-value>)",
        kh: {
          purple: {
            50: "var(--color-purple-50)",
            100: "var(--color-purple-100)",
            200: "var(--color-purple-200)",
            300: "var(--color-purple-300)",
            400: "var(--color-purple-400)",
            500: "var(--color-purple-500)",
            600: "var(--color-purple-600)",
            700: "var(--color-purple-700)",
            800: "var(--color-purple-800)",
            900: "var(--color-purple-900)",
            950: "var(--color-purple-950)",
          },
          brand: {
            DEFAULT: "var(--color-brand)",
            hover: "var(--color-brand-hover)",
            muted: "var(--color-brand-muted)",
            fg: "var(--color-brand-foreground)",
          },
          commerce: {
            DEFAULT: "var(--color-accent)",
            hover: "var(--color-accent-hover)",
            muted: "var(--color-accent-muted)",
          },
          surface: {
            DEFAULT: "var(--color-surface)",
            elevated: "var(--color-surface-elevated)",
            glass: "var(--color-surface-glass)",
          },
          text: {
            primary: "var(--color-text-primary)",
            secondary: "var(--color-text-secondary)",
            muted: "var(--color-text-muted)",
          },
        },
      },
      boxShadow: {
        nav: "var(--shadow-nav)",
        card: "var(--shadow-card)",
      },
    },
  },
};
