import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/blog',
    component: ComponentCreator('/blog', '9f3'),
    exact: true
  },
  {
    path: '/blog/archive',
    component: ComponentCreator('/blog/archive', '182'),
    exact: true
  },
  {
    path: '/blog/authors',
    component: ComponentCreator('/blog/authors', '0b7'),
    exact: true
  },
  {
    path: '/blog/tags',
    component: ComponentCreator('/blog/tags', '287'),
    exact: true
  },
  {
    path: '/blog/tags/announcements',
    component: ComponentCreator('/blog/tags/announcements', '448'),
    exact: true
  },
  {
    path: '/blog/transition-announcement',
    component: ComponentCreator('/blog/transition-announcement', '271'),
    exact: true
  },
  {
    path: '/changelog',
    component: ComponentCreator('/changelog', 'fda'),
    exact: true
  },
  {
    path: '/ecosystem',
    component: ComponentCreator('/ecosystem', '18c'),
    exact: true
  },
  {
    path: '/markdown-page',
    component: ComponentCreator('/markdown-page', '3d7'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', 'c4c'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '7f1'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', 'd43'),
            routes: [
              {
                path: '/docs/empty',
                component: ComponentCreator('/docs/empty', 'd08'),
                exact: true,
                sidebar: "defaultSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/', 'e5f'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
