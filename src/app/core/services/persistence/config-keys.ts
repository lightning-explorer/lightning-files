import { FileModel } from "../../models/file-model";

export interface ConfigKeys {
    moveItemsDontAskAgain: boolean,
    pinnedFiles: FileModel[]
}

export function getDefaultConfig(): ConfigKeys {
    return { moveItemsDontAskAgain: false, pinnedFiles: [] };
}