
import React from 'react';
import { Week } from '../types';
import { ActiveView } from '../App';
import { CheckCircleIcon } from './icons/CheckCircleIcon';
import { CircleIcon } from './icons/CircleIcon';
import { BookOpenIcon } from './icons/BookOpenIcon';
import { LogoIcon } from './icons/LogoIcon';
import { ClipboardDocumentCheckIcon } from './icons/ClipboardDocumentCheckIcon';
import { LinkIcon } from './icons/LinkIcon';

interface SidebarProps {
  weeks: Week[];
  selectedWeekId: number;
  completedWeeks: Set<number>;
  onSelectWeek: (id: number) => void;
  onSelectView: (view: ActiveView) => void;
  activeView: ActiveView;
}

const SidebarButton: React.FC<{
  onClick: () => void;
  isActive: boolean;
  icon: React.ReactNode;
  label: string;
}> = ({ onClick, isActive, icon, label }) => (
  <button
    onClick={onClick}
    className={`w-full text-left flex items-center gap-3 p-2 rounded-md transition-colors duration-200 ${
      isActive
        ? 'bg-rust-orange/20 text-rust-orange font-semibold'
        : 'text-slate-400 hover:bg-slate-700/50 hover:text-slate-200'
    }`}
  >
    {icon}
    <span className="flex-1">{label}</span>
  </button>
);


export const Sidebar: React.FC<SidebarProps> = ({ weeks, selectedWeekId, completedWeeks, onSelectWeek, onSelectView, activeView }) => {
  return (
    <aside className="w-72 bg-slate-800/50 border-r border-slate-700/50 p-6 flex-shrink-0 hidden md:flex flex-col">
      <div className="flex items-center gap-2 mb-8">
        <LogoIcon className="w-8 h-8 text-rust-orange"/>
        <span className="text-xl font-bold text-white">Aprende Rust</span>
      </div>
      <nav className="flex-grow">
        <p className="px-2 text-xs font-semibold text-slate-500 uppercase tracking-wider mb-2">Plan de Estudio</p>
        <ul>
          {weeks.map(week => {
            const isSelected = activeView === 'week' && week.id === selectedWeekId;
            const isCompleted = completedWeeks.has(week.id);
            return (
              <li key={week.id} className="mb-2">
                 <SidebarButton
                  onClick={() => onSelectWeek(week.id)}
                  isActive={isSelected}
                  icon={isCompleted ? <CheckCircleIcon className="w-5 h-5 text-green-400" /> : <CircleIcon className="w-5 h-5 text-slate-500" />}
                  label={week.title}
                 />
              </li>
            );
          })}
        </ul>
      </nav>
      <div>
         <p className="px-2 text-xs font-semibold text-slate-500 uppercase tracking-wider mb-2 mt-4">Anexos</p>
         <ul className="space-y-2">
            <li>
                <SidebarButton
                    onClick={() => onSelectView('solutions')}
                    isActive={activeView === 'solutions'}
                    icon={<ClipboardDocumentCheckIcon className="w-5 h-5" />}
                    label="Soluciones de Ejercicios"
                />
            </li>
            <li>
                <SidebarButton
                    onClick={() => onSelectView('resources')}
                    isActive={activeView === 'resources'}
                    icon={<BookOpenIcon className="w-5 h-5" />}
                    label="Herramientas y Recursos"
                />
            </li>
            <li>
                <SidebarButton
                    onClick={() => onSelectView('sites')}
                    isActive={activeView === 'sites'}
                    icon={<LinkIcon className="w-5 h-5" />}
                    label="Sitios de Interés"
                />
            </li>
        </ul>
      </div>
    </aside>
  );
};