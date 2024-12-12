export interface DirectoryMetadata {
    /** A path such as 'C://' */
    directory: string,
    /** `true` if the program has access to this directory and is able to open it without permission issues */
    isAccessible: boolean
}

export function newDirMetadataDefault(): DirectoryMetadata {
    return { directory: 'C:', isAccessible: false }
}