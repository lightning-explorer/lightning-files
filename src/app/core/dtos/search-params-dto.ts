import { DateRange } from "@core/models/date-range";

export interface SearchParamsDTO {
    Name?: string,
    Metadata?: string,
    DateRange?: DateRange,
    FilePath?: string,
    NumResults: number,
}