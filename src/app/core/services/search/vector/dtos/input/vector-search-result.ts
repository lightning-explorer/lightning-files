import { FileModel } from "./file_model";

export interface VectorSearchResult {
    File: FileModel,
    Score: number
}