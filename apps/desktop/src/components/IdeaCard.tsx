import type { IdeaBundle } from '@idea-engine/shared';
import './IdeaCard.css';

interface IdeaCardProps {
  bundle: IdeaBundle;
}

export function IdeaCard({ bundle }: IdeaCardProps) {
  return (
    <div className="idea-card">
      <div className="idea-card-header">
        <span className="provider-badge">{bundle.provider}</span>
        <span className="model-badge">{bundle.model}</span>
      </div>
      {bundle.ideas.length > 0 && (
        <section className="idea-section">
          <h4>Ideas</h4>
          <ul>
            {bundle.ideas.map((i, idx) => (
              <li key={idx}>
                <strong>{i.title}</strong>
                <p>{i.description}</p>
                {i.rationale && <em>{i.rationale}</em>}
              </li>
            ))}
          </ul>
        </section>
      )}
      {bundle.stepPlan.length > 0 && (
        <section className="idea-section">
          <h4>Step Plan</h4>
          <ol>
            {bundle.stepPlan
              .sort((a, b) => a.order - b.order)
              .map((s, idx) => (
                <li key={idx}>
                  {s.action}
                  {s.details && <span className="step-details"> — {s.details}</span>}
                </li>
              ))}
          </ol>
        </section>
      )}
      {bundle.risks.length > 0 && (
        <section className="idea-section">
          <h4>Risks</h4>
          <ul>
            {bundle.risks.map((r, idx) => (
              <li key={idx} className={`risk-${r.severity}`}>
                {r.description}
                {r.mitigation && <span> — Mitigation: {r.mitigation}</span>}
              </li>
            ))}
          </ul>
        </section>
      )}
      {bundle.dependencies.length > 0 && (
        <section className="idea-section">
          <h4>Dependencies</h4>
          <ul>
            {bundle.dependencies.map((d, idx) => (
              <li key={idx}>{d}</li>
            ))}
          </ul>
        </section>
      )}
      <section className="idea-section effort">
        <strong>Effort:</strong> {bundle.effort.time}
        {bundle.effort.cost && ` | Cost: ${bundle.effort.cost}`}
        {bundle.effort.complexity && ` | Complexity: ${bundle.effort.complexity}`}
      </section>
      {bundle.nextActions.length > 0 && (
        <section className="idea-section">
          <h4>Next Actions</h4>
          <ul>
            {bundle.nextActions.map((a, idx) => (
              <li key={idx}>
                <span className={`priority-${a.priority}`}>{a.priority}</span>: {a.action}
              </li>
            ))}
          </ul>
        </section>
      )}
    </div>
  );
}
