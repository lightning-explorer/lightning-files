export function quadraticEase(
  startValue: number,
  endValue:number,
  durationMs:number,
  easing = "easeIn",
  onUpdate:(n:number)=>void,
  onComplete?:()=>void
) {
  const startTime = performance.now();
  const changeInValue = endValue - startValue;

  const easeFunctions:any = {
    easeIn: (t:number) => t * t,
    easeOut: (t:number) => t * (2 - t),
    easeInOut: (t:number) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t),
  };

  const ease = easeFunctions[easing] || easeFunctions.easeIn;

  function animate(currentTime:any) {
    const elapsedTime = currentTime - startTime;
    const progress = Math.min(elapsedTime / durationMs, 1);
    const easedProgress = ease(progress);

    const currentValue = startValue + changeInValue * easedProgress;
    if (onUpdate) onUpdate(currentValue);

    if (progress < 1) {
      requestAnimationFrame(animate);
    } else if (onComplete) {
      onComplete();
    }
  }

  requestAnimationFrame(animate);
}
