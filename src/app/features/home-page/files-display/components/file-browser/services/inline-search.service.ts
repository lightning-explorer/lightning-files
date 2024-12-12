import { Injectable, input } from "@angular/core";
import { InlineQueryDTO } from "@core/dtos/output/inline-query-dto";
import { FileModel } from "@core/models/file-model";
import { TauriCommandsService } from "@core/services/tauri/commands.service";
import { filterAlphanumeric } from "@shared/services/keyboard-press-filter";
import { BehaviorSubject, Observable } from "rxjs";


/**
 Calls the Rust backend to handle the query operation
 */
@Injectable()
export class InlineSearchService {

    constructor(private commandsService: TauriCommandsService) { }

    private searchQuerySubject = new BehaviorSubject<string>("");
    searchQuery$ = this.searchQuerySubject.asObservable();

    private numberOfFoundItemsSubject = new BehaviorSubject<number>(0);
    numberOfFoundItems$ = this.numberOfFoundItemsSubject.asObservable();

    private firstOccurenceOfQueryIndexSubject = new BehaviorSubject<number>(0);
    firstOccurenceOfQueryIndex$ = this.firstOccurenceOfQueryIndexSubject.asObservable();

    async handleKeydown(event: KeyboardEvent, files: FileModel[]) {
        if (!this.isInputFocused()) {
            const key = filterAlphanumeric(event);
            let input_was_backspace = false;
            if (event.key == "Backspace") {
                if (this.searchQuerySubject.getValue().length == 0) {
                    return;
                }
                this.searchQuerySubject.next(this.searchQuerySubject.getValue().slice(0, -1));
                input_was_backspace = true;
            } else {
                if (key == undefined) {
                    return;
                }
                let searchQueryValue = this.searchQuerySubject.getValue();
                this.searchQuerySubject.next(searchQueryValue += event.key);
            }

            const queryDto: InlineQueryDTO = { Query: this.searchQuerySubject.getValue() };
            const dtos = await this.query(queryDto, files);

            this.numberOfFoundItemsSubject.next(dtos.length);

            for (let i = 0; i < files.length; i++) {
                let file = files[i];
                if (dtos.some(x => x.Name === file.Name)) {
                    if (!input_was_backspace)
                        this.firstOccurenceOfQueryIndexSubject.next(i);

                    file.HighlightedText = this.searchQuerySubject.getValue();
                } else {
                    file.HighlightedText = "";
                }
            }
        }
    }

    clearQuery() {
        this.searchQuerySubject.next("");
    }

    private isInputFocused(): boolean {
        const focusedElement = document.activeElement;
        const result = focusedElement && (focusedElement.tagName === 'INPUT' || focusedElement.tagName === 'TEXTAREA');
        return result ? result : false;
    }

    // TODO: Typescript now handles the inline search. The Tauri command for the inline search may be unecessary
    private async query(query: InlineQueryDTO, files: FileModel[]): Promise<FileModel[]> {
        const queryLower = query.Query.toLowerCase();
        return files.filter(file => file.Name.toLowerCase().includes(queryLower));
        //return await this.commandsService.searchFilesInline(query);
    }
}