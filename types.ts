
import { ReactNode } from 'react';

export interface Topic {
  title: string;
  content: ReactNode;
}

export interface Week {
  id: number;
  title: string;
  summary: string;
  topics: Topic[];
  reading: ReactNode;
  exercise: ReactNode;
  exerciseSolution?: ReactNode;
  project?: ReactNode;
  finalWords?: ReactNode;
}

export interface Resource {
    title: string;
    description: ReactNode;
    icon: ReactNode;
}

export type SiteCategory = 'Oficial' | 'Comunidad' | 'Práctica' | 'Noticias';

export interface SiteLink {
  title: string;
  url: string;
  description: ReactNode;
  category: SiteCategory;
  icon: ReactNode;
}