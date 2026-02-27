import { useEffect, useState, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import './App.css';
import { WizardModal } from './components/WizardModal';
import { ProgressView } from './components/ProgressView';
import { VolumeInfo, ImportProgress, ScanResult } from './types';

function App() {
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [detectedVolume, setDetectedVolume] = useState<VolumeInfo | null>(null);
  const [importProgress, setImportProgress] = useState<ImportProgress | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Use refs to track latest state in polling interval
  const detectedVolumeRef = useRef<VolumeInfo | null>(null);
  const importProgressRef = useRef<ImportProgress | null>(null);

  // Keep refs in sync with state
  useEffect(() => {
    detectedVolumeRef.current = detectedVolume;
  }, [detectedVolume]);

  useEffect(() => {
    importProgressRef.current = importProgress;
  }, [importProgress]);

  // Start monitoring on mount
  useEffect(() => {
    let intervalId: NodeJS.Timeout;

    const startPolling = async () => {
      setIsMonitoring(true);
      setError(null);

      // Poll for new volumes every 1500ms
      intervalId = setInterval(async () => {
        try {
          // Skip if we already have a detected volume or import in progress
          if (detectedVolumeRef.current || importProgressRef.current) {
            return;
          }

          const volumes: string[] = await invoke('list_volumes');
          console.log('Polling volumes:', volumes);

          // Check each volume for images
          for (const volumePath of volumes) {
            // Skip Macintosh HD and system volumes
            if (volumePath.includes('Macintosh HD') || volumePath === '/') {
              continue;
            }

            console.log('Scanning volume:', volumePath);
            const result: ScanResult = await invoke('scan_volume_for_images', { volumePath });
            console.log('Scan result:', result);

            if (result.hasImages && result.imageCount > 0) {
              const volumeName = volumePath.split('/').pop() || volumePath;
              setDetectedVolume({
                name: volumeName,
                path: volumePath,
                imageCount: result.imageCount,
                sampleImages: result.samplePaths,
              });
              break;
            }
          }
        } catch (err) {
          console.error('Error monitoring volumes:', err);
        }
      }, 1500);
    };

    startPolling();

    // Listen for import progress events
    const unlistenProgress = listen<ImportProgress>('import-progress', (event) => {
      setImportProgress(event.payload);

      // Clear import state when completed or failed
      if (event.payload.status === 'completed' || event.payload.status === 'failed') {
        setTimeout(() => {
          setImportProgress(null);
          setDetectedVolume(null);
        }, 5000);
      }
    });

    return () => {
      if (intervalId) {
        clearInterval(intervalId);
      }
      unlistenProgress.then(fn => fn());
    };
  }, []);

  const handleChooseDestination = async (folderName: string) => {
    try {
      // Open dialog to choose destination folder
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Velg destinasjonsmappe',
      });

      if (selected && typeof selected === 'string') {
        startImport(selected, folderName);
      }
    } catch (err) {
      setError(`Kunne ikke velge mappe: ${err}`);
    }
  };

  const startImport = async (destination: string, folderName: string) => {
    if (!detectedVolume) return;

    try {
      const importId: string = await invoke('start_import', {
        volumePath: detectedVolume.path,
        destinationRoot: destination,
        folderName,
      });

      setImportProgress({
        importId,
        totalFiles: detectedVolume.imageCount,
        copiedFiles: 0,
        currentFile: '',
        status: 'running',
      });
    } catch (err) {
      setError(`Import feilet: ${err}`);
      setDetectedVolume(null);
    }
  };

  const handleCancelImport = async () => {
    if (!importProgress) return;

    try {
      await invoke('cancel_import', { importId: importProgress.importId });
    } catch (err) {
      console.error('Error cancelling import:', err);
    }
  };

  const handleCancelWizard = () => {
    setDetectedVolume(null);
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>📷 SD Copy - Foto Import</h1>
        <p>Automatisk import av bilder fra SD-kort og minnekort</p>
      </header>

      <main className="main-content">
        <div className="status-section">
          <h3>
            <span className="monitoring-indicator"></span>
            Status: {isMonitoring ? 'Overvåker volumer...' : 'Ikke aktiv'}
          </h3>
          <p>Sett inn et SD-kort eller minnekort for å starte import.</p>
        </div>

        {error && (
          <div className="error-message">
            {error}
          </div>
        )}

        {importProgress && (
          <ProgressView
            progress={importProgress}
            onCancel={importProgress.status === 'running' ? handleCancelImport : undefined}
          />
        )}
      </main>

      {detectedVolume && !importProgress && (
        <WizardModal
          volume={detectedVolume}
          onImport={handleChooseDestination}
          onCancel={handleCancelWizard}
        />
      )}
    </div>
  );
}

export default App;





