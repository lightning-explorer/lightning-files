import { Injectable } from '@angular/core';

@Injectable({
    providedIn: 'root'
})
export class IconService {
    private icons: { [key: string]: string } = {
        default: `<svg xmlns="http://www.w3.org/2000/svg"  width="{size}" height="{size}" viewBox="0 0 50 50"><g fill="none" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"><path stroke="#306cfe" d="M37.5 43.75h-25a2.083 2.083 0 0 1-2.083-2.083V8.333A2.083 2.083 0 0 1 12.5 6.25h14.583l12.5 12.5v22.917A2.083 2.083 0 0 1 37.5 43.75"/><path stroke="#344054" d="m39.583 18.75l-12.5-12.5L25 16.667z"/></g></svg>`,
        folder: '<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 32 32"><g fill="none"><path fill="#ffb02e" d="m15.385 7.39l-2.477-2.475A3.12 3.12 0 0 0 10.698 4H4.126A2.125 2.125 0 0 0 2 6.125V13.5h28v-3.363a2.125 2.125 0 0 0-2.125-2.125H16.888a2.13 2.13 0 0 1-1.503-.621"/><path fill="#fcd53f" d="M27.875 30H4.125A2.12 2.12 0 0 1 2 27.888V13.112C2 11.945 2.951 11 4.125 11h23.75c1.174 0 2.125.945 2.125 2.112v14.776A2.12 2.12 0 0 1 27.875 30"/></g></svg>',
        hardDrive: '<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 512 512"><path fill="#fff" d="M0 96c0-35.3 28.7-64 64-64h384c35.3 0 64 28.7 64 64v184.4c-17-15.2-39.4-24.4-64-24.4H64c-24.6 0-47 9.2-64 24.4zm64 192h384c35.3 0 64 28.7 64 64v64c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64v-64c0-35.3 28.7-64 64-64m256 128a32 32 0 1 0 0-64a32 32 0 1 0 0 64m128-32a32 32 0 1 0-64 0a32 32 0 1 0 64 0"/></svg>',
    };

    getIcon(name: string, size: string = '2em'): string | undefined {
        const iconSvg = this.icons[name];
        if (!iconSvg) {
            return undefined;
        }
        return iconSvg.replace(/{size}/g, size);
    }
}