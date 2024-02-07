import {selfUrl} from "@/utils";
import {MetadataRoute} from "next";

export default function sitemap(): MetadataRoute.Sitemap {
    const base = selfUrl();
    return [
        {
            url: `${base}/`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 1,
        },
        {
            url: `${base}/about`,
            lastModified: new Date(),
            changeFrequency: "monthly",
            priority: 0.5,
        },
    ];
}
