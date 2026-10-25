type UserId = number & { readonly __brand: "UserId" };
type OrderId = number & { readonly __brand: "OrderId" };

function loadUser(id: UserId): string {
  return `user #${id}`;
}

const order = 7 as OrderId;
console.log(loadUser(order));
