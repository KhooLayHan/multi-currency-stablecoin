"use client"

import { useState } from "react"
import Link from "next/link"
import { usePathname } from "next/navigation"
import {
  LayoutDashboard,
  Coins,
  Flame,
  ArrowLeftRight,
  BracketsIcon as Bridge,
  FileText,
  Settings,
  Menu,
  X,
} from "lucide-react"
import { cn } from "@/lib/utils"

// Navigation items
const navItems = [
  {
    title: "Dashboard",
    href: "/dashboard",
    icon: LayoutDashboard,
  },
  {
    title: "Mint",
    href: "/Mint",
    icon: Coins,
  },
  {
    title: "Burn",
    href: "/Burn",
    icon: Flame,
  },
  {
    title: "Swap",
    href: "/swap",
    icon: ArrowLeftRight,
  },
  {
    title: "Bridge",
    href: "/bridge",
    icon: Bridge,
  },
  {
    title: "Transactions",
    href: "/transactions",
    icon: FileText,
  },
  {
    title: "Settings",
    href: "/settings",
    icon: Settings,
  },
]

export function Sidebar() {
  const pathname = usePathname()
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)

  return (
    <>
      {/* Mobile menu button */}
      <div className="fixed top-4 left-4 z-40 md:hidden">
        <button
          onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
          className="p-2 rounded-md bg-white text-[#573900] shadow-md"
        >
          {isMobileMenuOpen ? <X size={24} /> : <Menu size={24} />}
        </button>
      </div>

      {/* Sidebar for mobile (slide-in) */}
      <div
        className={cn(
          "fixed inset-y-0 left-0 z-30 w-64 bg-white transform transition-transform duration-300 ease-in-out md:hidden",
          isMobileMenuOpen ? "translate-x-0" : "-translate-x-full",
        )}
      >
        <div className="h-full flex flex-col">
          <div className="p-6 border-b">
            <h1 className="text-2xl font-bold text-[#573900]">GlobeCoin</h1>
          </div>
          <nav className="flex-1 p-4 overflow-y-auto">
            <ul className="space-y-2">
              {navItems.map((item) => {
                const isActive = pathname === item.href
                return (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      className={cn(
                        "flex items-center gap-3 px-3 py-2 rounded-md text-[#573900] hover:bg-[#F1B62A]/20 transition-colors",
                        isActive && "bg-[#F1B62A]/30 font-medium",
                      )}
                      onClick={() => setIsMobileMenuOpen(false)}
                    >
                      <item.icon size={20} />
                      <span>{item.title}</span>
                    </Link>
                  </li>
                )
              })}
            </ul>
          </nav>
        </div>
      </div>

      {/* Sidebar for desktop (permanent) */}
      <div className="hidden md:block w-64 bg-white border-r border-gray-200 h-screen fixed">
        <div className="h-full flex flex-col">
          <div className="p-6 border-b">
            <h1 className="text-2xl font-bold text-[#573900]">GlobeCoin</h1>
          </div>
          <nav className="flex-1 p-4 overflow-y-auto">
            <ul className="space-y-2">
              {navItems.map((item) => {
                const isActive = pathname === item.href
                return (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      className={cn(
                        "flex items-center gap-3 px-3 py-2 rounded-md text-[#573900] hover:bg-[#F1B62A]/20 transition-colors",
                        isActive && "bg-[#F1B62A]/30 font-medium",
                      )}
                    >
                      <item.icon size={20} />
                      <span>{item.title}</span>
                    </Link>
                  </li>
                )
              })}
            </ul>
          </nav>
        </div>
      </div>
    </>
  )
}
