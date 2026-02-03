export function debounce(func: Function, delay: number = 500) {
  let timer: NodeJS.Timeout;
  return function (this: Function, ...args: any[]) {
    clearTimeout(timer);
    timer = setTimeout(() => func.apply(this, args), delay) as NodeJS.Timeout;
  };
}

export function throttle(func: Function, delay: number = 500) {
  let startTime = 0;
  return function (this: Function, ...args: any[]) {
    const current = Date.now();
    if (current - startTime >= delay) {
      startTime = current;
      func.apply(this, args);
    }
  };
}

export function throttleWithCache(func: Function, delay: number = 500) {
  let cache: any;
  let startTime = 0;
  return function (this: Function, ...args: any[]) {
    const current = Date.now();
    if (current - startTime >= delay) {
      startTime = current;
      cache = func.apply(this, args);
    }
    return cache;
  };
}
