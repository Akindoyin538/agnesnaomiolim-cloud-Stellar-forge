import React from 'react';

export interface NavbarLink {
  label: string;
  href: string;
  active?: boolean;
}

export interface NavbarProps {
  /** Optional custom brand title, defaults to 'Stellar-K8s' */
  title?: string;
  /** Optional logo source override */
  logoSrc?: string;
  /** Optional navigation links */
  links?: NavbarLink[];
  /** Optional current environment or network indicator (e.g. 'Testnet', 'Mainnet') */
  networkBadge?: string;
  /** Optional CSS class name */
  className?: string;
  /** Children or additional toolbar items */
  children?: React.ReactNode;
}

/**
 * Default logo asset path referencing the repository brand logo asset relative to frontend/components/Navbar.tsx.
 * Resolves to assets/logo.png from frontend/components/.
 */
export const DEFAULT_LOGO_PATH = '../../assets/logo.png';

/**
 * Navbar - Navigation Header Component
 *
 * Provides the application header with brand logo assets, navigation links,
 * and system/environment indicators. Handles relative logo paths and provides
 * SVG vector fallbacks when raster logo illustrations are unavailable.
 */
export const Navbar: React.FC<NavbarProps> = ({
  title = 'Stellar-K8s',
  logoSrc = DEFAULT_LOGO_PATH,
  links = [],
  networkBadge,
  className = '',
  children,
}) => {
  const [imgError, setImgError] = React.useState<boolean>(false);

  return (
    <header className={`navbar-header ${className}`} role="banner" style={headerStyles}>
      <div className="navbar-brand-container" style={brandContainerStyles}>
        <a href="/" className="navbar-brand-link" style={brandLinkStyles} aria-label={`${title} Home`}>
          {!imgError ? (
            <img
              src={logoSrc}
              alt={`${title} Logo`}
              className="navbar-brand-logo"
              style={logoImageStyles}
              onError={() => setImgError(true)}
            />
          ) : (
            <svg
              className="navbar-brand-svg-fallback"
              viewBox="0 0 32 32"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              style={logoImageStyles}
              aria-label={`${title} Vector Logo`}
            >
              <defs>
                <linearGradient id="stellarBrandGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stopColor="#4f8ef7" />
                  <stop offset="100%" stopColor="#a855f7" />
                </linearGradient>
              </defs>
              <circle cx="16" cy="16" r="14" fill="url(#stellarBrandGradient)" />
              <path
                d="M16 6L18.5 13.5L26 16L18.5 18.5L16 26L13.5 18.5L6 16L13.5 13.5L16 6Z"
                fill="#ffffff"
              />
            </svg>
          )}
          <span className="navbar-brand-title" style={titleStyles}>
            {title}
          </span>
        </a>

        {networkBadge && (
          <span className="navbar-network-badge" style={networkBadgeStyles}>
            {networkBadge}
          </span>
        )}
      </div>

      {links.length > 0 && (
        <nav className="navbar-nav" role="navigation" aria-label="Main Navigation" style={navStyles}>
          {links.map((link) => (
            <a
              key={link.href}
              href={link.href}
              className={`navbar-nav-link ${link.active ? 'active' : ''}`}
              style={{
                ...navLinkStyles,
                color: link.active ? '#4f8ef7' : '#94a3b8',
              }}
              aria-current={link.active ? 'page' : undefined}
            >
              {link.label}
            </a>
          ))}
        </nav>
      )}

      {children && <div className="navbar-actions" style={actionsStyles}>{children}</div>}
    </header>
  );
};

const headerStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  padding: '0.75rem 1.5rem',
  background: '#0e1422',
  borderBottom: '1px solid #1e293b',
  color: '#f8fafc',
  gap: '1rem',
};

const brandContainerStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: '0.75rem',
};

const brandLinkStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: '0.625rem',
  textDecoration: 'none',
  color: 'inherit',
};

const logoImageStyles: React.CSSProperties = {
  width: '32px',
  height: '32px',
  objectFit: 'contain',
  borderRadius: '4px',
};

const titleStyles: React.CSSProperties = {
  fontSize: '1.25rem',
  fontWeight: 700,
  letterSpacing: '-0.01em',
  background: 'linear-gradient(90deg, #4f8ef7, #a855f7)',
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
};

const networkBadgeStyles: React.CSSProperties = {
  fontSize: '0.75rem',
  fontWeight: 600,
  padding: '0.2rem 0.5rem',
  borderRadius: '9999px',
  background: '#1e293b',
  color: '#06b6d4',
  border: '1px solid #334155',
  textTransform: 'uppercase',
  letterSpacing: '0.05em',
};

const navStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: '1.25rem',
};

const navLinkStyles: React.CSSProperties = {
  fontSize: '0.9rem',
  fontWeight: 500,
  textDecoration: 'none',
  transition: 'color 150ms ease',
};

const actionsStyles: React.CSSProperties = {
  display: 'flex',
  alignItems: 'center',
  gap: '0.75rem',
  marginLeft: 'auto',
};

export default Navbar;
