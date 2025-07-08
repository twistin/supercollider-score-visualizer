
import React, { ReactNode } from 'react';

interface CardProps {
  children: ReactNode;
  className?: string;
}

export const Card: React.FC<CardProps> = ({ children, className = '' }) => {
  return (
    <div className={`bg-slate-800/70 border border-slate-700/80 rounded-lg p-6 shadow-lg ${className}`}>
      {children}
    </div>
  );
};
