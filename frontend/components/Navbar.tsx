import Link from "next/link";
import { useRouter } from "next/navigation";

export interface NavItem {
  label: string;
  href: string;
}

export const navItems: NavItem[] = [
  { label: "Home", href: "/" },
  { label: "Dashboard", href: "/dashboard" },
  { label: "Projects", href: "/projects" },
  { label: "Bounties", href: "/bounties" },
  { label: "Settings", href: "/settings" },
];

// Base link color: slate-700 on white (approx. 10.3.1 contrast).
// Hover color: slate-900 on white (approx. 17.6:1), well above WCAG AA 4.5:1.
// Dark mode: slate-200 base, slate-50 hover, both high contrast on slate-900.
export default function Navbar() {
  const pathname = useRouter();

  return (
    <nav
      aria-label="Main navigation"
      className="flex items-center justify-between px-4 py-3 border-b border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900"
    >
      <Link
        href="/"
        className="text-lg font-semibold text-slate-900 dark:text-white"
      >
        App
      </Link>
      <ul className="flex items-center gap-6">
        {navItems.map((item) => {
          const isActive = pathname === item.href;
          return (
            <li key={item.href}>
              <Link
                href={item.href}
                aria-current={isActive ? "page" : undefined}
                className={[
                  "text-sm font-medium transition-colors focus-outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2",
                  isActive
                    ? "text-blue-700 dark:text-blue-300"
                    : "text-slate-700 hover:text-slate-900 dark:text-slate-200 dark:hover:text-slate-50",
                ].join(" ")}
              >
                {item.label}
              </Link>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
