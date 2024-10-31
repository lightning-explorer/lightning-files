import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../dtos/search-params-dto";
import { FileDTOReceived } from "../../dtos/file-dto-received";
import { environment } from "../../../../environments/environment";
import { invoke } from "@tauri-apps/api/core";

@Injectable({ 'providedIn': 'root' })
export class LocalSearchEngineService {
    constructor() { }

    async query(params: SearchParamsDTO): Promise<FileDTOReceived[]> {
        return invoke<FileDTOReceived[]>("search_index_query", {
            params
        }).then(result =>
            result
        ).catch((err) => {
            console.log("error executing query", err);
            return [];
        })
    }
}