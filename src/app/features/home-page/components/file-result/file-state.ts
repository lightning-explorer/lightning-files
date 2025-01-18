import { FileModel, newDefaultFileModel } from "@core/models/file-model";
import { BehaviorSubject } from "rxjs";

export interface FileState{
    highlightedText: BehaviorSubject<string>,
    draggedOver:boolean,
    hide:boolean,
    renameRequested:BehaviorSubject<boolean>,
    
    model:FileModel,
}

  /** Default file state factory */
  export function defaultFileState(model?:FileModel): FileState { 
    return {
      highlightedText: new BehaviorSubject(""),
      draggedOver: false,
      hide: false,
      renameRequested:new BehaviorSubject(false),

      model: model ?? newDefaultFileModel()
    };
  }