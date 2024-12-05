import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../../dtos/output/search-params-dto";
import { environment } from "../../../../../environments/environment";
import { FileModel } from "../../../models/file-model";
import { StreamingSearchParamsDTO } from "../../../dtos/output/streaming-search-params-dtos";

// Expects the 'SearchForJunk' service to be active
@Injectable({ 'providedIn': 'root' })
export class SearchEngineService {
    constructor(private http: HttpClient) { }

    async query(searchParams: SearchParamsDTO): Promise<FileModel[]> {
        return new Promise((resolve, reject) => {
            const url = environment.searchapi.url;
            this.http.post<FileModel[]>(`${url}/query`, searchParams).subscribe({
                next: (response) => {
                    resolve(response);
                },
                error: (error) => {
                    console.error('Search query error', error);
                    reject(error);
                },
            });
        });
    }
}