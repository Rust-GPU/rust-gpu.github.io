import React from 'react';
import OriginalBlogLayout from '@theme-original/BlogLayout';
import type { WrapperProps } from '@docusaurus/types';

type BlogLayoutProps = React.ComponentProps<typeof OriginalBlogLayout>;

// This is used to add an id so we can target with CSS.
// Without this we can't tell the difference between blog posts and other pages from CSS.
export default function BlogLayout(props: WrapperProps<BlogLayoutProps>) {
  return (
    <div id="blog">
      <OriginalBlogLayout {...props} />
    </div>
  );
}
