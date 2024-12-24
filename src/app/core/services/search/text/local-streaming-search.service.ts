import { Injectable } from "@angular/core";
import { FileModel } from "../../../models/file-model";
import { BehaviorSubject } from "rxjs";
import { StreamingSearchParamsDTO } from "../../../dtos/streaming-search-params-dtos";
import { TauriCommandsService } from "../../tauri/commands.service";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";

@Injectable({ providedIn: "root" })
export class LocalStreamingSearchService {
  private filesSubject = new BehaviorSubject<FileModel[]>([]);
  public files$ = this.filesSubject.asObservable();

  constructor(private commandsService: TauriCommandsService) {}

  async query(params: StreamingSearchParamsDTO) {
    this.filesSubject.next([]);
    const innerParams = params.Params;
    // Because fuzzy queries have a tendency to return junk results when a low character
    // count is given, ignore calling the query altogether if the word length doesn't suffice
    if (
      innerParams.QueryType === "Fuzzy" &&
      !this.fuzzyQueryIsAdequate(innerParams)
    )
      return; // Early return
    await this.commandsService.searchIndexQueryStreamingOrganized(
      params,
      (emittedFiles) => {
        // Check if the emitted result corresponds to the correct query.
        // The query string is stored in the metadata field
        if (emittedFiles.Metadata == params.Params.FilePath) {
          this.filesSubject.next(emittedFiles.Data);
        }
      }
    );
  }

  fuzzyQueryIsAdequate(params: SearchParamsDTO): boolean {
    const minLen = 4;
    if (params.FilePath !== undefined && params.FilePath.length < minLen) {
      return false;
    }
    if (params.Name !== undefined && params.Name.length < minLen) {
      return false;
    }
    return true;
  }
}
