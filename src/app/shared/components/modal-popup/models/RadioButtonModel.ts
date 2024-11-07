export interface RadioButtonModel {
    text: string,
    onToggle: (checked: boolean) => void,
    isChecked:boolean
}