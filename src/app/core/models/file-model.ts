export interface FileModelVariables{
    HighlightedText: string,
    DraggedOver:boolean,
    ShouldHide:boolean,
}
/**
 Corresponds to `SystemFileModel` in the Rust backend
 */
export interface FileModel {
    Name: string,
    DateModified: string,
    Metadata: string,
    FilePath: string,
    Score: number,
    Popularity: number,
    IsDirectory: boolean,
    // TODO: do something with these, as they are just metadata
    Variables:FileModelVariables
}

export function newDefaultFileModel(): FileModel {
    return {
        Name: "test",
        DateModified: "test",
        Metadata: "test",
        FilePath: "test",
        Score: 0,
        Popularity: 0,
        IsDirectory: false,
        Variables: {HighlightedText: "", DraggedOver: false, ShouldHide: false}
    }
}