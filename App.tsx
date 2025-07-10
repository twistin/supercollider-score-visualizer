
import React, { useState, useEffect, useRef } from 'react';
import { Sidebar } from './components/Sidebar';
import { ContentDisplay } from './components/ContentDisplay';
import { studyPlan, resources, finalThoughts, interestingSites } from './data/studyPlanData';
import { Week } from './types';
import { LogoIcon } from './components/icons/LogoIcon';

export type ActiveView = 'week' | 'resources' | 'solutions' | 'sites';

const App: React.FC = () => {
  const [selectedWeekId, setSelectedWeekId] = useState<number>(studyPlan[0].id);
  const [completedWeeks, setCompletedWeeks] = useState<Set<number>>(new Set());
  const [activeView, setActiveView] = useState<ActiveView>('week');
  const contentRef = useRef<HTMLDivElement>(null);

  const handleSelectWeek = (id: number) => {
    setSelectedWeekId(id);
    setActiveView('week');
  };
  
  const handleSelectView = (view: ActiveView) => {
      setActiveView(view);
  }

  const toggleWeekCompletion = (id: number) => {
    setCompletedWeeks(prev => {
      const newSet = new Set(prev);
      if (newSet.has(id)) {
        newSet.delete(id);
      } else {
        newSet.add(id);
      }
      return newSet;
    });
  };

  useEffect(() => {
    if (contentRef.current) {
      contentRef.current.scrollTo(0, 0);
    }
  }, [selectedWeekId, activeView]);

  const selectedWeek: Week | undefined = studyPlan.find(w => w.id === selectedWeekId);

  return (
    <div className="flex h-screen bg-slate-900 text-slate-300">
      <Sidebar 
        weeks={studyPlan}
        selectedWeekId={selectedWeekId}
        completedWeeks={completedWeeks}
        onSelectWeek={handleSelectWeek}
        onSelectView={handleSelectView}
        activeView={activeView}
      />
      <main ref={contentRef} className="flex-1 overflow-y-auto p-4 md:p-8 lg:p-12">
        <div className="max-w-4xl mx-auto">
            <div className="flex items-center gap-4 mb-8">
                <LogoIcon className="h-16 w-16 text-rust-orange" />
                <div>
                    <h1 className="text-4xl font-bold text-slate-100">Plan de Estudio de Rust</h1>
                    <p className="text-slate-400 text-lg">Un viaje interactivo para convertirte en un Rustacean</p>
                </div>
            </div>
          
            <ContentDisplay
                week={selectedWeek}
                isCompleted={completedWeeks.has(selectedWeekId)}
                toggleCompletion={toggleWeekCompletion}
                activeView={activeView}
                resources={resources}
                finalThoughts={finalThoughts}
                studyPlan={studyPlan}
                interestingSites={interestingSites}
            />
        </div>
      </main>
    </div>
  );
};

export default App;