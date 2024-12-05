import { invoke } from "@tauri-apps/api/core";

export async function isPathAFile(filePath: string): Promise<boolean> {
    return invoke<boolean>("is_path_a_file", {
        filePath
    }).then(result =>
        result
    )
}