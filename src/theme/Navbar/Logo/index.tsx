import React from 'react';
import { useLocation } from '@docusaurus/router';
import Logo from '@theme/Logo';

export default function NavbarLogo(): JSX.Element {
  const location = useLocation();
  if (location.pathname === '/') {
    return null;
  }


  return (
    <Logo
      className="navbar__brand"
      imageClassName="navbar__logo"
      titleClassName="navbar__title text--truncate"
    />
  );
}
