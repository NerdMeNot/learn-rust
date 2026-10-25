namespace Shop {
  const PRICE = 10;

  // Not exported: invisible outside the namespace.
  function discountPercent(qty: number): number {
    return qty >= 10 ? 20 : 0;
  }

  export function total(qty: number): number {
    const full = qty * PRICE;
    return full - (full * discountPercent(qty)) / 100;
  }
}

console.log("3 items:", Shop.total(3));
console.log("12 items:", Shop.total(12));
