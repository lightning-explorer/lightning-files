
/**
 Corresponds to `SystemFileModel` in the Rust backend
 */
export interface FileModel {
    Name: string,
    DateModified: string,
    Metadata: string,
    FilePath: string,
    Score: number,
    IsDirectory: boolean,
    HighlightedText: string,
}

export function newDefaultFileModel(): FileModel {
    return {
        Name: "test",
        DateModified: "test",
        Metadata: "test",
        FilePath: "test",
        Score: 0,
        IsDirectory: false,
        HighlightedText: ""
    }
}