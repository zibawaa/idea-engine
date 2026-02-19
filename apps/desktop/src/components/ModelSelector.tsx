import './ModelSelector.css';

const PROVIDERS = [
  { id: 'openai', label: 'OpenAI' },
  { id: 'anthropic', label: 'Anthropic' },
  { id: 'gemini', label: 'Gemini' },
];

interface ModelSelectorProps {
  selected: string[];
  onChange: (providers: string[]) => void;
}

export function ModelSelector({ selected, onChange }: ModelSelectorProps) {
  const toggle = (id: string) => {
    if (selected.includes(id)) {
      onChange(selected.filter((p) => p !== id));
    } else {
      onChange([...selected, id]);
    }
  };

  return (
    <div className="model-selector">
      <span className="selector-label">Models:</span>
      {PROVIDERS.map((p) => (
        <label key={p.id} className="provider-checkbox">
          <input
            type="checkbox"
            checked={selected.includes(p.id)}
            onChange={() => toggle(p.id)}
          />
          {p.label}
        </label>
      ))}
    </div>
  );
}
