// app/layout.tsx
import { Inter as FontSans } from 'next/font/google';
import { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'GlobeCoin App',
  description: 'A cryptocurrency management application',
};

// Use Inter as the font - you can replace this with your preferred font
const fontSans = FontSans({
  subsets: ['latin'],
  variable: '--font-sans',
});

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${fontSans.variable} font-sans antialiased`}
        suppressHydrationWarning
      >
        {children}
      </body>
    </html>
  );
}