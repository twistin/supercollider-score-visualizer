
import React, { useState } from 'react';
import { ClipboardIcon } from './icons/ClipboardIcon';
import { CheckIcon } from './icons/CheckIcon';

interface CodeBlockProps {
  children: string;
}

export const CodeBlock: React.FC<CodeBlockProps> = ({ children }) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(children.trim());
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="bg-slate-900 rounded-lg my-4 relative group">
      <button
        onClick={handleCopy}
        className="absolute top-2 right-2 p-1.5 bg-slate-700 text-slate-300 rounded-md opacity-0 group-hover:opacity-100 transition-opacity focus:opacity-100"
        aria-label="Copiar código"
      >
        {copied ? <CheckIcon className="w-5 h-5 text-green-400" /> : <ClipboardIcon className="w-5 h-5" />}
      </button>
      <pre className="p-4 overflow-x-auto text-sm font-mono">
        <code className="text-slate-300">{children.trim()}</code>
      </pre>
    </div>
  );
};
