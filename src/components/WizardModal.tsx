import { useState } from 'react';
import { VolumeInfo } from '../types';
import { getDefaultFolderName } from '../utils';

interface WizardModalProps {
  volume: VolumeInfo;
  onImport: (folderName: string) => void;
  onCancel: () => void;
}

export const WizardModal = ({ volume, onImport, onCancel }: WizardModalProps) => {
  const [folderName, setFolderName] = useState(getDefaultFolderName());

  const handleImport = () => {
    if (folderName.trim()) {
      onImport(folderName.trim());
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <h2>🎉 Bilder funnet!</h2>
        <div className="modal-body">
          <p>
            <strong>Volum:</strong> {volume.name}
          </p>
          <p>
            <strong>Antall bilder:</strong> {volume.imageCount}
          </p>

          {volume.sampleImages.length > 0 && (
            <div className="sample-images">
              <p><strong>Eksempel filer:</strong></p>
              <ul>
                {volume.sampleImages.slice(0, 5).map((img, idx) => (
                  <li key={idx}>{img}</li>
                ))}
                {volume.sampleImages.length > 5 && <li>... og {volume.sampleImages.length - 5} flere</li>}
              </ul>
            </div>
          )}

          <div className="folder-name-input">
            <label htmlFor="folderName">
              <strong>Mappenavn:</strong>
            </label>
            <input
              id="folderName"
              type="text"
              value={folderName}
              onChange={(e) => setFolderName(e.target.value)}
              placeholder="DDMMYY - Import"
            />
            <small>Filene vil bli kopiert til: [Destinasjon]/{folderName}</small>
          </div>
        </div>

        <div className="modal-actions">
          <button onClick={onCancel} className="btn-secondary">
            Avbryt
          </button>
          <button onClick={handleImport} className="btn-primary" disabled={!folderName.trim()}>
            Importer bilder
          </button>
        </div>
      </div>
    </div>
  );
};


