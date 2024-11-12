
import { FileDTOReceived } from "../../../../core/dtos/file-dto-received";
import { VectorSearchResult } from "../../../../core/services/search/vector/dtos/input/vector-search-result";
import { FileModel } from "../FileModel";

export function vectorResultToModel(result: VectorSearchResult): FileModel {
    const dto: FileDTOReceived = {
        Name: result.File.Name,
        FilePath: result.File.ParentDir, // TODO: join this with the name
        Metadata: "",
        DateModified: "",
        Score: 0,
        IsDirectory: false
    }
    return {
        Dto: dto,
        HighlightedText: ""
    }
}