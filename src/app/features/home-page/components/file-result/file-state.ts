export interface FileState{
    highlightedText: string,
    draggedOver:boolean,
    hide:boolean,
}

  /** Default file state factory */
  export function defaultFileState(): FileState {
    return {
      highlightedText: "",
      draggedOver: false,
      hide: false,
    };
  }