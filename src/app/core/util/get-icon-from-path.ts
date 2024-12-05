export function getIconFromPath(path: string): string {
    const ext = getFileExtension(path);
    switch (ext) {
        case 'png':
            return "png";
        case 'jpg':
            return "jpg";
        case 'txt':
            return "txt";
        case 'zip':
            return "zip";
        case 'xlsx':
            return "excel";
        case 'docx':
        case 'dotm':
            return "word";
        case 'md':
            return 'markdown'
        case 'pdf':
            return 'pdf'
    }
    return "default";
}

function getFileExtension(filePath: string) {
    const parts = filePath.split('.');
    return parts.length > 1 ? parts.pop() : '';
}