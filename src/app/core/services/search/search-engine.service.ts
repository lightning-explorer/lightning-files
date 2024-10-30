import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../dtos/search-params-dto";
import { FileDTOReceived } from "../../dtos/file-dto-received";
import { environment } from "../../../../environments/environment";

@Injectable({ 'providedIn': 'root' })
export class SearchEngineService {
    constructor(private http: HttpClient) { }

    async query(searchParams: SearchParamsDTO): Promise<FileDTOReceived[]> {
        return new Promise((resolve, reject) => {
            const url = environment.searchapi.url;
            this.http.post<FileDTOReceived[]>(`${url}/query`, searchParams).subscribe({
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