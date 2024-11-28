import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../../dtos/output/search-params-dto";
import { FileDTO } from "../../../dtos/input/file-dto";
import { environment } from "../../../../../environments/environment";
import { invoke } from "@tauri-apps/api/core";

@Injectable({ 'providedIn': 'root' })
export class LocalSearchEngineService {
    constructor() { }

    async query(params: SearchParamsDTO): Promise<FileDTO[]> {
        return invoke<FileDTO[]>("search_index_query", {
            params
        }).then(result =>
            result
        ).catch((err) => {
            console.log("error executing query", err);
            return [];
        })
    }
}