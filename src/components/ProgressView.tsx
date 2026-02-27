import { ImportProgress } from '../types';

interface ProgressViewProps {
  progress: ImportProgress;
  onCancel?: () => void;
}

export const ProgressView = ({ progress, onCancel }: ProgressViewProps) => {
  const percentage = progress.totalFiles > 0
    ? Math.round((progress.copiedFiles / progress.totalFiles) * 100)
    : 0;

  return (
    <div className="progress-view">
      <h3>Importerer bilder...</h3>

      <div className="progress-bar-container">
        <div className="progress-bar" style={{ width: `${percentage}%` }}>
          <span className="progress-text">{percentage}%</span>
        </div>
      </div>

      <div className="progress-info">
        <p>
          <strong>Status:</strong> {progress.copiedFiles} av {progress.totalFiles} filer kopiert
        </p>
        {progress.currentFile && (
          <p className="current-file">
            <strong>Kopierer:</strong> {progress.currentFile}
          </p>
        )}
      </div>

      {progress.status === 'failed' && progress.error && (
        <div className="error-message">
          <strong>Feil:</strong> {progress.error}
        </div>
      )}

      {progress.status === 'completed' && (
        <div className="success-message">
          ✅ Import fullført! {progress.copiedFiles} filer kopiert.
        </div>
      )}

      {progress.status === 'running' && onCancel && (
        <div className="progress-actions">
          <button onClick={onCancel} className="btn-danger">
            Avbryt import
          </button>
        </div>
      )}
    </div>
  );
};

