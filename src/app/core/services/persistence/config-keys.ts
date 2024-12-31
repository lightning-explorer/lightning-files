import { FileModel } from "../../models/file-model";

export interface ConfigKeys {
  /** `true` if it is the user's first time opening the app */
  isFirstUse: boolean;
  moveItemsDontAskAgain: boolean;
  pinnedFiles: FileModel[];
  /** A list of file paths that should be easy for the user to access. (example: Downloads, Desktop, etc) */
  quickAccessFiles: string[];
  lastDirectoryAt: string|undefined;
}

export function getDefaultConfig(): ConfigKeys {
  return {
    isFirstUse: true,
    moveItemsDontAskAgain: false,
    pinnedFiles: [],
    quickAccessFiles: [],
    lastDirectoryAt:undefined,
  };
}
