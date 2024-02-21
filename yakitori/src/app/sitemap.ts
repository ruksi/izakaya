import {getSelfUrl} from "@/urls";
import {MetadataRoute} from "next";

export default function sitemap(): MetadataRoute.Sitemap {
    const self = getSelfUrl();
    return [
        {
            url: `${self}/`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 1,
        },
        {
            url: `${self}/about`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 0.5,
        },
    ];
}
