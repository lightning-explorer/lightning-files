export function capitalizeFirstLetter(word: string) {
  if (!word) return ""; // Handle empty strings
  return word.charAt(0).toUpperCase() + word.slice(1);
}

export function rangeToFirstPeriod(text:string):{start:number,end:number}{
    const firstPeriod = text.indexOf('.');
    if(firstPeriod<=1){
        return {start:0, end:text.length}
    }else{
        return {start:0, end:firstPeriod}
    }
}