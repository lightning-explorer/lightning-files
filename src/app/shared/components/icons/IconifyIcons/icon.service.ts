import { Injectable, Renderer2, RendererFactory2 } from "@angular/core";
import { replaceColorWithGradient } from "./util/svg";

@Injectable({ providedIn: "root" })
export class IconService {
  private renderer: Renderer2 = this.rendererFactory.createRenderer(null, null);
  private icons: { [key: string]: string } = {
    default: `<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M13 9V3.5L18.5 9M6 2c-1.11 0-2 .89-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6z"/></svg>`,
    folder:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 32 32"><g fill="none"><path fill="#ffb02e" d="m15.385 7.39l-2.477-2.475A3.12 3.12 0 0 0 10.698 4H4.126A2.125 2.125 0 0 0 2 6.125V13.5h28v-3.363a2.125 2.125 0 0 0-2.125-2.125H16.888a2.13 2.13 0 0 1-1.503-.621"/><path fill="#fcd53f" d="M27.875 30H4.125A2.12 2.12 0 0 1 2 27.888V13.112C2 11.945 2.951 11 4.125 11h23.75c1.174 0 2.125.945 2.125 2.112v14.776A2.12 2.12 0 0 1 27.875 30"/></g></svg>',
    hardDrive:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 512 512"><path fill="currentColor" d="M0 96c0-35.3 28.7-64 64-64h384c35.3 0 64 28.7 64 64v184.4c-17-15.2-39.4-24.4-64-24.4H64c-24.6 0-47 9.2-64 24.4zm64 192h384c35.3 0 64 28.7 64 64v64c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64v-64c0-35.3 28.7-64 64-64m256 128a32 32 0 1 0 0-64a32 32 0 1 0 0 64m128-32a32 32 0 1 0-64 0a32 32 0 1 0 64 0"/></svg>',
    png: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M7.782 14.576c-.186 0-.312.018-.377.036v1.193c.077.018.174.023.306.023c.485 0 .785-.246.785-.659c0-.371-.258-.593-.714-.593"/><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM9.03 16.105c-.313.293-.774.426-1.313.426c-.12 0-.229-.007-.312-.019v1.445h-.906V13.97a7.5 7.5 0 0 1 1.235-.083c.563 0 .966.107 1.235.323c.258.204.432.54.432.936s-.131.731-.371.959m4.302 1.853h-.96l-.863-1.56c-.24-.432-.504-.953-.701-1.427l-.019.006c.024.534.036 1.104.036 1.763v1.218h-.84v-4.042h1.067l.84 1.481c.24.426.479.93.659 1.385h.019a15 15 0 0 1-.078-1.685v-1.182h.84zm4.169-.186a4.5 4.5 0 0 1-1.349.228c-.737 0-1.271-.186-1.644-.546c-.371-.348-.575-.875-.569-1.469c.006-1.344.983-2.111 2.309-2.111c.521 0 .924.103 1.121.198l-.191.731c-.222-.096-.498-.174-.941-.174c-.762 0-1.338.432-1.338 1.308c0 .833.522 1.325 1.271 1.325c.21 0 .378-.024.45-.061v-.846h-.624v-.713h1.505zM14 9h-1V4l5 5z"/></svg>',
    jpg: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM9.239 16.446c0 1.152-.551 1.554-1.438 1.554c-.21 0-.486-.036-.665-.097l.101-.737c.127.042.289.072.469.072c.384 0 .623-.174.623-.804v-2.543h.911zm3.294-.365c-.313.293-.773.426-1.313.426c-.12 0-.228-.007-.312-.019v1.445h-.906v-3.988a7.5 7.5 0 0 1 1.236-.083c.563 0 .965.107 1.234.323c.259.204.433.54.433.936s-.133.732-.372.96m4.331 1.667c-.28.096-.815.228-1.349.228c-.737 0-1.271-.186-1.643-.546c-.371-.348-.575-.875-.57-1.469c.007-1.344.983-2.111 2.309-2.111c.521 0 .924.103 1.121.198l-.191.731c-.222-.096-.497-.174-.941-.174c-.761 0-1.338.432-1.338 1.308c0 .833.523 1.325 1.271 1.325c.211 0 .378-.024.451-.061v-.846h-.624v-.713h1.504zM14 9h-1V4l5 5z"/><path fill="currentColor" d="M11.285 14.552c-.186 0-.312.018-.377.036v1.193c.077.018.174.023.307.023c.484 0 .784-.246.784-.659c0-.372-.257-.593-.714-.593"/></svg>',
    txt: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM9.998 14.768H8.895v3.274h-.917v-3.274H6.893V14h3.105zm2.725 3.274l-.365-.731c-.15-.282-.246-.492-.359-.726h-.013c-.083.233-.185.443-.312.726l-.335.731h-1.045l1.171-2.045L10.336 14h1.05l.354.738c.121.245.21.443.306.671h.013c.096-.258.174-.438.276-.671l.341-.738h1.043l-1.139 1.973l1.198 2.069zm4.384-3.274h-1.104v3.274h-.917v-3.274h-1.085V14h3.105zM14 9h-1V4l5 5z"/></svg>',
    zip: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M2 6.25V8h6.129a.75.75 0 0 0 .53-.22l2.591-2.59l-1.53-1.531A2.25 2.25 0 0 0 8.129 3H5.25A3.25 3.25 0 0 0 2 6.25m0 11.5V9.5h6.129a2.25 2.25 0 0 0 1.59-.659L13.062 5.5h.439v3.75c0 .414.336.75.75.75H15v3h-.25a.75.75 0 0 0 0 1.5H15V16h-.25a.75.75 0 0 0 0 1.5H15V21H5.25A3.25 3.25 0 0 1 2 17.75M16.5 21h2.25A3.25 3.25 0 0 0 22 17.75v-9a3.25 3.25 0 0 0-3.25-3.25H18v3.75a.75.75 0 0 1-.75.75h-.75v4.5h.25a.75.75 0 0 1 0 1.5h-.25zm0-15.5H15v3h1.5z"/></svg>',
    excel:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 32 32"><defs><linearGradient id="vscodeIconsFileTypeExcel0" x1="4.494" x2="13.832" y1="-2092.086" y2="-2075.914" gradientTransform="translate(0 2100)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#18884f"/><stop offset=".5" stop-color="#117e43"/><stop offset="1" stop-color="#0b6631"/></linearGradient></defs><path fill="#185c37" d="M19.581 15.35L8.512 13.4v14.409A1.19 1.19 0 0 0 9.705 29h19.1A1.19 1.19 0 0 0 30 27.809V22.5Z"/><path fill="#21a366" d="M19.581 3H9.705a1.19 1.19 0 0 0-1.193 1.191V9.5L19.581 16l5.861 1.95L30 16V9.5Z"/><path fill="#107c41" d="M8.512 9.5h11.069V16H8.512Z"/><path d="M16.434 8.2H8.512v16.25h7.922a1.2 1.2 0 0 0 1.194-1.191V9.391A1.2 1.2 0 0 0 16.434 8.2" opacity="0.1"/><path d="M15.783 8.85H8.512V25.1h7.271a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path d="M15.783 8.85H8.512V23.8h7.271a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path d="M15.132 8.85h-6.62V23.8h6.62a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path fill="url(#vscodeIconsFileTypeExcel0)" d="M3.194 8.85h11.938a1.193 1.193 0 0 1 1.194 1.191v11.918a1.193 1.193 0 0 1-1.194 1.191H3.194A1.19 1.19 0 0 1 2 21.959V10.041A1.19 1.19 0 0 1 3.194 8.85"/><path fill="#fff" d="m5.7 19.873l2.511-3.884l-2.3-3.862h1.847L9.013 14.6c.116.234.2.408.238.524h.017q.123-.281.26-.546l1.342-2.447h1.7l-2.359 3.84l2.419 3.905h-1.809l-1.45-2.711A2.4 2.4 0 0 1 9.2 16.8h-.024a1.7 1.7 0 0 1-.168.351l-1.493 2.722Z"/><path fill="#33c481" d="M28.806 3h-9.225v6.5H30V4.191A1.19 1.19 0 0 0 28.806 3"/><path fill="#107c41" d="M19.581 16H30v6.5H19.581Z"/></svg>',
    word: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 32 32"><defs><linearGradient id="vscodeIconsFileTypeWord0" x1="4.494" x2="13.832" y1="-1712.086" y2="-1695.914" gradientTransform="translate(0 1720)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="#2368c4"/><stop offset=".5" stop-color="#1a5dbe"/><stop offset="1" stop-color="#1146ac"/></linearGradient></defs><path fill="#41a5ee" d="M28.806 3H9.705a1.19 1.19 0 0 0-1.193 1.191V9.5l11.069 3.25L30 9.5V4.191A1.19 1.19 0 0 0 28.806 3"/><path fill="#2b7cd3" d="M30 9.5H8.512V16l11.069 1.95L30 16Z"/><path fill="#185abd" d="M8.512 16v6.5l10.418 1.3L30 22.5V16Z"/><path fill="#103f91" d="M9.705 29h19.1A1.19 1.19 0 0 0 30 27.809V22.5H8.512v5.309A1.19 1.19 0 0 0 9.705 29"/><path d="M16.434 8.2H8.512v16.25h7.922a1.2 1.2 0 0 0 1.194-1.191V9.391A1.2 1.2 0 0 0 16.434 8.2" opacity="0.1"/><path d="M15.783 8.85H8.512V25.1h7.271a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path d="M15.783 8.85H8.512V23.8h7.271a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path d="M15.132 8.85h-6.62V23.8h6.62a1.2 1.2 0 0 0 1.194-1.191V10.041a1.2 1.2 0 0 0-1.194-1.191" opacity="0.2"/><path fill="url(#vscodeIconsFileTypeWord0)" d="M3.194 8.85h11.938a1.193 1.193 0 0 1 1.194 1.191v11.918a1.193 1.193 0 0 1-1.194 1.191H3.194A1.19 1.19 0 0 1 2 21.959V10.041A1.19 1.19 0 0 1 3.194 8.85"/><path fill="#fff" d="M6.9 17.988q.035.276.046.481h.028q.015-.195.065-.47c.05-.275.062-.338.089-.465l1.255-5.407h1.624l1.3 5.326a8 8 0 0 1 .162 1h.022a8 8 0 0 1 .135-.975l1.039-5.358h1.477l-1.824 7.748h-1.727l-1.237-5.126q-.054-.222-.122-.578t-.084-.52h-.021q-.021.189-.084.561t-.1.552L7.78 19.871H6.024L4.19 12.127h1.5l1.131 5.418a5 5 0 0 1 .079.443"/></svg>',
    pin: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="m15.99 4.95l.53-.53zm3.082 3.086l-.531.53zM8.738 19.429l-.53.53zm-4.116-4.12l.53-.53zm12.945-.315l-.264-.702zm-1.917.72l.264.703zM8.332 8.383l-.704-.258zm.695-1.896l.704.258zm-3.182 4.188l.2.723zm1.457-.539l-.439-.609zm.374-.345l.57.487zm6.575 6.59l.491.568zm-.87 1.821l-.724-.199zm.536-1.454l-.61-.438zM2.718 12.755l-.75.005zm.212-.803l-.65-.374zm8.375 9.391l.001-.75zm.788-.208l-.371-.652zm-.396-19.099l.162.732zM1.47 21.47a.75.75 0 0 0 1.062 1.06zm5.715-3.598a.75.75 0 0 0-1.061-1.06zM15.459 5.48l3.082 3.086l1.061-1.06L16.52 4.42zM9.269 18.9l-4.117-4.12l-1.06 1.06l4.116 4.12zm8.034-4.607l-1.917.72l.528 1.405l1.917-.72zM9.036 8.64l.695-1.896l-1.409-.516l-.694 1.896zm-2.992 2.756c.712-.196 1.253-.334 1.696-.652l-.877-1.218c-.172.125-.397.198-1.217.424zm1.584-3.272c-.293.8-.385 1.018-.523 1.18l1.142.973c.353-.415.535-.944.79-1.637zm.112 2.62q.281-.203.507-.467l-1.142-.973a1.4 1.4 0 0 1-.242.222zm7.646 4.268c-.689.26-1.214.445-1.626.801l.982 1.135c.16-.14.377-.233 1.172-.531zM14.104 18.4c.225-.819.298-1.043.422-1.216l-1.219-.875c-.317.443-.454.983-.65 1.693zm-.344-2.586q-.256.22-.453.495l1.22.875q.093-.132.215-.236zm-8.608-1.036c-.646-.647-1.084-1.087-1.368-1.444c-.286-.359-.315-.514-.316-.583l-1.5.009c.004.582.293 1.07.642 1.508c.35.44.861.95 1.481 1.57zm.494-4.828c-.846.234-1.542.424-2.063.634c-.52.208-1.012.49-1.302.994l1.3.748c.034-.06.136-.18.56-.35s1.022-.337 1.903-.58zm-2.178 2.8a.84.84 0 0 1 .112-.424l-1.3-.748a2.34 2.34 0 0 0-.312 1.182zm4.74 7.21c.624.624 1.137 1.139 1.578 1.49c.441.352.932.642 1.518.643l.002-1.5c-.07 0-.225-.029-.585-.316c-.36-.286-.802-.727-1.452-1.378zm4.45-1.958c-.245.888-.412 1.49-.583 1.917c-.172.428-.293.53-.353.564l.743 1.303c.509-.29.792-.786 1.002-1.309c.21-.524.402-1.225.637-2.077zm-1.354 4.091c.407 0 .807-.105 1.161-.307l-.743-1.303a.84.84 0 0 1-.416.11zm7.237-13.527c1.064 1.064 1.8 1.803 2.25 2.413c.444.598.495.917.441 1.167l1.466.317c.19-.878-.16-1.647-.701-2.377c-.534-.72-1.366-1.551-2.395-2.58zm-.71 7.13c1.361-.511 2.463-.923 3.246-1.358c.795-.44 1.431-.996 1.621-1.875l-1.466-.317c-.054.25-.232.52-.883.88c-.663.369-1.638.737-3.046 1.266zM16.52 4.42c-1.036-1.037-1.872-1.876-2.595-2.414c-.734-.544-1.508-.897-2.39-.702l.324 1.464c.25-.055.569-.005 1.171.443c.613.455 1.358 1.197 2.429 2.27zM9.73 6.744c.522-1.423.886-2.41 1.251-3.08c.36-.66.628-.84.878-.896l-.323-1.464c-.882.194-1.435.84-1.872 1.642c-.431.792-.837 1.906-1.342 3.282zM2.53 22.53l4.654-4.658l-1.061-1.06l-4.654 4.658z"/></svg>',
    markdown:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 1024 1024"><path fill="currentColor" d="M854.6 288.7c6 6 9.4 14.1 9.4 22.6V928c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32V96c0-17.7 14.3-32 32-32h424.7c8.5 0 16.7 3.4 22.7 9.4zM790.2 326L602 137.8V326zM426.13 600.93l59.11 132.97a16 16 0 0 0 14.62 9.5h24.06a16 16 0 0 0 14.63-9.51l59.1-133.35V758a16 16 0 0 0 16.01 16H641a16 16 0 0 0 16-16V486a16 16 0 0 0-16-16h-34.75a16 16 0 0 0-14.67 9.62L512.1 662.2l-79.48-182.59a16 16 0 0 0-14.67-9.61H383a16 16 0 0 0-16 16v272a16 16 0 0 0 16 16h27.13a16 16 0 0 0 16-16z"/></svg>',
    gear: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M12 15.5A3.5 3.5 0 0 1 8.5 12A3.5 3.5 0 0 1 12 8.5a3.5 3.5 0 0 1 3.5 3.5a3.5 3.5 0 0 1-3.5 3.5m7.43-2.53c.04-.32.07-.64.07-.97s-.03-.66-.07-1l2.11-1.63c.19-.15.24-.42.12-.64l-2-3.46c-.12-.22-.39-.31-.61-.22l-2.49 1c-.52-.39-1.06-.73-1.69-.98l-.37-2.65A.506.506 0 0 0 14 2h-4c-.25 0-.46.18-.5.42l-.37 2.65c-.63.25-1.17.59-1.69.98l-2.49-1c-.22-.09-.49 0-.61.22l-2 3.46c-.13.22-.07.49.12.64L4.57 11c-.04.34-.07.67-.07 1s.03.65.07.97l-2.11 1.66c-.19.15-.25.42-.12.64l2 3.46c.12.22.39.3.61.22l2.49-1.01c.52.4 1.06.74 1.69.99l.37 2.65c.04.24.25.42.5.42h4c.25 0 .46-.18.5-.42l.37-2.65c.63-.26 1.17-.59 1.69-.99l2.49 1.01c.22.08.49 0 .61-.22l2-3.46c.12-.22.07-.49-.12-.64z"/></svg>',
    pdf: '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="none" stroke="#ed8796" stroke-linecap="round" stroke-linejoin="round" d="M2.8 14.34c1.81-1.25 3.02-3.16 3.91-5.5c.9-2.33 1.86-4.33 1.44-6.63c-.06-.36-.57-.73-.83-.7c-1.02.06-.95 1.21-.85 1.9c.24 1.71 1.56 3.7 2.84 5.56c1.27 1.87 2.32 2.16 3.78 2.26c.5.03 1.25-.14 1.37-.58c.77-2.8-9.02-.54-12.28 2.08c-.4.33-.86 1-.6 1.46c.2.36.87.4 1.23.15h0Z"/></svg>',
    dropDown:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="m8.71 11.71l2.59 2.59c.39.39 1.02.39 1.41 0l2.59-2.59c.63-.63.18-1.71-.71-1.71H9.41c-.89 0-1.33 1.08-.7 1.71"/></svg>',
    dropUp:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M8.2 14q-.225 0-.362-.15T7.7 13.5q0-.05.15-.35l3.625-3.625q.125-.125.25-.175T12 9.3t.275.05t.25.175l3.625 3.625q.075.075.113.163t.037.187q0 .2-.137.35T15.8 14z"/></svg>',
    hardDisk:
      '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 14 14"><path fill="currentColor" fill-rule="evenodd" d="M1 1.5A1.5 1.5 0 0 1 2.5 0h9A1.5 1.5 0 0 1 13 1.5v11a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 1 12.5zm5.991 6.692a.625.625 0 0 0-.653-.947a.57.57 0 0 0-.401.276l-1.75 2.75a.625.625 0 1 0 1.054.671zm.258 3.414c0-.345.28-.625.625-.625h2.5a.625.625 0 0 1 0 1.25h-2.5a.625.625 0 0 1-.625-.625M6.99 1.897A3.875 3.875 0 0 0 4 8.237l.882-1.387c.298-.468.76-.768 1.284-.843c.49-.071.946.064 1.293.285c.348.221.664.578.807 1.051a1.83 1.83 0 0 1-.22 1.52l-.471.74a3.876 3.876 0 0 0-.585-7.706m.082 5.815a.62.62 0 0 1-.08.48a.58.58 0 0 0 .08-.48" clip-rule="evenodd"/></svg>',
  };

