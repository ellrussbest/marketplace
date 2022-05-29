export function getProducts() {
  return window.contract.getProducts();
}

export async function buyProduct({ id }) {
  await window.contract.buyProduct({ id: id });
}