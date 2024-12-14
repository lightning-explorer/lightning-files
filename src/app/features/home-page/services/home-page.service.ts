import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";

export type SubPage = "main" | "extended-search";

@Injectable()
export class HomePageService {
    private pageSubject = new BehaviorSubject<SubPage>("extended-search");
    page$ = this.pageSubject.asObservable();

    setPage(page: SubPage) {
        this.pageSubject.next(page);
    }
}