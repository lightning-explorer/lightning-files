import { FileModel } from "../../../features/home-page/models/FileModel";

export interface ConfigKeys {
    moveItemsDontAskAgain:boolean,
    pinnedFiles:FileModel[]
}

export function getDefaultConfig():ConfigKeys{
    return {moveItemsDontAskAgain:false, pinnedFiles:[]};
}