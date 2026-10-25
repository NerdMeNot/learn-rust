namespace Shop {
  function discountPercent(qty: number): number {
    return qty >= 10 ? 20 : 0;
  }

  export function total(qty: number): number {
    const full = qty * 10;
    return full - (full * discountPercent(qty)) / 100;
  }
}

console.log("total:", Shop.total(12));
console.log(`discount: ${Shop.discountPercent(12)}%`);
