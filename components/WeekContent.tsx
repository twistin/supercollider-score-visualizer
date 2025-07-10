
import React from 'react';
import { Week } from '../types';
import { Card } from './Card';
import { BookOpenIcon } from './icons/BookOpenIcon';
import { BeakerIcon } from './icons/BeakerIcon';
import { WrenchScrewdriverIcon } from './icons/WrenchScrewdriverIcon';
import { CheckCircleIcon } from './icons/CheckCircleIcon';

interface WeekContentProps {
  week: Week;
  isCompleted: boolean;
  toggleCompletion: (id: number) => void;
}

export const WeekContent: React.FC<WeekContentProps> = ({ week, isCompleted, toggleCompletion }) => {
  return (
    <article className="animate-fade-in">
      <header className="mb-8">
        <h2 className="text-3xl font-bold text-slate-100">{week.title}</h2>
        <p className="text-lg text-slate-400 mt-2">{week.summary}</p>
      </header>

      <div className="space-y-8">
        {week.topics.map((topic, index) => (
          <section key={index}>
            <h3 className="text-2xl font-semibold text-rust-orange mb-4">{topic.title}</h3>
            <div className="prose prose-invert max-w-none prose-p:text-slate-300 prose-li:text-slate-300 prose-strong:text-slate-100">
                {topic.content}
            </div>
          </section>
        ))}

        <Card>
            <h4 className="flex items-center gap-3 text-xl font-semibold text-slate-200 mb-3">
                <BookOpenIcon className="w-6 h-6 text-blue-400"/>
                Lectura Recomendada
            </h4>
            <div className="text-slate-400">{week.reading}</div>
        </Card>
        
        <Card>
            <h4 className="flex items-center gap-3 text-xl font-semibold text-slate-200 mb-3">
                <BeakerIcon className="w-6 h-6 text-green-400"/>
                Ejercicio Práctico
            </h4>
            <div className="text-slate-400">{week.exercise}</div>
        </Card>

        {week.project && (
            <Card>
                 <h4 className="flex items-center gap-3 text-xl font-semibold text-slate-200 mb-3">
                    <WrenchScrewdriverIcon className="w-6 h-6 text-yellow-400"/>
                    Proyecto Práctico
                </h4>
                <div className="text-slate-400">{week.project}</div>
            </Card>
        )}
      </div>

      <div className="mt-12 text-center">
        <button 
          onClick={() => toggleCompletion(week.id)}
          className={`inline-flex items-center gap-2 px-6 py-3 rounded-lg font-semibold transition-all duration-200 ${
            isCompleted
              ? 'bg-green-500/80 text-white hover:bg-green-500'
              : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
          }`}
        >
          {isCompleted ? <CheckCircleIcon className="w-6 h-6"/> : null}
          {isCompleted ? 'Semana Completada' : 'Marcar como Completada'}
        </button>
      </div>

       {week.finalWords && (
         <div className="mt-12 p-6 bg-slate-800/50 rounded-lg border border-slate-700">
            {week.finalWords}
         </div>
       )}

    </article>
  );
};
