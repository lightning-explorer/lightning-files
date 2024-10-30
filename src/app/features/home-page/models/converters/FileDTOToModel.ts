import { FileDTOReceived } from "../../../../core/dtos/file-dto-received";
import { FileModel } from "../FileModel";

export function fileDTOToModel(dto:FileDTOReceived, highlightedText?:string):FileModel{
    return {
        Dto:dto,
        HighlightedText: highlightedText ? highlightedText : ""
    }
}