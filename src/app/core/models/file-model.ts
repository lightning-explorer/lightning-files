
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