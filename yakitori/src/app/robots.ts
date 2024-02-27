import {getSelfUrl} from "@/urls";
import {MetadataRoute} from "next";

export default function robots(): MetadataRoute.Robots {
    const self = getSelfUrl();
    return {
        rules: {
            userAgent: "*",
            allow: "/",
            disallow: ["/settings/"],
        },
        sitemap: `${self}/sitemap.xml`,
    };
}
