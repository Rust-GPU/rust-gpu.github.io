import React from "react";
import clsx from "clsx";
import ErrorBoundary from "@docusaurus/ErrorBoundary";
import {
  PageMetadata,
  SkipToContentFallbackId,
  ThemeClassNames,
} from "@docusaurus/theme-common";
import { useKeyboardNavigation } from "@docusaurus/theme-common/internal";
import SkipToContent from "@theme/SkipToContent";
import AnnouncementBar from "@theme/AnnouncementBar";
import Navbar from "@theme/Navbar";
import Footer from "@theme/Footer";
import LayoutProvider from "@theme/Layout/Provider";
import ErrorPageContent from "@theme/ErrorPageContent";
import type { Props as LayoutProps } from "@theme/Layout";

interface Props extends LayoutProps {
  hero: React.ReactNode;
}

export default function HomeLayout(props: Props): JSX.Element {
  const {
    children,
    noFooter,
    wrapperClassName,
    // Not really layout-related, but kept for convenience/retro-compatibility
    title,
    description,
    hero,
  } = props;

  useKeyboardNavigation();
  return (
    <LayoutProvider>
      <PageMetadata title={title} description={description} />

      <SkipToContent />

      <AnnouncementBar />

      <div
        id={SkipToContentFallbackId}
        className={clsx(ThemeClassNames.wrapper.main, wrapperClassName)}
      >
        <ErrorBoundary fallback={(params) => <ErrorPageContent {...params} />}>
          <div className="bg-white justify-center flex flex-col w-full h-screen max-h-[1600px]">
            <div className="w-full">
              <Navbar />
            </div>
            <div className="flex flex-grow justify-center">{hero}</div>
          </div>
          <div className="bg-secondary w-full flex justify-center">
            {children}
          </div>
        </ErrorBoundary>
      </div>
    </LayoutProvider>
  );
}
