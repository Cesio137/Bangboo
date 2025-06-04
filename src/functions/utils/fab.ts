import { fab } from "#settings";

type ProductList = typeof fab;

const products: ProductList = Object.create({});

for (const [name, id] of Object.entries(fab)) {
    Object.assign(products, { [name]: id });
}

export { products }