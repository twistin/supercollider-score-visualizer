
import React from 'react';
import { LockClosedIcon } from './icons/LockClosedIcon';
import { SparklesIcon } from './icons/SparklesIcon';
import { ArrowRightIcon } from './icons/ArrowRightIcon';

export const OwnershipVisual: React.FC = () => {
    return (
        <div className="bg-slate-800/50 border border-slate-700 rounded-lg p-6 my-6">
            <h4 className="text-lg font-semibold text-center text-slate-200 mb-4">Explicación Visual: Propiedad y Préstamos</h4>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-center">
                
                {/* Mutable Borrow */}
                <div className="bg-slate-900/50 p-4 rounded-lg">
                    <h5 className="font-semibold text-rust-orange mb-2">Préstamo Mutable (&amp;mut)</h5>
                    <div className="flex items-center justify-center gap-2">
                        <div className="p-2 border border-dashed border-slate-500 rounded">
                            <span className="font-mono">valor</span>
                        </div>
                        <ArrowRightIcon className="w-8 h-8 text-slate-500 shrink-0"/>
                        <div className="p-2 bg-red-500/20 text-red-300 border border-red-500/50 rounded flex items-center gap-2">
                            <LockClosedIcon className="w-5 h-5"/>
                            <span className="font-mono">&amp;mut referencia</span>
                        </div>
                    </div>
                    <p className="text-sm text-slate-400 mt-3">
                        Una referencia mutable <strong className="text-slate-300">bloquea</strong> el valor original. Solo puede existir <strong className="text-slate-300">un préstamo mutable</strong> a la vez.
                    </p>
                </div>

                {/* Immutable Borrows */}
                <div className="bg-slate-900/50 p-4 rounded-lg">
                     <h5 className="font-semibold text-rust-orange mb-2">Préstamos Inmutables (&amp;)</h5>
                    <div className="flex items-center justify-center gap-2">
                        <div className="p-2 border border-dashed border-slate-500 rounded">
                            <span className="font-mono">valor</span>
                        </div>
                         <ArrowRightIcon className="w-8 h-8 text-slate-500 shrink-0"/>
                         <div className="flex flex-col gap-1">
                            <div className="p-2 bg-blue-500/20 text-blue-300 border border-blue-500/50 rounded flex items-center gap-2">
                                <SparklesIcon className="w-5 h-5"/>
                                <span className="font-mono">&amp; referencia 1</span>
                            </div>
                            <div className="p-2 bg-blue-500/20 text-blue-300 border border-blue-500/50 rounded flex items-center gap-2">
                                <SparklesIcon className="w-5 h-5"/>
                                <span className="font-mono">&amp; referencia 2</span>
                            </div>
                         </div>
                    </div>
                    <p className="text-sm text-slate-400 mt-3">
                       Las referencias inmutables <strong className="text-slate-300">congelan</strong> el valor. Puedes tener <strong className="text-slate-300">múltiples préstamos inmutables</strong> simultáneamente.
                    </p>
                </div>
            </div>
            <p className="text-center text-xs text-slate-500 mt-4">Regla clave: O un préstamo mutable exclusivo, o varios inmutables compartidos, pero no ambos a la vez.</p>
        </div>
    );
};
