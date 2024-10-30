import { Injectable } from "@angular/core";
import { FileDTOReceived } from "../../dtos/file-dto-received";
import { InlineQueryDTO } from "../../dtos/inline-query-dto";
import { invoke } from "@tauri-apps/api/core";

Injectable({'providedIn':'root'})
export class InlineSearchService{

    async query(query:InlineQueryDTO):Promise<FileDTOReceived[]>{
        return invoke<FileDTOReceived[]>("search_files_inline", {
            query
        }).then(result =>
            result
        )
    }

}