
import { FileDTO } from "../../dtos/input/file-dto";
import { VectorSearchResult } from "../../services/search/vector/dtos/input/vector-search-result";
import { FileModel } from "../file-model";

export function vectorResultToModel(result: VectorSearchResult): FileModel {
    const dto: FileDTO = {
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