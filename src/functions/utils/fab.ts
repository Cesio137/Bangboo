import { settings } from "#settings";

type ChannelList = typeof settings.fab;

const products: ChannelList = Object.create({});

for (const [name, id] of Object.entries(settings.fab)) {
    Object.assign(products, { [name]: id });
}

export { products }