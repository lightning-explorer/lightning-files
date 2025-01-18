import { FileModel } from "@core/models/file-model";

export interface FileState{
    highlightedText: string,
    draggedOver:boolean,
    hide:boolean,
    
    model:FileModel,
}

  /** Default file state factory */
  export function defaultFileState(model:FileModel): FileState {
    return {
      highlightedText: "",
      draggedOver: false,
      hide: false,

      model
    };
  }