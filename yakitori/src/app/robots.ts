import {getSelfUrl} from "@/urls";
import {MetadataRoute} from "next";

export default function robots(): MetadataRoute.Robots {
    const self = getSelfUrl();
    return {
        rules: {
            userAgent: "*",
            allow: "/",
            disallow: ["/dashboard/", "/settings/"],
        },
        sitemap: `${self}/sitemap.xml`,
    };
}
