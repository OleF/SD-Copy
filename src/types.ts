// Types for the photo import application

export interface VolumeInfo {
  name: string;
  path: string;
  imageCount: number;
  sampleImages: string[];
}

export interface ImportRequest {
  volumePath: string;
  destinationRoot: string;
  folderName: string;
}

export interface ImportProgress {
  importId: string;
  totalFiles: number;
  copiedFiles: number;
  currentFile: string;
  status: "running" | "completed" | "failed" | "cancelled";
  error?: string;
}

export interface ImportManifestEntry {
  sourcePath: string;
  destPath: string;
  sizeBytes: number;
  timestamp: string;
}

export interface ScanResult {
  hasImages: boolean;
  imageCount: number;
  samplePaths: string[];
}