  constructor(private rendererFactory: RendererFactory2) {}

  getIcon(
    name: string,
    size: string = "2em",
    color: string | undefined
  ): string {
    let iconSvg = this.icons[name];
    if (!iconSvg) {
      return this.icons["default"];
    }

    //const noPlaceholderColors = iconSvg.includes("currentColor");
    const noPlaceholderColors = true; // Re-enable if needed
    if (color) {
      const hexTokens = this.extractHexColorsFromColorString(color);
      if (hexTokens.length == 2 && noPlaceholderColors) {
        // SVG is a solid color and is eligible for a gradient BG
        const hex1: string = hexTokens[0];
        const hex2: string = hexTokens[1];
        iconSvg = replaceColorWithGradient(iconSvg, hex1, hex2).replace(
          /1em/g,
          size
        );
      } else {
        if (noPlaceholderColors) {
          if (color.startsWith("--")) {
            color = this.getCssVarHex(color);
          }

          iconSvg = iconSvg.replace(/currentColor/g, color);
        } else {
          // SVG contains multiple colors, so fill them in accordingly
          iconSvg = this.replacePathFillColors(iconSvg, hexTokens);
        }
      }
    }

    return iconSvg.replace(/1em/g, size);
  }

  /** Pass in a string such as "--color1 --color2" and you'll get something like
   * [#333,#222]
   */
  private extractHexColorsFromColorString(str: string): string[] {
    let hexes: string[] = [];
    const tokens: string[] = str.split(" ");
    tokens.forEach((x) => {
      let hex1 = "#000";
      if (x.startsWith("--")) {
        // Its a CSS variable
        hex1 = this.getCssVarHex(x);
      } else if (x.startsWith("#")) {
        hex1 = x;
      }
      hexes.push(hex1);
    });
    return hexes;
  }

  /**
   * // Example usage
   *
   * input = 'path fill="#ffb02e" width="10px", path fill="#ffb02a", path fill="#faa02e"';
   *
   * colors = ['#333', '#444'];
   *
   * Output: 'path fill="#333" width="10px", path fill="#444", path fill="#faa02e"'
   * @param input
   * @param colors
   * @returns
   */
  private replacePathFillColors(inputSvg: string, colors: string[]): string {
    // Match all instances of `path fill="#<color>"` in the string
    const regex = /path fill="#[a-fA-F0-9]{6}"/g;

    let colorIndex = 0;

    // Replace each match with the corresponding color from the colors array
    return inputSvg.replace(regex, (match) => {
      if (colorIndex < colors.length) {
        // Replace the color value in the matched string
        const newColor = colors[colorIndex];
        colorIndex++;
        return match.replace(/#[a-fA-F0-9]{6}/, newColor);
      }
      // If there are no more colors, return the match as is
      return match;
    });
  }

  private getCssVarHex(variable: string): string {
    const root = this.renderer.selectRootElement(":root", true);
    return getComputedStyle(root).getPropertyValue(variable).trim();
  }
}
