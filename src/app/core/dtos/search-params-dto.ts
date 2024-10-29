export interface SearchParamsDTO{
    Name?:string,
    Metadata?:string,
    DateRange?:DateRange,
    FilePath?:string,
}

interface DateRange{
    Start:Date
    End:Date
}