
import { FileDTO } from "../../dtos/input/file-dto";
import { FileModel } from "../file-model";

export function fileDTOToModel(dto: FileDTO, highlightedText?: string): FileModel {
    return {
        Dto: dto,
        HighlightedText: highlightedText ? highlightedText : ""
    }
}