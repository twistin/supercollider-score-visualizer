
import React from 'react';
import { Week, Resource, SiteLink, SiteCategory } from '../types';
import { ActiveView } from '../App';
import { WeekContent } from './WeekContent';
import { Card } from './Card';

interface ContentDisplayProps {
  week: Week | undefined;
  isCompleted: boolean;
  toggleCompletion: (id: number) => void;
  activeView: ActiveView;
  resources: Resource[];
  finalThoughts: React.ReactNode;
  studyPlan: Week[];
  interestingSites: SiteLink[];
}

const renderResources = (resources: Resource[], finalThoughts: React.ReactNode) => (
    <div className="animate-fade-in">
        <h2 className="text-3xl font-bold text-slate-100 border-b-2 border-rust-orange/50 pb-2 mb-6">Herramientas y Recursos</h2>
        <p className="text-slate-400 mb-8">Para acompañar tu aprendizaje, aquí tienes algunas herramientas y recursos gratuitos que te serán de gran ayuda:</p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {resources.map((resource, index) => (
              <Card key={index} className="flex flex-col">
                  <div className="flex items-center gap-4 mb-3">
                      {resource.icon}
                      <h3 className="text-xl font-semibold text-slate-100">{resource.title}</h3>
                  </div>
                  <div className="text-slate-400 flex-grow">{resource.description}</div>
              </Card>
          ))}
        </div>
        <div className="mt-12 p-6 bg-slate-800/50 rounded-lg border border-slate-700">
          {finalThoughts}
        </div>
    </div>
);

const renderSolutions = (studyPlan: Week[]) => (
    <div className="animate-fade-in">
        <h2 className="text-3xl font-bold text-slate-100 border-b-2 border-rust-orange/50 pb-2 mb-6">Anexo: Soluciones a los Ejercicios</h2>
        <p className="text-slate-400 mb-8">Aquí encontrarás las soluciones a los ejercicios prácticos propuestos cada semana. Úsalas para comparar tus resultados o si necesitas una pista.</p>
        <div className="space-y-8">
            {studyPlan.map(week => week.exerciseSolution && (
                <Card key={week.id}>
                    <h3 className="text-xl font-semibold text-slate-200 mb-4">{week.title}</h3>
                    <div className="prose prose-invert max-w-none prose-p:text-slate-300 prose-li:text-slate-300 prose-strong:text-slate-100">
                        {week.exerciseSolution}
                    </div>
                </Card>
            ))}
        </div>
    </div>
);

const renderSites = (sites: SiteLink[]) => {
    const categories: SiteCategory[] = ['Oficial', 'Práctica', 'Comunidad', 'Noticias'];
    const groupedSites = categories.map(category => ({
        category,
        sites: sites.filter(site => site.category === category)
    }));

    return (
        <div className="animate-fade-in">
            <h2 className="text-3xl font-bold text-slate-100 border-b-2 border-rust-orange/50 pb-2 mb-6">Sitios de Interés</h2>
            <p className="text-slate-400 mb-8">El ecosistema de Rust es amplio y activo. Aquí tienes algunos enlaces clave para seguir aprendiendo, resolver dudas y mantenerte al día.</p>
            <div className="space-y-10">
                {groupedSites.map(({ category, sites }) => sites.length > 0 && (
                    <section key={category}>
                        <h3 className="text-2xl font-semibold text-rust-orange mb-4">{category}</h3>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                            {sites.map(site => (
                                <a href={site.url} target="_blank" rel="noopener noreferrer" key={site.title} className="block hover:scale-[1.02] transition-transform duration-200">
                                    <Card className="h-full flex flex-col">
                                        <div className="flex items-center gap-4 mb-3">
                                            {site.icon}
                                            <h4 className="text-xl font-semibold text-slate-100">{site.title}</h4>
                                        </div>
                                        <div className="text-slate-400 flex-grow">{site.description}</div>
                                    </Card>
                                </a>
                            ))}
                        </div>
                    </section>
                ))}
            </div>
        </div>
    );
};


export const ContentDisplay: React.FC<ContentDisplayProps> = (props) => {
    const { week, isCompleted, toggleCompletion, activeView, resources, finalThoughts, studyPlan, interestingSites } = props;

    switch(activeView) {
        case 'resources':
            return renderResources(resources, finalThoughts);
        case 'solutions':
            return renderSolutions(studyPlan);
        case 'sites':
            return renderSites(interestingSites);
        case 'week':
            if (!week) {
                return <div className="text-center text-slate-500">Selecciona una semana para comenzar.</div>;
            }
            return <WeekContent week={week} isCompleted={isCompleted} toggleCompletion={toggleCompletion} />;
        default:
             return <div className="text-center text-slate-500">Contenido no encontrado.</div>;
    }
};