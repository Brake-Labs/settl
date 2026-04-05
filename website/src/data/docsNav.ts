export interface NavItem {
  title: string;
  href: string;
}

export interface NavSection {
  title: string;
  items: NavItem[];
}

export const docsNav: NavSection[] = [
  {
    title: "Getting Started",
    items: [
      { title: "Getting Started", href: "/docs/getting-started/" },
      { title: "How to Play", href: "/docs/how-to-play/" },
      { title: "Controls", href: "/docs/controls/" },
    ],
  },
  {
    title: "Advanced",
    items: [
      { title: "AI Players", href: "/docs/ai-players/" },
      { title: "Development", href: "/docs/development/" },
    ],
  },
];

export function getFlatNavItems(): NavItem[] {
  return docsNav.flatMap((section) => section.items);
}
