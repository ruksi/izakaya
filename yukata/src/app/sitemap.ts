import {yukataUrl} from "@/utils";
import {MetadataRoute} from "next";

export default function sitemap(): MetadataRoute.Sitemap {
    const host = yukataUrl();
    return [
        {
            url: `${host}/`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 1,
        },
        {
            url: `${host}/about`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 0.5,
        },
    ];
}
