export interface GetFilesParamsModel{
    ShowHidden:boolean
}

export function defaultParams():GetFilesParamsModel{
    return {ShowHidden: false};
}