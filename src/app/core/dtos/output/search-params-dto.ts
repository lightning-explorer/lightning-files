export interface SearchParamsDTO {
    Name?: string,
    Metadata?: string,
    DateRange?: DateRange,
    FilePath?: string,
    NumResults: number,
}

interface DateRange {
    Start: Date
    End: Date
}